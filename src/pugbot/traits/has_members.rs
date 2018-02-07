use serenity::model::channel::{ Embed };
use serenity::model::user::User;

pub trait HasMembers {
  fn members(&self) -> Vec<User>;
  fn add_member(&mut self, user: User) -> Option<Embed>;
  fn remove_member(&mut self, user: User) -> Option<Embed>;
  fn members_changed_embed(&mut self, r: u8, g: u8, b: u8) -> Option<Embed>;
}
