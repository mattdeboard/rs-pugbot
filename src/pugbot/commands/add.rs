use serenity::model::channel::Message;
use serenity::model::user::User;
use serenity::{framework::standard::macros::command, utils::Colour};

use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;

use crate::models::game::GameContainer;
use crate::{models::game::Phases, queue_size};
use serenity::framework::standard::CommandResult;
use serenity::prelude::Context;

#[command]
#[aliases("a")]
#[description(r#"Adds yourself to the pool of draftable players, or "draft pool."

Once enough people to fill out all the teams have added themselves, captains will be automatically selected at random, and drafting will begin."#
)]
#[allow(unused_must_use)]
pub(crate) async fn add(ctx: &Context, msg: &Message) -> CommandResult {
  update_members(ctx, msg, true); // XXX: should this be the return value?
  Ok(())
}

#[allow(unused_must_use)]
pub async fn update_members(
  ctx: &Context,
  msg: &Message,
  send_embed: bool,
) -> Vec<User> {
  let mut data = ctx.data.write().await;
  if let Some(game) = data.get_mut::<GameContainer>() {
    let embed_descrip: String = game
      .draft_pool
      .members
      .clone()
      .into_iter()
      .map(|m| m.clone().name)
      .collect();
    let embed_color = Colour::from_rgb(165, 255, 241);
    let author = msg.author.clone();
    if game.phase == Some(Phases::PlayerRegistration) {
      match game.draft_pool.add_member(author) {
        Ok(_) => {
          if send_embed {
            msg.channel_id.send_message(&ctx.http, |m| {
              m.embed(|e| {
                e.color(embed_color);
                e.description(embed_descrip);
                e.footer(|f| {
                  f.text(format!(
                    "{} of {} users in queue",
                    game.draft_pool.members.len(),
                    queue_size()
                  ))
                })
              })
            });
          }
        }
        _ => {
          if send_embed {
            msg.channel_id.send_message(&ctx.http, |m| {
              m.embed(|e| {
                e.color(embed_color);
                e.description(embed_descrip);
                e.footer(|f| {
                  f.text(format!("The queue is full! Now picking captains!"))
                });
                e.title("Members in queue:".to_string())
              })
            });
          }
        }
      }
    } else {
      if send_embed {
        msg.channel_id.send_message(&ctx.http, |m| {
          m.embed(|e| {
            e.color(embed_color);
            e.description(embed_descrip);
            e.footer(|f| {
              f.text(format!("The queue is full! Now picking captains!"))
            });
            e.title("Members in queue:".to_string())
          })
        });
      }
    }
    game.next_phase();
    return game.draft_pool.members();
  };
  return vec![];
}

#[cfg(test)]
mod tests {
  use serde;
  use serde_json;
  use serenity::{self, client::Context, model::prelude::User};

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::models::game::{Game, Phases};
  use crate::models::{draft_pool::DraftPool, game::GameContainer};
  use crate::{commands, struct_from_json};
  use serenity::model::channel::Message;
  use std::env;
  use std::fs::File;

  async fn test_context() -> Context {
    let context = commands::mock_context::tests::mock_context();
    {
      let game = Game::new(
        vec![],
        DraftPool::new(vec![User::default()], 12),
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

  #[allow(unused_must_use)]
  #[tokio::test]
  async fn test_update_members() {
    let message = struct_from_json!(Message, "message");
    let key = "TEAM_SIZE";
    env::set_var(key, "1");
    let context = test_context().await;
    {
      let data = context.data.read().await;
      if let Some(game) = data.get::<GameContainer>() {
        assert_eq!(game.phase, Some(Phases::PlayerRegistration));
      }
    }

    {
      let members =
        commands::add::update_members(&context, &message, false).await;
      // There should be one member in the members vec to start with: our test
      // user. `update_members` above should add an additional user, the
      // author of the message (which is defined in
      // src/tests/resources/message.json).
      assert_eq!(members.len(), 2);
    }

    let data = context.data.read().await;
    if let Some(game) = data.get::<GameContainer>() {
      assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    }
  }
}
