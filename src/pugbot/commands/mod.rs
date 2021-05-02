pub mod add;
pub mod mapvote;
pub mod pick;
pub mod remove;
use add::ADD_COMMAND;
use mapvote::MAPVOTE_COMMAND;
use pick::PICK_COMMAND;
use remove::REMOVE_COMMAND;
use serenity::framework::standard::macros::{group, help};
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

// These groups really beg to be defined within the same module as the
// commands they are grouping.
// The "names" in the `#[commands]` attr is transformed into an all-caps version
// with a `_COMMAND` suffix, which needs to be explicitly brought into scope,
// unless of course everything happens to be in the same module to begin with.
//
// Similarly, the output from these `#[group]` attr macros will be all-caps
// versions of the unit structs defined here, but with a `_GROUP` suffix.
//
// Ideally, all this code would be reshaped such that the need to import the
// mangled names is reduced. This could be achieved by arranging the modules
// based on groups first, then commands. Finally, the framework config could
// happen via functions in each group's module that receive and return a
// `StandardFramework`, each adding their groups to the config.

#[group("Player Registration")]
#[commands(add, remove)]
pub(crate) struct PlayerRegistration;

#[group("Player Drafting")]
#[description("Commands here are available to Captains only")]
#[commands(pick)]
pub(crate) struct PlayerDrafting;

#[group("Map Voting")]
#[commands(mapvote)]
pub(crate) struct MapVoting;

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
