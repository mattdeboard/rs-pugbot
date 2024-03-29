use crate::models::{game_mode::GameMode, user::DiscordUser};
use crate::schema::*;
use bigdecimal::BigDecimal;

#[derive(Debug, Identifiable, Insertable, Queryable, Associations)]
#[table_name = "user_ratings"]
#[belongs_to(DiscordUser, foreign_key = "user_id")]
#[belongs_to(GameMode, foreign_key = "game_mode_id")]
pub struct UserRating {
  pub id: Option<i32>,
  pub user_id: i32,
  pub rating: Option<BigDecimal>,
  pub deviation: Option<BigDecimal>,
  pub volatility: Option<BigDecimal>,
  pub game_mode_id: i32,
}
