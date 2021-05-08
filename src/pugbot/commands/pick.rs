use crate::models::game::GameContainer;
use crate::models::game::Phases;

use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::{framework::standard::macros::command, utils::Colour};

// FIXME: look at the `#[allow_roles()]` attr to restrict this to captains.
#[command]
#[aliases("p")]
#[description(r#"(Captains Only) `pick #` adds player `#` to your team.

Once enough players to fill out all the teams have added themselves, captains will be automatically selected at random. One captain will be selected per team.

The bot will then display a numbered list of players, like so:

```
  Index     Player Name
----------|-------------
    1     | Alice
    2     | Bob
    3     | Charlie
```

Captains will be able to use the `~pick <index>` command."#
)]
#[allow(unused_must_use)]
pub(crate) async fn pick(
  ctx: &Context,
  msg: &Message,
  mut args: Args,
) -> CommandResult {
  let user_index = args.single::<usize>().unwrap();

  draft_player(ctx, msg, true, user_index).await;
  Ok(())
}

#[allow(unused_must_use)]
pub async fn draft_player(
  ctx: &Context,
  msg: &Message,
  send_embed: bool,
  user_index: usize,
) -> Result<(), &'static str> {
  let mut data = ctx.data.write().await;
  let game = data.get_mut::<GameContainer>().unwrap();
  let embed_color = Colour::from_rgb(165, 255, 241);
  if game.phase != Some(Phases::PlayerDrafting) && send_embed {
    let err = "We're not drafting right now!";
    msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|create_embed| {
        create_embed.color(embed_color);
        create_embed.description(String::from(err));
        create_embed.title(String::from("ERROR"))
      })
    });
    return Err(err);
  }

  if let Some(user) = game.draft_pool.pop_available_player(&user_index) {
    let next_id = game.turn_taker.next().unwrap();
    game.teams[next_id as usize].add_member(user);
  } else {
    let err =
      "The user selected for drafting has been drafted or is otherwise invalid";
    if send_embed {
      // consume_message(ctx, msg, |_| &mut error_embed(err));
    }
    return Err(err);
  }

  game.next_phase();

  if game.phase == Some(Phases::MapSelection) && send_embed {
    let maps: Vec<String> = game.map_choices.iter().enumerate().fold(
      vec![String::from(
        "Typing `~mv <#>` will register your map vote (You must be on a team to vote)",
      )],
      |mut acc, (index, map)| {
        acc.push(format!("{} -> {}", index + 1, map.map_name));
        acc
      },
    );
    let embed_colour = Colour::from_rgb(165, 255, 241);
    msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
        e.color(embed_colour);
        e.description(game.roster().join("\n--\n"));
        e.title(String::from("Drafting has been completed!"))
      })
    });
    msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
        e.color(embed_colour);
        e.description(maps.join("\n"));
        e.title("Time to pick a map!")
      })
    });
  }
  Ok(())
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
  use serde;
  use serde_json;
  use serenity::{self, client::Context};

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::models::game::{Game, Phases};
  use crate::models::{draft_pool::DraftPool, game::GameContainer};
  use crate::traits::phased::Phased;
  use crate::{commands, struct_from_json};
  use serenity::model::channel::Message;
  use serenity::model::user::User;
  use std::fs::File;

  fn test_context(authors: Vec<User>) -> Box<Context> {
    let context = commands::mock_context::tests::mock_context();
    let user_count = authors.len() as u32;
    {
      let game = Game::new(
        vec![],
        DraftPool::new(authors, 12),
        1,
        Vec::new(),
        // Draft pool max size: 12 (2 * 6)
        2,
        user_count / 2,
      );
      let mut data = tokio_test::block_on(context.data.write());
      data.insert::<GameContainer>(game);
    }
    Box::new(context)
  }

  #[test]
  fn test_pick_player() {
    let authors: Vec<User> = struct_from_json!(Vec, "authors");
    let team_count = 2;
    // Choosing 2 teams of 5 here since there are 10 authors in authors.json
    let context = test_context(authors);
    let mut data = tokio_test::block_on(context.data.write());
    let the_game = data.get_mut::<GameContainer>();

    if let Some(game) = the_game {
      game.next_phase();
      // Check that captain-selection has finished before proceeding as
      // this is a precondition of the test.
      assert_eq!(game.select_captains(), Ok(()));

      // Make a random selection from available players
      let pool = game.draft_pool.available_players.clone();

      if let Some(key) = pool.keys().next() {
        if let Some(_user) = game.draft_pool.available_players.get(key) {
          let message = struct_from_json!(Message, "message");
          // Drafting a single player works as expected?
          async {
            assert_eq!(
              commands::pick::draft_player(&context, &message, false, *key)
                .await,
              Ok(())
            );
            // This stuff has to stay inside the async block or it gets
            // weird.
            let member_count = &game
              .teams
              .iter()
              .fold(0, |acc, team| acc + team.members.len());
            // The order in which teams choose seems to be
            // non-deterministic, so instead of checking that team 1 has
            // x members and team 2 has x+1 members, just test their
            // member counts equal 3.
            assert_eq!(*member_count as u32, 3);
          };
        }
        // There should be as many teams as specified.
        assert_eq!(game.teams.len() as u32, team_count);
      }
    }
  }

  #[test]
  fn test_full_teams() {
    let authors: Vec<User> = struct_from_json!(Vec, "authors");
    let message = struct_from_json!(Message, "message");
    let (team_count, team_size) = (2, (authors.len() / 2) as u32);
    // Choosing 2 teams of 5 here since there are 10 authors in authors.json
    let context = test_context(authors);
    let mut data = tokio_test::block_on(context.data.write());
    let the_game = data.get_mut::<GameContainer>();

    if let Some(game) = the_game {
      game.next_phase();
      game.select_captains();

      let player_pool = game.draft_pool.available_players.clone();
      for (key, _) in player_pool.iter() {
        async {
          commands::pick::draft_player(&context, &message, false, *key).await;
          // available_players should be empty. Each drafted player is popped out of
          // the available_players pool.
          assert_eq!(game.draft_pool.available_players.len(), 0);
          // Since all players were drafted and teams are now full, the game should
          // proceed to the next phase.
          assert_eq!(game.phase, Some(Phases::MapSelection));
          let member_count = &game
            .teams
            .iter()
            .fold(0, |acc, team| acc + team.members.len());
          // Final post-condition: The sum of the counts of each team's roster should
          // equal the max size of the draft pool.
          assert_eq!(*member_count as u32, team_count * team_size);
        };
      }
    }
  }
}
