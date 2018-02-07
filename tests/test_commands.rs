extern crate pugbot;
extern crate serde;
extern crate serde_json;
extern crate serenity;

use pugbot::commands;
use pugbot::models::game::Game;
use pugbot::models::draft_pool::DraftPool;
use pugbot::traits;
use serde::de::Deserialize;
use serde_json::Value;
use serenity::model::channel::{ Embed, Message };
use serenity::model::user::User;
use std::fs::File;

macro_rules! p {
  ($s:ident, $filename:expr) => ({
    let f = File::open(concat!("./tests/resources/", $filename, ".json")).unwrap();
    let v = serde_json::from_reader::<File, Value>(f).unwrap();

    $s::deserialize(v).unwrap()
  })
}

struct ClosedPool {
  pub members: Vec<User>
}

impl traits::has_members::HasMembers for ClosedPool {
  fn members(&self) -> Vec<User> { self.members.clone() }

  fn add_member(&mut self, _user: User) -> Option<Embed> {
    None
  }

  fn remove_member(&mut self, _user: User) -> Option<Embed> {
    None
  }

  fn members_changed_embed(&mut self, _r: u8, _g: u8, _b: u8) -> Option<Embed> {
    None
  }
}

impl traits::pool_availability::PoolAvailability for ClosedPool {
  fn is_open(&self) -> bool { false }

  fn members_full_embed(&mut self, _r: u8, _g: u8, _b: u8) -> Option<Embed> {
    None
  }
}

#[test]
fn update_members() {
  let message = p!(Message, "message");

  // Test updating members with a closed draft pool.
  let draft_pool = ClosedPool { members: Vec::new() };
  let game_closed = &mut Game { teams: None, draft_pool: draft_pool };
  let users = commands::add::update_members(game_closed, &message, false);
  // There should be no members in the members vec, since `is_open` always yields
  // `false`.
  assert_eq!(users.len(), 0);

  // Test updating members with an open draft pool.
  let game_open = &mut Game { teams: None, draft_pool: DraftPool { members: Vec::new() } };
  let users = commands::add::update_members(game_open, &message, false);
  // There should be one member in the members vec, the author of the message (which is
  // defined in ./resources/message.json)
  assert_eq!(users.len(), 1);
}
