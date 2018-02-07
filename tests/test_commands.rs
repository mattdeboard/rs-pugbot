extern crate pugbot;
extern crate serde;
extern crate serde_json;
extern crate serenity;

use pugbot::commands;
use pugbot::models;
use serde::de::Deserialize;
use serde_json::Value;
use serenity::model::channel::Message;
use std::fs::File;

macro_rules! p {
  ($s:ident, $filename:expr) => ({
    let f = File::open(concat!("./tests/resources/", $filename, ".json")).unwrap();
    let v = serde_json::from_reader::<File, Value>(f).unwrap();

    $s::deserialize(v).unwrap()
  })
}

#[test]
fn update_members() {
  let message = p!(Message, "message");
  let draft_pool = models::draft_pool::DraftPool { members: Vec::new() };
  let game = &mut models::game::Game { teams: None, draft_pool: draft_pool };
  let users = commands::add::update_members(game, &message, false);
  // There should be one member in the members vec, the author of the message
  // (defined in resources/message.json)
  assert_eq!(users.len(), 1);
}
