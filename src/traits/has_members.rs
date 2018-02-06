use serenity::model::channel::{ Embed };
use serenity::model::user::User;

pub trait HasMembers {
  fn add_member(&mut self, user: User) -> Embed;
  fn remove_member(&mut self, user: User) -> Embed;
  fn create_embed(&mut self, r: u8, g: u8, b: u8) -> Embed;
}
