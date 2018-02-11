use serenity::model::channel::{ Message };
use serenity::model::user::User;

use models::game::{ Game, Phases };
use traits::has_members::HasMembers;
use traits::pool_availability::PoolAvailability;
use traits::phased::Phased;
use consume_message;
use queue_size;

command!(add(ctx, msg) {
  {
    let mut data = ctx.data.lock();
    let game = data.get_mut::<Game>().unwrap();

    update_members(game, msg, true);
  }
});

pub fn update_members(game: &mut Game, msg: &Message, send_embed: bool) -> Vec<User> {
  // The `send_embed` parameter exists only as a way to avoid trying to hit the Discord
  // API during testing.
  if game.phase != Some(Phases::PlayerRegistration) {
    if let Some(embed) = game.draft_pool.members_full_embed(165, 255, 241) {
      if send_embed {
        consume_message(msg, embed);
      }
    }
  } else {
    let author = msg.author.clone();
    if let Some(embed) = game.draft_pool.add_member(author) {
      if send_embed {
        consume_message(msg, embed);
      }
    }
  }

  let members = game.draft_pool.members();
  if members.len() as u32 == queue_size() &&
    game.phase == Some(Phases::PlayerRegistration) {
      game.next_phase();
    }

  members
}
