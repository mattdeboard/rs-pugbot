use crate::models::game::GameContainer;
use crate::models::game::Phases;
use crate::traits::phased::Phased;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;

#[command]
#[aliases("v", "mv")]
#[description("Records your vote for map selection")]
#[allow(unused_must_use)]
pub(crate) async fn mapvote(
  ctx: &Context,
  msg: &Message,
  mut args: Args,
) -> CommandResult {
  match args.single::<i32>() {
    Ok(user_index) => {
      map_vote(ctx, msg, true, user_index).await;
    }
    Err(_) => {
      let data = ctx.data.read().await;
      let game = data.get::<GameContainer>().unwrap();
      match game.phase {
        Some(Phases::MapSelection) => {
          msg
            .channel_id
            .send_message(&ctx.http, |m| {
              m.embed(|create_embed| {
                create_embed.color(super::ERROR_EMBED_COLOR);
                create_embed.description(String::from(
              "You must specify a map number with this command. Example: ~mv 1",
            ));
                create_embed.title(String::from("ERROR"))
              })
            })
            .await;
        }
        _ => {
          let err = "We're not picking maps right now!";
          msg
            .channel_id
            .send_message(&ctx.http, |m| {
              m.embed(|create_embed| {
                create_embed.color(super::ERROR_EMBED_COLOR);
                create_embed.description(String::from(err));
                create_embed.title(String::from("ERROR"))
              })
            })
            .await;
        }
      }
    }
  };
  Ok(())
}

#[allow(unused_must_use)]
pub async fn map_vote(
  ctx: &Context,
  msg: &Message,
  send_embed: bool,
  map_index: i32,
) -> Result<(), &'static str> {
  let mut data = ctx.data.write().await;
  let game = data.get_mut::<GameContainer>().unwrap();
  if game.phase != Some(Phases::MapSelection) {
    let err = "We're not picking maps right now!";

    if send_embed {
      msg
        .channel_id
        .send_message(&ctx.http, |m| {
          m.embed(|create_embed| {
            create_embed.color(super::ERROR_EMBED_COLOR);
            create_embed.description(String::from(err));
            create_embed.title(String::from("ERROR"))
          })
        })
        .await;
    }

    return Err(err);
  }
  if !game.draft_pool.members.contains(&msg.author) && send_embed {
    match msg.author.direct_message(&ctx.http, |m| m.content(
      "Sorry, but you're not allowed to map vote because you're not registered to play!"
    )).await {
      Ok(_) => {
        msg.reply(&ctx.http, "You're welcome").await;
        Ok(())
      },
      Err(why) => {
        println!("Error sending message: {:?}", why);
        let err = "Had some kind of problem sending you a message.";
        msg.reply(&ctx.http, err).await;
        Err(err)
      }
    }
  } else {
    if let Some(vote_count) = game.map_votes.clone().get(&map_index) {
      game.map_votes.insert(map_index, vote_count + 1);
      game.register_vote(msg.author.id);
      game.next_phase();
      if game.phase == Some(Phases::ResultRecording) && send_embed {
        msg
          .channel_id
          .send_message(&ctx.http, |m| {
            m.embed(|create_embed| {
              create_embed.color(super::SUCCESS_EMBED_COLOR);
              create_embed.description(format!(
                "The winning map is {:?}!",
                game.active_map
              ));
              create_embed
                .title(format!("The winning map is {:?}!", game.active_map))
            })
          })
          .await;
      }
      Ok(())
    } else {
      let err = "Invalid map selection.";

      if send_embed {
        msg
          .channel_id
          .send_message(&ctx.http, |m| {
            m.embed(|create_embed| {
              create_embed.color(super::ERROR_EMBED_COLOR);
              create_embed.description(String::from(err));
              create_embed.title(String::from("ERROR"))
            })
          })
          .await;
      }

      Err(err)
    }
  }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::models::game::{Game, Phases};
  use crate::models::map::Map as GameMap;
  use crate::models::{draft_pool::DraftPool, game::GameContainer};
  use crate::traits::phased::Phased;
  use crate::{commands, struct_from_json};
  use serde;
  use serde_json;
  use serenity::model::user::User;
  use serenity::{self};
  use serenity::{client::Context, model::channel::Message};
  use std::fs::File;

  async fn test_context(authors: Vec<User>, maps: Vec<GameMap>) -> Context {
    let context = commands::mock_context::tests::mock_context();
    let user_count = authors.len() as u32;
    {
      let game = Game::new(
        vec![],
        DraftPool::new(authors, 2 * user_count / 2),
        1,
        maps,
        2,
        user_count / 2,
      );
      let mut data = context.data.write().await;
      data.insert::<GameContainer>(game);
    }
    context
  }

  #[tokio::test]
  async fn test_register_vote() {
    let authors: Vec<User> = struct_from_json!(Vec, "authors");
    let maps: Vec<GameMap> = struct_from_json!(Vec, "maps");
    let context = test_context(authors, maps).await;
    let message = struct_from_json!(Message, "message");
    let candidate_map_idx = 1;

    // First, we retrieve the pool of available players. We need the
    // write lock on `context.data` so this has to happen inside a
    // block. This is because the commands also need the exclusive lock
    // on `context.data`. So those two have to be isolated from each
    // other.
    let pool = {
      let mut data = context.data.write().await;
      if let Some(game) = data.get_mut::<GameContainer>() {
        // Now we set up our test conditions:
        // 1. Advance the game state "phase machine" via `next_phase`
        // 2. Randomly select two captains from the draft pool via
        //    `select_captains`.
        game.next_phase();
        game.select_captains();

        Some(game.draft_pool.available_players.clone())
      } else {
        None
      }
    };

    // And now, we populate our teams with the players from the draft
    // pool. Here, `draft_player` requires an exclusive lock on
    // `context.data` so it's inside a block.
    {
      if let Some(player_pool) = pool {
        for (key, _) in player_pool.iter() {
          commands::pick::draft_player(&context, &message, false, *key).await;
        }
      }
    }

    // Confirm we're on the phase we ought to be: `MapSelection`
    {
      let data = context.data.read().await;
      if let Some(game) = data.get::<GameContainer>() {
        assert_eq!(game.phase, Some(Phases::MapSelection));
      }
    }

    let team_metadata = {
      let data = context.data.read().await;
      if let Some(game) = data.get::<GameContainer>() {
        Some((game.team_size, game.team_count))
      } else {
        None
      }
    };

    // Register a map vote for every drafted player
    {
      if let Some((team_size, team_count)) = team_metadata {
        for _ in 0..(team_count * team_size) {
          commands::mapvote::map_vote(
            &context,
            &message,
            false,
            candidate_map_idx,
          )
          .await;
        }
      }
    }

    {
      let data = context.data.read().await;
      if let Some(game) = data.get::<GameContainer>() {
        let vote_counts: i32 = game
          .map_votes
          .values()
          .clone()
          .fold(0, |acc, val| acc + *val);
        // The total number of votes should now equal the total number of players.
        assert_eq!(vote_counts as u32, game.team_count * game.team_size);
        // The number of votes for our candidate should be all the votes. (No other
        // maps should have votes)
        assert_eq!(game.map_votes.get(&candidate_map_idx), Some(&vote_counts));
        // The game should advance to the next phase since all the votes have been
        // tallied.
        assert_eq!(game.phase, Some(Phases::ResultRecording));
      }
    }
  }
}
