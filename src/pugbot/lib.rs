#![allow(unused_attributes)]
#[macro_use]
extern crate log;
extern crate serenity;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use env_logger;
use kankyo;

pub mod commands;
pub mod db;
pub mod models;
pub mod schema;
pub mod traits;

use crate::commands::{
  add::ADD_COMMAND, mapvote::MAPVOTE_COMMAND, pick::PICK_COMMAND,
  remove::REMOVE_COMMAND,
};
use crate::models::draft_pool::DraftPool;
use crate::models::game::Game;
// use crate::models::team::Team;
// use glicko2::{new_rating, GameResult, Glicko2Rating};
use serenity::builder::CreateEmbed;
use serenity::framework::standard::{
  help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
  StandardFramework,
};
use serenity::http::raw::Http;
use serenity::model::channel::{Embed, Message};
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::collections::HashSet;
use std::convert::From;
use std::env;

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

// group!({
//   name: "general",
//   options: {},
//   commands: [ADD_COMMAND, MAPVOTE_COMMAND, PICK_COMMAND, REMOVE_COMMAND],
// });

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

#[help]
fn pugbot_help(
  ctx: &mut Context,
  msg: &Message,
  args: Args,
  help_options: &'static HelpOptions,
  groups: &[&'static CommandGroup],
  owners: HashSet<UserId>,
) -> CommandResult {
  help_commands::with_embeds(ctx, msg, args, help_options, groups, owners)
}
#[allow(unused_must_use)]
pub fn client_setup() {
  env_logger::init().expect("Failed to initialize env_logger");
  let token =
    env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  let mut client = Client::new(&token, Handler).expect("Err creating client");

  {
    let mut data = client.data.write();
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
    data.insert::<Game>(game);
    data.insert::<db::Pool>(db_pool);
  }

  let owners = match client.cache_and_http.http.get_current_application_info() {
    Ok(info) => {
      let mut set = HashSet::new();
      set.insert(info.owner.id);
      set
    }
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  };

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c.owners(owners).prefix("~"))
      .help(&PUGBOT_HELP)
      .group("Map Voting", |g| {
        g.command("vote", |c| {
          c.desc("Records your vote for map selection")
            .cmd(MAPVOTE_COMMAND)
            .batch_known_as(vec!["v", "mv"])
        })
      })
      .group("Player Drafting", |g| {
        g.desc("Commands here are available to Captains only")
          .command("pick", |c| {
            c.desc("(Captains Only) `pick #` adds player `#` to your team.

Once enough players to fill out all the teams have added themselves, captains will be automatically selected at random. One captain will be selected per team.

The bot will then display a numbered list of players, like so:

```
  Index     Player Name
----------|-------------
    1     | Alice
    2     | Bob
    3     | Charlie
```

Captains will be able to use the `~pick <index>` command.")
              .cmd(PICK_COMMAND)
              .batch_known_as(vec!["p"])
          })
      })
      .group("Player Registration", |g| {
        g.command("add", |c| {
          c.desc(
            "Adds yourself to the pool of draftable players, or \"draft pool.\"

Once enough people to fill out all the teams have added themselves, captains will be automatically selected at random, and drafting will begin.",
          )
          .cmd(ADD_COMMAND)
          .batch_known_as(vec!["a"])
        })
        .command("remove", |c| {
          c.desc("Removes yourself from the draft pool.")
            .cmd(REMOVE_COMMAND)
            .batch_known_as(vec!["r"])
        })
      }),
  );
  client.start();
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
