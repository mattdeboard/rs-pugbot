#[table_name="users"]
#[primary_key(user_id)]
#[derive(Queryable, Insertable, Associations)]
pub struct Users {
  pub user_id: u64,
  pub bot: bool,
  pub discriminator: u16,
  pub name: String
}

#[table_name = "user_ratings"]
#[derive(Queryable, Associations)]
#[belongs_to(Users)]
pub struct UserRatings {
  pub id: u64,
  pub user_id: u64,
  pub rating: f64
}
