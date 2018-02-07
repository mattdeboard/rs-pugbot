extern crate kankyo;

use rand::{thread_rng, Rng};
use serenity::model::channel::{ Embed, EmbedFooter };
use serenity::model::user::User;
use serenity::utils::Colour;
use std::clone::Clone;
use typemap::Key;

use ::traits::has_members::HasMembers;

#[derive(Debug, Clone)]
pub struct Team {
  pub id: usize,
  pub captain: Option<User>,
  pub members: Vec<User>
}

impl Team {
  pub fn select_captain(&self, queue: &Vec<User>) -> Option<User> {
    let mut rng = thread_rng();
    match rng.choose(&queue) {
      Some(user) => Some(user.clone()),
      None => None
    }
  }

  pub fn with_captain(old_team: &Team, captain: Option<User>) -> Team {
    Team {
      id: old_team.id,
      captain: captain,
      members: old_team.members.clone()
    }
  }
}

impl Key for Team {
  type Value = Vec<Team>;
}

impl HasMembers for Team {
  fn add_member(&mut self, user: User) -> Embed {
    self.members.push(user);
    self.members.dedup();
    self.members_changed_embed(255, 223, 165)
  }

  fn remove_member(&mut self, user: User) -> Embed {
    self.members.retain(|m| m.id != user.id);
    self.members.dedup();
    self.members_changed_embed(255, 223, 165)
  }

  fn members_changed_embed(&mut self, r: u8, g: u8, b: u8) -> Embed {
    let members = &self.members;

    Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(members.into_iter().map(|m| m.clone().name).collect()),
      footer: Some(EmbedFooter {
        icon_url: None,
        proxy_icon_url: None,
        text: match self.captain {
          Some(ref user) => format!("{} is Team {} Captain", user.name, self.id),
          None => format!("Team {} has no captain, yet", self.id)
        }
      }),
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some(format!("Team {} has {} members:", self.id, self.members.len())),
      url: None,
      video: None
    }
  }
}
