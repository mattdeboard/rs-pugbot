use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::model::user::User;

use crate::models::game::Phases;
use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;
use crate::traits::pool_availability::PoolAvailability;
use crate::{consume_message, models::game::GameContainer};
use serenity::framework::standard::CommandResult;
use serenity::prelude::Context;

#[command]
#[aliases("a")]
#[description(r#"Adds yourself to the pool of draftable players, or "draft pool."

Once enough people to fill out all the teams have added themselves, captains will be automatically selected at random, and drafting will begin."#
)]
pub(crate) async fn add(ctx: &Context, msg: &Message) -> CommandResult {
  update_members(ctx, msg, true); // XXX: should this be the return value?
  Ok(())
}

pub async fn update_members(
  ctx: &Context,
  msg: &Message,
  send_embed: bool,
) -> Vec<User> {
  let mut data = ctx.data.write().await;
  let game = data.get_mut::<GameContainer>().unwrap();
  // The `send_embed` parameter exists only as a way to avoid trying to hit the
  // Discord API during testing.
  if game.phase != Some(Phases::PlayerRegistration) {
    if let Some(embed) = game.draft_pool.members_full_embed(165, 255, 241) {
      if send_embed {
        consume_message(ctx, msg, embed);
      }
    }
  } else {
    let author = msg.author.clone();
    if let Some(embed) = game.draft_pool.add_member(author) {
      if send_embed {
        consume_message(ctx, msg, embed);
      }
    }
  }
  game.next_phase();
  game.draft_pool.members()
}

#[cfg(test)]
mod tests {
  use serde;
  use serde_json;
  use serenity::{self};

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::models::draft_pool::DraftPool;
  use crate::models::game::{Game, Phases};
  use crate::{commands, struct_from_json};
  use serenity::model::channel::Message;
  use std::env;
  use std::fs::File;

  #[test]
  fn test_update_members() {
    let context = commands::mock_context::tests::mock_context();
    let message = struct_from_json!(Message, "message");
    let key = "TEAM_SIZE";
    env::set_var(key, "1");
    let game = &mut Game::new(
      vec![],
      DraftPool::new(vec![], 12),
      1,
      Vec::new(),
      // Draft pool max size: 12 (2 * 6)
      2,
      6,
    );
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    let members = tokio_test::block_on(commands::add::update_members(
      &context, &message, false,
    ));
    // There should be one member in the members vec to start with: our test
    // user. `update_members` above should add an additional user, the
    // author of the message (which is defined in
    // src/tests/resources/message.json).
    assert_eq!(members.len(), 2);
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
  }
}
