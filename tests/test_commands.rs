extern crate bigdecimal;
extern crate diesel;
extern crate kankyo;
extern crate pugbot;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;
extern crate serenity;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::RunQueryDsl;
use pugbot::commands;
use pugbot::db::init_pool;
use pugbot::models::game::{ Game, Phases };
use pugbot::models::draft_pool::DraftPool;
use pugbot::traits::has_members::HasMembers;
use pugbot::traits::phased::Phased;
use r2d2_diesel::ConnectionManager;
use serde::de::Deserialize;
use serde_json::Value;
use serenity::model::channel::{ Message };
use serenity::model::id::UserId;
use serenity::model::user::User;
use std::env;
use std::fs::File;

use pugbot::db::*;

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
  env::set_var(key, "1");
  let game = &mut Game::new(None, DraftPool::new(vec![gen_test_user()]));
  assert_eq!(game.phase, Some(Phases::PlayerRegistration));
  let members = commands::add::update_members(game, &message, false);
  // There should be one member in the members vec to start with: our test user.
  // `update_members` above should add an additional user, the author of the message (which is
  // defined in ./resources/message.json).
  assert_eq!(members.len(), 2);
  assert_eq!(game.phase, Some(Phases::CaptainSelection));
}

#[test]
fn select_captains() {
  let message = p!(Message, "message");
  let game = &mut Game::new(None, DraftPool::new(vec![gen_test_user()]));
  game.draft_pool.add_member(message.author);
  assert_eq!(game.phase, Some(Phases::PlayerRegistration));
  assert_eq!(game.select_captains(), Err("We aren't picking captains, yet!"));
  game.next_phase();
  // Switching to Captain Selection should build the available_players HashMap.
  assert_eq!(game.draft_pool.available_players.len(), 2);
  assert_eq!(game.phase, Some(Phases::CaptainSelection));
  assert_eq!(game.select_captains(), Ok(()));
  // Selecting captains successfully should consume all the entries in available_players
  assert_eq!(game.draft_pool.available_players.len(), 0);
  // There should now be two teams built.
  assert_eq!(game.teams.clone().unwrap().len(), 2);
}

pub fn connection() -> r2d2::PooledConnection<ConnectionManager<PgConnection>> {
  let pool = init_pool(Some("postgres://pugbot:pugbot@localhost:5432/test_pugbot".to_string()));
  let conn = pool.get().unwrap();
  conn.begin_test_transaction().unwrap();
  sql_query("DROP TABLE IF EXISTS users CASCADE").execute(&*conn).unwrap();
  sql_query("create table users (\
             user_id serial primary key,\
             bot bool not null default false,\
             discriminator varchar not null,\
             name varchar not null\
             )")
  .execute(&*conn)
  .unwrap();
  conn
}

#[test]
#[allow(unused_must_use)]
fn write_to_db() {
  assert_eq!(create_user_and_ratings(connection(), gen_test_user()), Ok(()));
}
