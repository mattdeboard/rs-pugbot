use serenity::model::id::UserId;

pub trait HasMembers {
  fn members(&self) -> Vec<UserId>;
  fn add_member(&mut self, user: UserId) -> Result<usize, &str>;
  fn remove_member(&mut self, user: UserId) -> Result<usize, &str>;
}
