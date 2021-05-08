use serenity::model::user::User;
use std::collections::HashMap;
use typemap::Key;

use crate::queue_size;
use crate::traits::has_members::HasMembers;
use crate::traits::pool_availability::*;

pub struct DraftPool {
  pub members: Vec<User>,
  pub available_players: HashMap<usize, User>,
  pub max_members: u32,
}

impl DraftPool {
  pub fn new(members: Vec<User>, max_members: u32) -> DraftPool {
    DraftPool {
      members: members,
      available_players: HashMap::new(),
      max_members: max_members,
    }
  }

  pub fn available_players(self) -> HashMap<usize, User> {
    self.available_players
  }

  pub fn generate_available_players(&mut self) {
    for (idx, member) in self.members.clone().iter().enumerate() {
      self.available_players.insert(idx + 1, member.clone());
    }
  }

  pub fn pop_available_player(
    &mut self,
    player_number: &usize,
  ) -> Option<User> {
    self.available_players.remove(player_number)
  }
}

impl PoolAvailability for DraftPool {
  fn is_open(&self) -> bool {
    (self.members().len() as u32) < queue_size()
  }
}

impl HasMembers for DraftPool {
  fn members(&self) -> Vec<User> {
    self.members.clone()
  }

  fn add_member(&mut self, user: User) -> Result<usize, &str> {
    self.members.push(user);
    self.members.dedup();

    if (self.members.len() as u32) == queue_size() {
      return Err("Draft pool is full!");
    }

    Ok(self.members.len())
  }

  fn remove_member(&mut self, user: User) -> Result<usize, &str> {
    self.members.retain(|m| m.id != user.id);
    self.members.dedup();
    Ok(self.members.len())
  }
}

impl Key for DraftPool {
  type Value = DraftPool;
}
