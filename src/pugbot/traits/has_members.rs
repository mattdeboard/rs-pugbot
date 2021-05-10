use serenity::model::user::User;

pub trait HasMembers {
  fn members(&self) -> Vec<User>;
  fn add_member(&mut self, user: User) -> Result<usize, String>;
  fn remove_member(&mut self, user: &User) -> Result<usize, &str>;
}
