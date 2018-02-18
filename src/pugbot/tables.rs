pub mod query {
  use bigdecimal::BigDecimal;

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
  #[belongs_to(Users)]
  pub struct UserRatings {
    pub id: i32,
    pub user_id: i32,
    pub rating: Option<BigDecimal>,
    pub deviation: Option<BigDecimal>,
    pub volatility: Option<BigDecimal>
  }
}

pub mod insert {
  use bigdecimal::BigDecimal;
  use schema::*;

  #[table_name="users"]
  #[derive(Insertable)]
  pub struct Users {
    pub bot: bool,
    pub discriminator: i32,
    pub name: String
  }

  #[table_name = "user_ratings"]
  #[derive(Insertable)]
  #[belongs_to(Users)]
  pub struct UserRatings {
    pub user_id: i32,
    pub rating: Option<BigDecimal>,
    pub deviation: Option<BigDecimal>,
    pub volatility: Option<BigDecimal>
  }
}
