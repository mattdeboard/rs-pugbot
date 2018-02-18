pub mod query {
  use bigdecimal::BigDecimal;

  #[primary_key(game_title_id)]
  #[table_name="game_titles"]
  #[derive(Debug, Queryable, Associations)]
  pub struct GameTitles {
    pub game_title_id: i32,
    pub game_name: String
  }

  #[primary_key(game_mode_id)]
  #[table_name="game_modes"]
  #[belongs_to(GameTitles)]
  #[derive(Debug, Queryable, Associations)]
  pub struct GameModes {
    pub game_mode_id: i32,
    pub game_title_id: i32,
    pub mode_name: String,
    pub team_size: i32
  }

  #[primary_key(user_id)]
  #[table_name="users"]
  #[derive(Debug, Queryable, Associations)]
  pub struct Users {
    pub user_id: i32,
    pub bot: bool,
    pub discriminator: i32,
    pub name: String
  }

  #[table_name = "user_ratings"]
  #[derive(Debug, Queryable, Associations)]
  #[belongs_to(Users, GameModes)]
  pub struct UserRatings {
    pub id: i32,
    pub user_id: i32,
    pub rating: Option<BigDecimal>,
    pub deviation: Option<BigDecimal>,
    pub volatility: Option<BigDecimal>,
    pub game_mode_id: i32
  }
}

pub mod insert {
  use bigdecimal::BigDecimal;
  use schema::*;

  #[primary_key(game_title_id)]
  #[table_name="game_titles"]
  #[derive(Insertable)]
  pub struct GameTitles {
    pub game_name: String
  }

  #[primary_key(game_mode_id)]
  #[table_name="game_modes"]
  #[belongs_to(GameTitles)]
  #[derive(Insertable)]
  pub struct GameModes {
    pub game_title_id: i32,
    pub mode_name: String,
    pub team_size: i32
  }

  #[table_name="users"]
  #[derive(Insertable)]
  pub struct Users {
    pub bot: bool,
    pub discriminator: i32,
    pub name: String
  }

  #[table_name = "user_ratings"]
  #[derive(Insertable)]
  #[belongs_to(Users, GameModes)]
  pub struct UserRatings {
    pub user_id: i32,
    pub rating: Option<BigDecimal>,
    pub deviation: Option<BigDecimal>,
    pub volatility: Option<BigDecimal>,
    pub game_mode_id: i32
  }
}
