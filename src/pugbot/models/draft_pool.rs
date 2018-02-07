use serenity::model::channel::{ Embed, EmbedFooter };
use serenity::model::user::User;
use serenity::utils::Colour;
use ::traits::pool_availability::*;
use typemap::Key;

use queue_size;
use ::traits::has_members::HasMembers;

pub struct DraftPool {
  pub members: Vec<User>,
}

impl PoolAvailability for DraftPool {
  fn is_open(&self) -> bool {
    (self.members().len() as u32) < queue_size()
  }

  fn members_full_embed(&mut self, r: u8, g: u8, b: u8) -> Embed {
    let members = self.members.clone();

    Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(members.into_iter().map(|m| m.clone().name).collect()),
      footer: Some(EmbedFooter {
        icon_url: None,
        proxy_icon_url: None,
        text: format!("The queue is full! Now picking captains!")
      }),
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some("Members in queue:".to_string()),
      url: None,
      video: None
    }
  }
}

impl HasMembers for DraftPool {
  fn members(&self) -> Vec<User> {
    self.members
  }

  fn add_member(&mut self, user: User) -> Embed {
    self.members.push(user);
    self.members.dedup();

    if (self.members.len() as u32) == queue_size() {
      return self.members_full_embed(165, 255, 241);
    }

    self.members_changed_embed(165, 255, 241)
  }

  fn remove_member(&mut self, user: User) -> Embed {
    self.members.retain(|m| m.id != user.id);
    self.members.dedup();
    self.members_changed_embed(165, 255, 241)
  }

  fn members_changed_embed(&mut self, r: u8, g: u8, b: u8) -> Embed {
    let members = self.members.clone();

    Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(members.into_iter().map(|m| m.clone().name).collect()),
      footer: Some(EmbedFooter {
        icon_url: None,
        proxy_icon_url: None,
        text: format!("{} of {} users in queue", self.members.len(), queue_size())
      }),
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some("Members in queue:".to_string()),
      url: None,
      video: None
    }
  }
}

impl Key for DraftPool {
  type Value = DraftPool;
}
