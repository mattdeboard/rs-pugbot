extern crate pugbot;
extern crate serde;
extern crate serde_json;
extern crate serenity;

use pugbot::commands;
use pugbot::models;
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
  let draft_pool = ClosedPool { members: Vec::new() };
  let game = &mut models::game::Game { teams: None, draft_pool: draft_pool };
  let users = commands::add::update_members(game, &message, false);
  // There should be no members in the members vec, since `is_open` always yields
  // `false`.
  assert_eq!(users.len(), 0);
}
