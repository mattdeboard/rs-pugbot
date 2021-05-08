use crate::traits::has_members::HasMembers;

pub trait PoolAvailability: HasMembers {
  fn is_open(&self) -> bool;
}
