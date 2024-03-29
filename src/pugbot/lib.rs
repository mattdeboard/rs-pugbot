#![allow(unused_attributes)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use env_logger;
use kankyo;

pub mod command_groups;
pub mod commands;
pub mod db;
pub mod models;
pub mod schema;
pub mod traits;

use crate::models::draft_pool::DraftPool;
use crate::models::game::{Game, GameContainer};

// use crate::models::team::Team;
// use glicko2::{new_rating, GameResult, Glicko2Rating};
use serenity::framework::StandardFramework;
use serenity::http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::collections::HashSet;
use std::ops::Range;

#[macro_export]
macro_rules! struct_from_json {
  ($s:ident, $filename:expr) => {{
    let f =
      File::open(concat!("./tests/resources/", $filename, ".json")).unwrap();
    let v = serde_json::from_reader::<File, Value>(f).unwrap();

    $s::deserialize(v).unwrap()
  }};
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
  }

  async fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }
}

fn team_size() -> u32 {
  kankyo::load().expect("Failed to load .env file");

  match kankyo::key("TEAM_SIZE")
    .expect("Invalid value for 'TEAM_SIZE'")
    .parse::<u32>()
  {
    Ok(size) => size,
    Err(_) => panic!("No 'TEAM_SIZE' env var found"),
  }
}

pub fn team_id_range(team_count: u32) -> Range<usize> {
  Range {
    start: 1,
    end: team_count as usize + 1,
  }
}

fn team_count() -> u32 {
  kankyo::load().expect("Failed to load .env file");

  if let Ok(num_teams) = kankyo::key("TEAM_COUNT")
    .expect("Invalid value for `TEAM_COUNT`")
    .parse::<u32>()
  {
    num_teams
  } else {
    2
  }
}

fn queue_size() -> u32 {
  kankyo::load().expect("Failed to load .env file");
  team_count() * team_size()
}

pub async fn client_setup() {
  env_logger::init().expect("Failed to initialize env_logger");
  let token = std::env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment");
  let owners = bot_owners(&token).await;
  let framework = StandardFramework::new()
    .configure(|c| c.owners(owners))
    .help(&commands::HELP_CMD)
    .group(&command_groups::map_voting::MAPVOTING_GROUP)
    .group(&command_groups::player_drafting::PLAYERDRAFTING_GROUP)
    .group(&command_groups::player_registration::PLAYERREGISTRATION_GROUP);

  let mut client = Client::builder(&token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Err creating client");

  {
    let mut data = client.data.write().await;
    let draft_pool = DraftPool::new(Vec::new(), team_count() * team_size());
    let db_pool = db::init_pool(None);
    let conn = db_pool.get().unwrap();
    let map_choices = db::select_maps_for_mode_id(conn, 1);
    let game = Game::new(
      vec![],
      draft_pool,
      5,
      map_choices,
      team_count(),
      team_size(),
    );
    data.insert::<GameContainer>(game);
    data.insert::<db::Pool>(db_pool);
  }

  client.start().await.unwrap(); // FIXME: should the return be a Result?
}

async fn bot_owners(token: &str) -> HashSet<UserId> {
  let client = http::client::Http::new_with_token(token); // XXX: maybe retain this client higher in the call stack?
  match client.get_current_application_info().await {
    Ok(info) => {
      let mut set = HashSet::new();
      set.insert(info.owner.id);
      set
    }
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  }
}

// pub fn new_rating_from_outcome(
//   original_rating: Glicko2Rating,
//   opposing_team: Team,
//   outcome: Outcome,
// ) -> Glicko2Rating {
//   let results: Vec<GameResult> = opposing_team
//     .glicko2_ratings
//     .into_iter()
//     .map(|r| match outcome {
//       Outcome::Win => GameResult::win(r),
//       Outcome::Loss => GameResult::loss(r),
//       Outcome::Draw => GameResult::draw(r),
//     })
//     .collect();
//   new_rating(original_rating, &results, 0.3)
// }
