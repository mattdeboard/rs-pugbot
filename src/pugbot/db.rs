use diesel::insert_into;
use diesel::result::Error;
use diesel::{
  ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl,
};
use r2d2;
use r2d2_diesel::ConnectionManager;
use serenity::model::user::User;
use std::env;
use std::ops::Deref;
use typemap::Key;

use crate::models::map::Map as GameMap;
use crate::models::user::DiscordUser;
use crate::models::user_rating::UserRating;
use crate::schema::user_ratings;
use crate::schema::users::dsl::*;
use crate::schema::*;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
  type Target = PgConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub struct Pool;

impl Key for Pool {
  type Value = r2d2::Pool<ConnectionManager<PgConnection>>;
}

/// Initializes a database pool.
pub fn init_pool(
  db_url: Option<String>,
) -> r2d2::Pool<ConnectionManager<PgConnection>> {
  let database_url = match db_url {
    Some(url) => url,
    _ => env::var("DATABASE_URL").expect("DATABASE_URL must be defined"),
  };
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}

pub fn create_user_and_ratings(
  conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>,
  mode_id: i32,
  user: User,
) -> Result<(), String> {
  match insert_into(users)
    .values(&user)
    .get_result::<DiscordUser>(&*conn)
  {
    Ok(user_record) => match create_rating(conn, mode_id, user_record) {
      Ok(_) => Ok(()),
      e => Err(format!("{:?}", e)),
    },
    e => Err(format!("{:?}", e)),
  }
}

pub fn create_rating(
  conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>,
  mode_id: i32,
  user_record: DiscordUser,
) -> Result<usize, Error> {
  let mut ratings = UserRating::from(user_record);
  ratings.game_mode_id = mode_id;
  insert_into(user_ratings::table)
    .values(&ratings)
    .execute(&*conn)
}

pub fn select_maps_for_mode_id(
  conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>,
  mode_id: i32,
) -> Vec<GameMap> {
  allow_tables_to_appear_in_same_query!(game_titles, game_modes, maps);
  no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");
  maps::table
    .inner_join(game_modes::table.on(game_modes::game_mode_id.eq(mode_id)))
    .filter(maps::game_title_id.eq(game_modes::game_title_id))
    .order(RANDOM)
    .limit(5)
    .select(maps::all_columns)
    .get_results::<GameMap>(&*conn)
    .expect("Unable to fetch game maps.")
}
