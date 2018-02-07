use traits::has_members::HasMembers;
use serenity::model::channel::Embed;

pub trait PoolAvailability: HasMembers {
  fn is_open(&self) -> bool;
  fn members_full_embed(&mut self, r: u8, g: u8, b: u8) -> Embed;
}
