use bigdecimal::BigDecimal;
use schema::*;

#[table_name = "user_ratings"]
#[derive(Debug, Insertable, Queryable, Associations)]
#[belongs_to(users, game_modes)]
pub struct UserRating {
  pub id: Option<i32>,
  pub user_id: i32,
  pub rating: Option<BigDecimal>,
  pub deviation: Option<BigDecimal>,
  pub volatility: Option<BigDecimal>,
  pub game_mode_id: i32,
}
