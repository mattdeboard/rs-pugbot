pub mod users {
  #[derive(Debug, Queryable, Identifiable, Associations)]
  #[table_name = "users"]
  #[primary_key(user_id)]
  pub struct Users {
    pub user_id: u64,
    pub bot: bool,
    pub discriminator: u16,
    pub name: String
  }
}

pub mod user_ratings {
  #[belongs_to(tables::users::Users)]
  #[table_name = "user_ratings"]
  #[primary_key(user_rating_id)]
  #[derive(Debug, Queryable, Identifiable, Associations)]
  pub struct UserRatings {
    pub user_rating_id: u64,
    pub user_id: u64,
    pub rating: f64
  }
}
