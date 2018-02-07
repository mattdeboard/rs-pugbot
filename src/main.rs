#![feature(const_fn)]

#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate env_logger;
extern crate kankyo;
extern crate rand;
extern crate typemap;

mod commands;
mod models;
mod traits;

use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::http;
use std::collections::HashSet;
use std::env;

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

fn main() {
  // This will load the environment variables located at `./.env`, relative to
  // the CWD. See `./.env.example` for an example on how to structure this.
  kankyo::load().expect("Failed to load .env file");

  // Initialize the logger to use environment variables.
  //
  // In this case, a good default is setting the environment variable
  // `RUST_LOG` to debug`.
  env_logger::init().expect("Failed to initialize env_logger");
  let token = env::var("DISCORD_TOKEN")
    .expect("Expected a token in the environment");

  let mut client = Client::new(&token, Handler).expect("Err creating client");

  let owners = match http::get_current_application_info() {
    Ok(info) => {
      let mut set = HashSet::new();
      set.insert(info.owner.id);
      set
    },
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  };
  {
    let mut data = client.data.lock();
    let draft_pool = models::draft_pool::DraftPool { members: Vec::new() };
    let game = models::game::Game { teams: None, draft_pool: draft_pool };
    data.insert::<models::game::Game>(game);
  }

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c
                 .owners(owners)
                 .prefix("~"))
      .command("add", |c| c
               // .after(|ctx, _, _| {
               //   let mut data = ctx.data.lock();
               //   let game = data.get_mut::<models::game::Game>().unwrap();
               //   let draft_pool = &mut game.draft_pool;

               //   if draft_pool.members.len == team_size() * 2 {
               //     let teams = &mut game.teams;
               //     let members = &mut draft_pool.members;
               //   }
               // })
               .cmd(commands::add::add)
               .batch_known_as(vec!["a"]))
      .command("remove", |c| c
               .cmd(commands::remove::remove)
               .batch_known_as(vec!["r"])
      )
  );

  if let Err(why) = client.start() {
    error!("Client error: {:?}", why);
  }
}
