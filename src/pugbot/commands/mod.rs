pub mod add;
pub mod mapvote;
pub mod mock_context;
pub mod pick;
pub mod remove;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};
use serenity::framework::standard::macros::help;
use serenity::framework::standard::{
  help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::Context;
use serenity::utils::Colour;
use std::collections::HashSet;

pub fn error_embed(description: &'static str) -> CreateEmbed {
  let mut create_embed = CreateEmbed::default();
  create_embed.set_author(CreateEmbedAuthor::default());
  create_embed.color(Colour::from_rgb(255, 0, 0));
  create_embed.description(String::from(description));
  create_embed.set_footer(CreateEmbedFooter::default());
  create_embed.title(String::from("ERROR"));
  create_embed
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
