use diesel::{ Connection, RunQueryDsl, PgConnection };
use diesel::insert_into;
use r2d2;
use r2d2_diesel::ConnectionManager;
use serenity::model::user::User;
use std::env;
use std::ops::Deref;
use typemap::Key;

use schema::users::dsl::*;
use schema::user_ratings::dsl::*;
use tables::insert::{ Users as IUsers, UserRatings as IUserRatings };
use tables::query::{ Users as QUsers, UserRatings as QUserRatings };

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
pub fn init_pool(db_url: Option<String>) -> r2d2::Pool<ConnectionManager<PgConnection>> {
  let database_url = match db_url {
    Some(url) => url,
    _ => env::var("DATABASE_URL").expect("DATABASE_URL must be defined")
  };
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

pub fn create_user_and_ratings(
  conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>,
  user: User
) -> Result<(), String> {

  match insert_into(users).values(&IUsers::from(user)).get_result::<QUsers>(&*conn) {
    Ok(user_record) => if let Ok(_ratings_record) = insert_into(user_ratings)
      .values(&IUserRatings::from(user_record))
      .get_result::<QUserRatings>(&*conn) {
        Ok(())
      } else {
        Err("No user_ratings reckid created".to_string())
      },
    Err(e) => Err(format!("{:?}", e))
  }

  // if let Ok(user_record) = insert_into(users).values(&IUsers::from(user)).get_result::<QUsers>(&*conn) {
  //   if let Ok(_ratings_record) = insert_into(user_ratings)
  //     .values(&IUserRatings::from(user_record))
  //     .get_result::<QUserRatings>(&*conn)
  //   {
  //     Ok(())
  //   } else {
  //     Err("Could not create user_ratings record".to_string())
  //   }
  // } else {
  //   Err("Could not create users record".to_string())
  // }
}
