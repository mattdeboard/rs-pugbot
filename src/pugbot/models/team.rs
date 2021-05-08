use crate::traits::has_members::HasMembers;
// use glicko2::Glicko2Rating;
use serenity::{client::Context, model::id::UserId};
use std::clone::Clone;
use typemap::Key;

#[derive(Debug, Clone, Serialize)]
pub struct Team {
  pub id: usize,
  pub captain: Option<UserId>,
  pub members: Vec<UserId>,
  // pub glicko2_ratings: Vec<Glicko2Rating>,
}

impl Key for Team {
  type Value = Vec<Team>;
}

impl Team {
  pub async fn get_users(&self, ctx: &Context) -> Vec<String> {
    let mut users: Vec<String> = vec![];

    for user_id in &self.members {
      let user = user_id.to_user_cached(&ctx.cache).await;
      if let Some(u) = user {
        users.push(u.name);
      }
    }
    users
  }
}

impl HasMembers for Team {
  fn members(&self) -> Vec<UserId> {
    self.members.clone()
  }

  fn add_member(&mut self, user: UserId) -> Result<usize, &str> {
    self.members.push(user);
    self.members.dedup();
    Ok(self.members.len())
  }

  fn remove_member(&mut self, user_id: UserId) -> Result<usize, &str> {
    self.members.retain(|m| m != &user_id);
    self.members.dedup();
    Ok(self.members.len())
  }
}
