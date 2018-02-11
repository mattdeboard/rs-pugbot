use models::game::{ Game, Phases };
use traits::has_members::HasMembers;
use traits::phased::Phased;
use consume_message;
use queue_size;
use team_count;

command!(pick(ctx, msg, args) {
  let user_index = args.single::<usize>().unwrap();
  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();

  if game.phase != Some(Phases::PlayerDrafting) {
    return panic!("We're not drafting right now!");
  }

  let user = game.draft_pool.pop_available_player(&user_index).unwrap();
  game.next_team_to_draft().add_member(user);

  let max_turns: u32 = queue_size() - team_count().unwrap();

  if game.turn_number == max_turns as usize {
    game.next_phase();
  } else {
    game.turn_number += 1;
  }

  consume_message(msg, game.drafting_complete_embed(165, 255, 241).unwrap());
});

