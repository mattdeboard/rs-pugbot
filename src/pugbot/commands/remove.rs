use crate::models::game::GameContainer;
use crate::{queue_size, traits::has_members::HasMembers};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::prelude::Context;
use serenity::{
  builder::CreateEmbedAuthor, framework::standard::CommandResult,
};

#[command]
#[aliases("r")]
#[description("Removes yourself from the draft pool.")]
#[allow(unused_must_use)]
pub(crate) async fn remove(ctx: &Context, msg: &Message) -> CommandResult {
  remove_member(ctx, msg, true).await;
  Ok(())
}

#[allow(unused_must_use)]
pub async fn remove_member(
  ctx: &Context,
  msg: &Message,
  send_embed: bool,
) -> Vec<User> {
  let mut data = ctx.data.write().await;
  let game = data.get_mut::<GameContainer>().unwrap();
  let author = msg.author.clone();

  if let Ok(_) = game.draft_pool.remove_member(&author) {
    let next_id = game.turn_taker.next().unwrap();

    if let Some(next_team) = game.teams.get_mut(next_id as usize) {
      next_team.remove_member(&author);
    }

    if send_embed {
      let embed_descrip: Vec<String> = game
        .draft_pool
        .members
        .clone()
        .into_iter()
        .map(|m| m.clone().name)
        .collect();
      msg
        .channel_id
        .send_message(&ctx.http, |m| {
          m.embed(|e| {
            let mut cea = CreateEmbedAuthor::default();
            cea.name(&author.name);

            if let Some(url) = &author.avatar_url() {
              cea.icon_url(url);
            }

            e.set_author(cea);
            e.color(super::SUCCESS_EMBED_COLOR);
            e.description(embed_descrip.join("\n"));
            e.footer(|f| {
              f.text(format!(
                "{} of {} users in queue",
                game.draft_pool.members.len(),
                queue_size()
              ))
            })
          })
        })
        .await;
    }
  }

  game.draft_pool.members()
}

#[cfg(test)]
mod tests {
  use serde;
  use serde_json;
  use serenity::{self};

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::models::game::{Game, Phases};
  use crate::models::{draft_pool::DraftPool, game::GameContainer};
  use crate::{commands, struct_from_json};
  use serenity::model::channel::Message;
  use std::fs::File;
  use tokio;

  async fn test_context(msg: &Message) -> serenity::client::Context {
    let context = commands::mock_context::tests::mock_context();
    {
      let game = Game::new(
        vec![],
        DraftPool::new(vec![msg.author.clone()], 12),
        1,
        Vec::new(),
        // Draft pool max size: 12 (2 * 6)
        2,
        6,
      );
      let mut data = context.data.write().await;
      data.insert::<GameContainer>(game);
    }
    context
  }

  #[tokio::test]
  async fn test_remove_member() {
    let message = struct_from_json!(Message, "message");
    let context = test_context(&message).await;

    {
      let data = context.data.read().await;
      if let Some(game) = data.get::<GameContainer>() {
        assert_eq!(game.phase, Some(Phases::PlayerRegistration));
        // Precondition. Draft pool should have 1 member, the author of
        // the message.
        assert_eq!(game.draft_pool.members.len(), 1);
      }
    }
    {
      let members =
        commands::remove::remove_member(&context, &message, false).await;
      assert_eq!(members.len(), 0);
    }

    let data = context.data.read().await;

    if let Some(game) = data.get::<GameContainer>() {
      // Post condition. Pool should now be empty.
      assert_eq!(game.draft_pool.members.len(), 0);
    }
  }
}
