use serenity::model::channel::Message;

use crate::models::game::{Game, Phases};
use crate::queue_size;
use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;
use serenity::framework::standard::{
  macros::command, CommandError, CommandResult,
};
use serenity::prelude::Context;
use serenity::utils::Colour;

#[command]
pub fn add(ctx: &mut Context, msg: &Message) -> CommandResult {
  let mut data = ctx.data.write();
  let game = data.get_mut::<Game>().unwrap();
  update_members(ctx, game, msg, true)
}

pub fn update_members(
  ctx: &mut Context,
  game: &mut Game,
  msg: &Message,
  send_embed: bool,
) -> Result<(), CommandError> {
  if !send_embed {
    return Ok(());
  }
  let members = game.draft_pool.members();

  // The `send_embed` parameter exists only as a way to avoid trying to hit the
  // Discord API during testing.
  if game.phase != Some(Phases::PlayerRegistration) {
    msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
        e.colour(Colour::from_rgb(165, 255, 241));
        e.description(
          members
            .into_iter()
            .map(|m| m.clone().name)
            .collect::<String>(),
        );
        e.title("Members in queue:");
        e.footer(|f| {
          f.text("The queue is full! Now picking captains!");
          f
        });
        e
      });

      m
    });
  } else {
    let author = msg.author.clone();
    game.draft_pool.add_member(author);
    msg.channel_id.send_message(&ctx.http, |m| {
      m.embed(|e| {
        e.colour(Colour::from_rgb(165, 255, 241));
        e.description(
          members
            .into_iter()
            .map(|m| m.clone().name)
            .collect::<String>(),
        );
        e.footer(|f| {
          if (members.len() as u32) == queue_size() {
            f.text("The queue is full! Now picking captains!");
          } else {
            f.text(format!(
              "{} of {} users in queue",
              members.len(),
              queue_size()
            ));
          }
          f
        });

        e.title("Members in queue:");
        e
      });
      m
    });
  }
  game.next_phase();
  Ok(())
}

#[cfg(test)]
mod tests {
  use serde;
  use serde_json;
  use serenity;

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::models::draft_pool::DraftPool;
  use crate::models::game::{Game, Phases};
  use crate::{commands, struct_from_json};
  use serenity::model::channel::Message;
  use serenity::model::id::UserId;
  use serenity::model::user::User;
  use std::env;
  use std::fs::File;

  fn gen_test_user() -> User {
    User {
      id: UserId(210),
      avatar: Some("abc".to_string()),
      bot: false,
      discriminator: 1432,
      name: "TestUser".to_string(),
    }
  }

  #[test]
  fn test_update_members() {
    let message = struct_from_json!(Message, "message");
    let key = "TEAM_SIZE";
    env::set_var(key, "1");
    let game = &mut Game::new(
      vec![],
      DraftPool::new(vec![gen_test_user()], 12),
      1,
      Vec::new(),
      // Draft pool max size: 12 (2 * 6)
      2,
      6,
    );
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    let members = commands::add::update_members(game, &message, false);
    // There should be one member in the members vec to start with: our test
    // user. `update_members` above should add an additional user, the
    // author of the message (which is defined in
    // src/tests/resources/message.json).
    assert_eq!(members.len(), 2);
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
  }
}
