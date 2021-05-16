use diesel::{
  result::Error, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl,
  RunQueryDsl,
};
use r2d2_diesel::ConnectionManager;

use crate::models::game_mode::GameMode;
use crate::schema::*;

#[derive(
  Clone, Debug, Identifiable, Insertable, Queryable, Associations, Deserialize,
)]
#[primary_key(game_config_id)]
#[table_name = "game_configs"]
#[belongs_to(GameMode)]
pub struct GameConfig {
  game_config_id: i32,
  game_mode_id: i32,
  team_count: i32,
  team_size: i32,
}

pub fn select_config_for_mode(
  conn: r2d2::PooledConnection<ConnectionManager<PgConnection>>,
  mode_id: i32,
) -> GameConfig {
  game_configs::table
    .inner_join(game_modes::table.on(game_modes::game_mode_id.eq(mode_id)))
    .select(game_configs::all_columns)
    .get_result::<GameConfig>(&*conn)
    .expect("Unable to find config for this mode.")
}

pub fn select_configs_for_game_title(
  conn: &r2d2::PooledConnection<ConnectionManager<PgConnection>>,
  game_title_id: i32,
) -> Result<Vec<GameConfig>, Error> {
  game_configs::table
    .inner_join(
      game_modes::table.on(game_modes::game_title_id.eq(game_title_id)),
    )
    .select(game_configs::all_columns)
    .get_results::<GameConfig>(&**conn)
}

#[cfg(test)]
#[allow(unused_must_use)]
pub mod tests {

  use diesel::{
    delete, insert_into, Connection, ExpressionMethods, PgConnection, QueryDsl,
    RunQueryDsl,
  };
  use diesel_migrations::embed_migrations;

  use crate::db::init_pool;
  use crate::models::{
    game_config::{select_configs_for_game_title, GameConfig},
    game_mode::GameMode,
    game_title::GameTitle,
  };
  use crate::schema::*;

  static DB_URL: &'static str =
    "postgres://pugbot:password@localhost/pugbot_test";

  fn game_mode_id_generator() -> std::iter::Cycle<std::ops::Range<i32>> {
    (1..std::i32::MAX).cycle()
  }

  fn setup() -> Result<(), &'static str> {
    if let Ok(conn) = PgConnection::establish(DB_URL) {
      embed_migrations!();
      embedded_migrations::run_with_output(&conn, &mut std::io::stdout());
      delete(game_configs::table)
        .execute(&conn)
        .expect("Could not clear game_configs");
      delete(game_titles::table)
        .execute(&conn)
        .expect("Could not clear game_titles");
      delete(game_modes::table)
        .execute(&conn)
        .expect("Could not clear game_modes");

      return Ok(());
    }
    Err("Migrations could not be run")
  }

  fn teardown() {
    if let Ok(conn) = PgConnection::establish(DB_URL) {
      delete(game_modes::table)
        .execute(&conn)
        .expect("Could not clear game_modes");
      delete(game_titles::table)
        .execute(&conn)
        .expect("Could not clear game_titles");
      delete(game_configs::table)
        .execute(&conn)
        .expect("Could not clear game_configs");
    }
  }

  #[test]
  fn test_select_configs_for_game_title() {
    setup();
    // Shorter db connection timeout in test
    let pool = init_pool(Some(DB_URL.to_string()), Some(1));
    let first_conn = pool.get().expect("Unable to get connection to DB");
    let mut game_mode_pk = game_mode_id_generator();

    let game_title = GameTitle {
      game_title_id: 1,
      game_name: "TestGame".to_string(),
    };
    insert_into(game_titles::table)
      .values(&game_title)
      .execute(&*first_conn);

    let game_title_id: i32 = game_titles::table
      .filter(game_titles::game_name.eq(&game_title.game_name))
      .select(game_titles::game_title_id)
      .get_result(&*first_conn)
      .expect("Oops");

    let game_mode = GameMode {
      game_mode_id: game_mode_pk.next().unwrap_or(1),
      game_title_id,
      mode_name: "Test Mode A".to_string(),
      team_size: 5,
    };

    let game_mode_id = insert_into(game_modes::table)
      .values(&game_mode)
      .returning(game_modes::game_mode_id)
      .get_result::<i32>(&*first_conn)
      .expect("Error inserting GameMode");

    insert_into(game_modes::table)
      .values(&game_mode)
      .execute(&*first_conn);

    let game_config = GameConfig {
      game_config_id: 1,
      game_mode_id,
      team_count: 2,
      team_size: game_mode.team_size,
    };

    insert_into(game_configs::table)
      .values(&game_config)
      .get_results::<GameConfig>(&*first_conn);

    if let Ok(configs) =
      select_configs_for_game_title(&first_conn, game_title_id)
    {
      assert_eq!(configs.len(), 1);
    }
    teardown();
  }
}
