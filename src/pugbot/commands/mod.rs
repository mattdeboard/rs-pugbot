pub mod add;
pub mod mapvote;
pub mod pick;
pub mod remove;

use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
  help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::channel::Embed;
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use serenity::utils::Colour;
use std::collections::HashSet;

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

// XXX: From reading the serenity docs, honestly it's unclear if this is needed
// or if we're reimplementing whatever the default behavior is.
// Regardless, we can't simply feed `help_commands::with_embeds()` to the
// `StandardFramework::help()` method since the types do not align.
// It demands whatever the output is from this `#[help]` attr macro.
#[help]
pub(crate) async fn help_cmd(
  context: &Context,
  msg: &Message,
  args: Args,
  help_options: &'static HelpOptions,
  groups: &[&'static CommandGroup],
  owners: HashSet<UserId>,
) -> CommandResult {
  let _ = help_commands::with_embeds(
    context,
    msg,
    args,
    help_options,
    groups,
    owners,
  )
  .await;
  Ok(())
}
