use crate::consume_message;
use crate::models::game::{Game, Phases};
use crate::queue_size;
use crate::team_count;
use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;
use serenity::model::channel::Message;

command!(pick(ctx, msg, args) {
  let user_index = args.single::<usize>().unwrap();
  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();
  draft_player(game, msg, true, user_index);
});

pub fn draft_player(
  game: &mut Game,
  msg: &Message,
  send_embed: bool,
  user_index: usize,
) {
  if game.phase != Some(Phases::PlayerDrafting) {
    panic!("We're not drafting right now!");
  }

  let user = game.draft_pool.pop_available_player(&user_index).unwrap();
  game.next_team_to_draft().add_member(user);

  let max_turns: u32 = queue_size() - team_count();

  if game.turn_number == max_turns as usize {
    game.next_phase();
    consume_message(msg, game.drafting_complete_embed(165, 255, 241).unwrap());
    consume_message(msg, game.map_selection_embed(164, 255, 241).unwrap());
  } else {
    game.turn_number += 1;
  }
}
