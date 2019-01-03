use crate::consume_message;
use crate::models::game::{Game, Phases};

use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;
use serenity::model::channel::{Embed, Message};
use serenity::utils::Colour;

command!(pick(ctx, msg, args) {
  let user_index = args.single::<usize>().unwrap();
  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();
  draft_player(game, msg, true, user_index)?;
});

pub fn draft_player<'a>(
  game: &'a mut Game,
  msg: &Message,
  send_embed: bool,
  user_index: usize,
) -> Result<(), &'static str> {
  if game.phase != Some(Phases::PlayerDrafting) && send_embed {
    let err = "We're not drafting right now!";
    consume_message(msg, error_embed(err));
    return Err("We're not drafting right now!");
  }

  if let Some(user) = game.draft_pool.pop_available_player(&user_index) {
    let next_id = game.turn_taker.next().unwrap();
    game.teams[next_id as usize].add_member(user);
  } else {
    let err =
      "The user selected for drafting has been drafted or is otherwise invalid";
    if send_embed {
      consume_message(msg, error_embed(err));
    }
    return Err(err);
  }

  if game.draft_pool.available_players.len() == 0 {
    game.next_phase();

    if send_embed {
      consume_message(
        msg,
        game.drafting_complete_embed(165, 255, 241).unwrap(),
      );
      consume_message(msg, game.map_selection_embed(164, 255, 241).unwrap());
    }
  } else {
    game.turn_number += 1;
  }
  Ok(())
}

fn error_embed(description: &'static str) -> Embed {
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
