use crate::models::game::GameContainer;
use crate::models::game::Phases;
use crate::traits::phased::Phased;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::{framework::standard::macros::command, utils::Colour};
use serenity::{
  framework::standard::{Args, CommandResult},
  utils::Color,
};

#[command]
#[aliases("v", "mv")]
#[description("Records your vote for map selection")]
pub(crate) async fn mapvote(
  ctx: &Context,
  msg: &Message,
  mut args: Args,
) -> CommandResult {
  map_vote(ctx, msg, true, args.single::<usize>()? as i32).await?;
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
  let embed_color = Colour::from_rgb(255, 0, 0);

  if game.phase != Some(Phases::MapSelection) {
    let err = "We're not picking maps right now!";

    if send_embed {
      msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|create_embed| {
          create_embed.color(embed_color);
          create_embed.description(String::from(err));
          create_embed.title(String::from("ERROR"))
        })
      });
    }

    return Err(err);
  }

  if !game.draft_pool.members.contains(&msg.author.id) && send_embed {
    match msg.author.direct_message(&ctx.http, |m| m.content(
      "Sorry, but you're not allowed to map vote because you're not registered to play!"
    )).await {
      Ok(_) => {
        msg.reply(&ctx.http, "You're welcome");
        Ok(())
      },
      Err(why) => {
        println!("Error sending message: {:?}", why);
        let err = "Had some kind of problem sending you a message.";
        msg.reply(&ctx.http, err);
        Err(err)
      }
    }
  } else {
    if let Some(vote_count) = game.map_votes.clone().get(&map_index) {
      game.map_votes.insert(map_index, vote_count + 1);
      game.register_vote(msg.author.id);
      game.next_phase();
      let embed_color = Color::from_rgb(164, 255, 241);
      if game.phase == Some(Phases::ResultRecording) && send_embed {
        msg.channel_id.send_message(&ctx.http, |m| {
          m.embed(|create_embed| {
            create_embed.color(embed_color);
            create_embed.description(format!(
              "The winning map is {:?}!",
              game.active_map
            ));
            create_embed
              .title(format!("The winning map is {:?}!", game.active_map))
          })
        });
        // consume_message(ctx, msg, |_| game.map_winner_embed(&164, &255, &241));
      }
      Ok(())
    } else {
      let err = "Invalid map selection.";

      if send_embed {
        msg.channel_id.send_message(&ctx.http, |m| {
          m.embed(|create_embed| {
            create_embed.color(embed_color);
            create_embed.description(String::from(err));
            create_embed.title(String::from("ERROR"))
          })
        });
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
  use crate::models::draft_pool::DraftPool;
  use crate::models::game::{Game, Phases};
  use crate::models::map::Map as GameMap;
  use crate::traits::phased::Phased;
  use crate::{commands, struct_from_json};
  use serde;
  use serde_json;
  use serenity::model::user::User;
  use serenity::model::{channel::Message, id::UserId};
  use serenity::{self};
  use std::fs::File;

  macro_rules! bo {
    ($e:expr) => {
      tokio_test::block_on($e)
    };
  }

  #[test]
  fn test_register_vote() {
    let context = commands::mock_context::tests::mock_context();
    let authors: Vec<User> = struct_from_json!(Vec, "authors");
    let maps: Vec<GameMap> = struct_from_json!(Vec, "maps");
    // Choosing 2 teams of 5 here since there are 10 authors in authors.json
    let (team_count, team_size) = (2, (authors.len() / 2) as u32);
    let game = &mut Game::new(
      vec![],
      DraftPool::new(
        vec![UserId(1), UserId(2), UserId(3), UserId(4)],
        team_count * team_size,
      ),
      1,
      maps,
      team_count,
      team_size,
    );
    game.next_phase();
    game.select_captains();

    let player_pool = game.draft_pool.available_players.clone();
    let message = struct_from_json!(Message, "message");

    // Populate the draft pool.
    for (key, _) in player_pool.iter() {
      commands::pick::draft_player(&context, &message, false, *key);
    }

    // This is the key of the game map we're voting for in this test.
    let candidate_map_idx = 1;
    let mut counter = 0;
    // let client = bo!(Client::builder("abc123")).unwrap();
    // We register a map vote for each player here.
    for _ in 0..(team_count * team_size) {
      // Precondition. We should be in the right phase every time.
      assert_eq!(game.phase, Some(Phases::MapSelection));
      // Precondition. The count of votes should be what we expect.
      assert_eq!(game.map_votes.get(&candidate_map_idx), Some(&counter));
      bo!(commands::mapvote::map_vote(
        &context,
        &message,
        false,
        candidate_map_idx,
      ));
      // Postcondition. The count of votes for this particular map should be one
      // higher now.
      assert_eq!(game.map_votes.get(&candidate_map_idx), Some(&(counter + 1)));
      counter += 1;
    }

    let vote_counts: i32 = game
      .map_votes
      .values()
      .clone()
      .fold(0, |acc, val| acc + *val);
    // The total number of votes should now equal the total number of players.
    assert_eq!(vote_counts as u32, team_count * team_size);
    // The number of votes for our candidate should be all the votes. (No other
    // maps should have votes)
    assert_eq!(game.map_votes.get(&candidate_map_idx), Some(&vote_counts));
    // The game should advance to the next phase since all the votes have been
    // tallied.
    assert_eq!(game.phase, Some(Phases::ResultRecording));
  }
}
