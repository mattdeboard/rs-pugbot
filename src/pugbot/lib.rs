#![feature(const_fn)]

#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate env_logger;
extern crate kankyo;
extern crate rand;
extern crate typemap;

pub mod commands;
pub mod models;
pub mod traits;

use models::draft_pool::DraftPool;
use models::game::Game;
use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::http;
use std::collections::HashSet;
use std::env;
use std::marker::PhantomData;

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
    Ok(size) =>
      if let Ok(s) = size.parse::<u32>() {
        s
      } else {
        panic!("Invalid value for `TEAM_COUNT`")
      },
    Err(_) => panic!("No 'TEAM_SIZE' env var found")
  }
}

fn queue_size() -> u32 {
  kankyo::load().expect("Failed to load .env file");

  match env::var("TEAM_COUNT") {
    Ok(size) =>
      if let Ok(num_teams) = size.parse::<u32>() {
        num_teams * team_size()
      } else {
        panic!("Invalid value for `TEAM_COUNT`");
      },
    Err(_) => panic!("No 'TEAM_COUNT' env var found")
  }
}

pub fn client_setup() -> Client {
  env_logger::init().expect("Failed to initialize env_logger");
  let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment");
  let mut client = Client::new(&token, Handler).expect("Err creating client");

  {
    let mut data = client.data.lock();
    let draft_pool = DraftPool { members: Vec::new() };
    let game = Game::new(None, draft_pool);
    data.insert::<Game<DraftPool>>(game);
  }

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c
                 .owners(bot_owners())
                 .prefix("~"))
      .command("add", |c| c
               .cmd(commands::add::add::<DraftPool> { phantom: PhantomData })
               .batch_known_as(vec!["a"]))
      .command("remove", |c| c
               .cmd(commands::remove::remove::<DraftPool> { phantom: PhantomData })
               .batch_known_as(vec!["r"])
      )
  );
  client
}

fn bot_owners() -> HashSet<UserId> {
  match http::get_current_application_info() {
    Ok(info) => {
      let mut set = HashSet::new();
      set.insert(info.owner.id);
      set
    },
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  }
}

