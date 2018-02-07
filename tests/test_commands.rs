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
use serenity::utils::Colour;
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

  fn add_member(&mut self, user: User) -> Embed {
    self.members_changed_embed(165, 255, 241)
  }

  fn remove_member(&mut self, user: User) -> Embed {
    self.members_changed_embed(165, 255, 241)
  }

  fn members_changed_embed(&mut self, r: u8, g: u8, b: u8) -> Embed {
    let members = self.members();

    Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(members.into_iter().map(|m| m.clone().name).collect()),
      footer: None,
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some("Test Embed".to_string()),
      url: None,
      video: None
    }
  }
}

impl traits::pool_availability::PoolAvailability for ClosedPool {
  fn is_open(&self) -> bool { false }

  fn members_full_embed(&mut self, r: u8, g: u8, b: u8) -> Embed {
    let members = self.members.clone();

    Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(members.into_iter().map(|m| m.clone().name).collect()),
      footer: None,
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some("Test Embed".to_string()),
      url: None,
      video: None
    }
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
