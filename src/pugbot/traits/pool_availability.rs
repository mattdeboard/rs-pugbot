use serenity::model::channel::Embed;
use traits::has_members::HasMembers;

pub trait PoolAvailability: HasMembers {
  fn is_open(&self) -> bool;
  fn members_full_embed(&mut self, r: u8, g: u8, b: u8) -> Option<Embed>;
}
