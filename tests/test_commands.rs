extern crate pugbot;
extern crate serde;
extern crate serde_json;
extern crate serenity;

use pugbot::commands;
use pugbot::models::game::{ Game, Phases };
use pugbot::models::draft_pool::DraftPool;
use serde::de::Deserialize;
use serde_json::Value;
use serenity::model::channel::{ Message };
use serenity::model::id::UserId;
use serenity::model::user::User;
use std::env::set_var;
use std::fs::File;

macro_rules! p {
  ($s:ident, $filename:expr) => ({
    let f = File::open(concat!("./tests/resources/", $filename, ".json")).unwrap();
    let v = serde_json::from_reader::<File, Value>(f).unwrap();

    $s::deserialize(v).unwrap()
  })
}

fn gen_test_user() -> User {
  User {
    id: UserId(210),
    avatar: Some("abc".to_string()),
    bot: true,
    discriminator: 1432,
    name: "test".to_string(),
  }
}

#[test]
fn update_members() {
  let message = p!(Message, "message");
  let key = "TEAM_SIZE";
  set_var(key, "1");
  let game = &mut Game::new(None, DraftPool::new(vec![gen_test_user()]));
  let users = commands::add::update_members(game, &message, false);
  // There should be one member in the members vec to start with: our test user.
  // `update_members` above should add an additional user, the author of the message (which is
  // defined in ./resources/message.json).
  assert_eq!(users.len(), 2);
  assert_eq!(game.phase, Some(Phases::CaptainSelection));
}
