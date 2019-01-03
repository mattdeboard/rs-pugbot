use crate::commands::error_embed;
use crate::consume_message;
use crate::models::game::{Game, Phases};
use crate::traits::phased::Phased;
use serenity::model::channel::Message;

command!(mapvote(ctx, msg, args) {
  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();
  map_vote(game, msg, true, args.single::<usize>()? as i32)?;
});

#[allow(unused_must_use)]
pub fn map_vote(
  game: &mut Game,
  msg: &Message,
  send_embed: bool,
  map_index: i32,
) -> Result<(), &'static str> {
  if game.phase != Some(Phases::MapSelection) {
    let err = "We're not picking maps right now!";

    if send_embed {
      consume_message(msg, error_embed(err));
    }

    return Err(err);
  }

  if !game.draft_pool.members.contains(&msg.author) {
    match msg.author.direct_message(|m| m.content(
      "Sorry, but you're not allowed to map vote because you're not registered to play!"
    )) {
      Ok(_) => {
        msg.reply("You're welcome");
        Ok(())
      },
      Err(why) => {
        println!("Error sending message: {:?}", why);
        let err = "Had some kind of problem sending you a message.";
        msg.reply(err);

        if send_embed {
          consume_message(msg, error_embed(err));
        }

        Err(err)
      }
    }
  } else {
    if let Some(vote_count) = game.map_votes.clone().get(&map_index) {
      game.map_votes.insert(map_index, vote_count + 1);
      game.register_vote(msg.author.id);
      game.next_phase();

      if game.phase == Some(Phases::ResultRecording) && send_embed {
        consume_message(msg, game.map_winner_embed(164, 255, 241).unwrap());
      }
      Ok(())
    } else {
      let err = "Invalid map selection.";

      if send_embed {
        consume_message(msg, error_embed(err));
      }

      Err(err)
    }
  }
}
