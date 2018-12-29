use serenity::model::channel::Message;
use serenity::model::user::User;

use crate::consume_message;
use crate::models::game::{Game, Phases};
use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;
use crate::traits::pool_availability::PoolAvailability;

command!(add(ctx, msg) {
  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();
  update_members(game, msg, true);
});

pub fn update_members(
  game: &mut Game,
  msg: &Message,
  send_embed: bool,
) -> Vec<User> {
  // The `send_embed` parameter exists only as a way to avoid trying to hit the Discord
  // API during testing.
  if game.phase != Some(Phases::PlayerRegistration) {
    if let Some(embed) = game.draft_pool.members_full_embed(165, 255, 241) {
      if send_embed {
        consume_message(msg, embed);
      }
    }
  } else {
    let author = msg.author.clone();
    if let Some(embed) = game.draft_pool.add_member(author) {
      if send_embed {
        consume_message(msg, embed);
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
  use serenity;

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::commands;
  use crate::models::draft_pool::DraftPool;
  use crate::models::game::{Game, Phases};
  use serenity::model::channel::Message;
  use serenity::model::id::UserId;
  use serenity::model::user::User;
  use std::env;
  use std::fs::File;

  macro_rules! p {
    ($s:ident, $filename:expr) => {{
      let f =
        File::open(concat!("./tests/resources/", $filename, ".json")).unwrap();
      let v = serde_json::from_reader::<File, Value>(f).unwrap();

      $s::deserialize(v).unwrap()
    }};
  }

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
    let message = p!(Message, "message");
    let key = "TEAM_SIZE";
    env::set_var(key, "1");
    let game = &mut Game::new(
      None,
      DraftPool::new(vec![gen_test_user()]),
      1,
      Vec::new(),
      // Draft pool max size: 12 (2 * 6)
      2,
      6,
    );
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    let members = commands::add::update_members(game, &message, false);
    // There should be one member in the members vec to start with: our test user.
    // `update_members` above should add an additional user, the author of the message (which is
    // defined in src/tests/resources/message.json).
    assert_eq!(members.len(), 2);
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
  }
}
