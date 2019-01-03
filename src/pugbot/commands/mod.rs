pub mod add;
pub mod mapvote;
pub mod pick;
pub mod remove;
use serenity::model::channel::Embed;
use serenity::utils::Colour;

pub fn error_embed(description: &'static str) -> Embed {
  Embed {
    author: None,
    colour: Colour::from_rgb(255, 0, 0),
    description: Some(String::from(description)),
    footer: None,
    fields: Vec::new(),
    image: None,
    kind: "rich".to_string(),
    provider: None,
    thumbnail: None,
    timestamp: None,
    title: Some(String::from("ERROR")),
    url: None,
    video: None,
  }
}
