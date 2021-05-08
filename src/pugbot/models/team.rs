use crate::traits::has_members::HasMembers;
// use glicko2::Glicko2Rating;
use serenity::model::user::User;
use std::clone::Clone;
use typemap::Key;

#[derive(Debug, Clone, Serialize)]
pub struct Team {
  pub id: usize,
  pub captain: Option<User>,
  pub members: Vec<User>,
  // pub glicko2_ratings: Vec<Glicko2Rating>,
}

impl Key for Team {
  type Value = Vec<Team>;
}

impl HasMembers for Team {
  fn members(&self) -> Vec<User> {
    self.members.clone()
  }

  fn add_member(&mut self, user: User) -> Result<usize, &str> {
    self.members.push(user);
    self.members.dedup();
    Ok(self.members.len())
  }

  fn remove_member(&mut self, user: User) -> Result<usize, &str> {
    self.members.retain(|m| m.id != user.id);
    self.members.dedup();
    Ok(self.members.len())
  }
}
