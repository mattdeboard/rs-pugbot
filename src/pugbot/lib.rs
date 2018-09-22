#![feature(const_fn, custom_attribute, plugin)]
#![allow(unused_attributes)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serenity;
#[macro_use]
extern crate diesel;

extern crate bigdecimal;
extern crate env_logger;
extern crate glicko2;
extern crate kankyo;
extern crate num;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate typemap;

pub mod commands;
pub mod db;
pub mod models;
pub mod schema;
pub mod traits;

use glicko2::{new_rating, GameResult, Glicko2Rating};
use models::draft_pool::DraftPool;
use models::game::{Game, Outcome};
use models::team::Team;
use serenity::builder::CreateEmbed;
use serenity::framework::StandardFramework;
use serenity::http;
use serenity::model::channel::{Embed, Message};
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::collections::HashSet;
use std::convert::From;
use std::env;
use std::ops::Range;

struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
  }

  fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }
}

fn team_size() -> u32 {
  kankyo::load().expect("Failed to load .env file");

  match env::var("TEAM_SIZE") {
    Ok(size) => {
      if let Ok(s) = size.parse::<u32>() {
        s
      } else {
        panic!("Invalid value for `TEAM_COUNT`")
      }
    }
    Err(_) => panic!("No 'TEAM_SIZE' env var found"),
  }
}

fn team_id_range() -> Range<usize> {
  let tc = team_count().unwrap();
  Range {
    start: 1,
    end: (tc as usize) + 1,
  }
}

fn team_count() -> Option<u32> {
  kankyo::load().expect("Failed to load .env file");

  match env::var("TEAM_COUNT") {
    Ok(size) => {
      if let Ok(num_teams) = size.parse::<u32>() {
        Some(num_teams)
      } else {
        None
      }
    }
    Err(_) => None,
  }
}

fn queue_size() -> u32 {
  kankyo::load().expect("Failed to load .env file");

  if let Some(tc) = team_count() {
    tc * team_size()
  } else {
    panic!("Invalid value for TEAM_COUNT");
  }
}

#[allow(unused_must_use)]
pub fn client_setup() -> Client {
  env_logger::init().expect("Failed to initialize env_logger");
  let token =
    env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  let mut client = Client::new(&token, Handler).expect("Err creating client");

  {
    let mut data = client.data.lock();
    let draft_pool = DraftPool::new(Vec::new());
    let db_pool = db::init_pool(None);
    let conn = db_pool.get().unwrap();
    let map_choices = db::select_maps_for_mode_id(conn, 1);
    let game = Game::new(None, draft_pool, 5, map_choices);
    data.insert::<Game>(game);
    data.insert::<db::Pool>(db_pool);
  }

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c.owners(bot_owners()).prefix("~"))
      .command("add", |c| {
        c.cmd(commands::add::add).batch_known_as(vec!["a"])
      }).command("remove", |c| {
        c.cmd(commands::remove::remove).batch_known_as(vec!["r"])
      }).command("pick", |c| {
        c.cmd(commands::pick::pick).batch_known_as(vec!["p"])
      }).command("vote", |c| {
        c.cmd(commands::mapvote::mapvote)
          .batch_known_as(vec!["v", "mv"])
      }),
  );
  client.start();
  client
}

pub fn consume_message(msg: &Message, embed: Embed) {
  msg
    .channel_id
    .send_message(|m| m.embed(|_| CreateEmbed::from(embed)))
    .unwrap();
}

fn bot_owners() -> HashSet<UserId> {
  match http::get_current_application_info() {
    Ok(info) => {
      let mut set = HashSet::new();
      set.insert(info.owner.id);
      set
    }
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  }
}

pub fn new_rating_from_outcome(
  original_rating: Glicko2Rating,
  opposing_team: Team,
  outcome: Outcome,
) -> Glicko2Rating {
  let results: Vec<GameResult> = opposing_team
    .glicko2_ratings
    .into_iter()
    .map(|r| match outcome {
      Outcome::Win => GameResult::win(r),
      Outcome::Loss => GameResult::loss(r),
      Outcome::Draw => GameResult::draw(r),
    }).collect();
  new_rating(original_rating, &results, 0.3)
}
