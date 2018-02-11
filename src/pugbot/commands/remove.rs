use models::game::Game;
use traits::has_members::HasMembers;
use consume_message;

command!(remove(ctx, msg) {
  let mut data = ctx.data.lock();
  let mut game = data.get_mut::<Game>().unwrap();
  let author = msg.author.clone();
  if let Some(embed) = game.draft_pool.remove_member(author) {
    consume_message(msg, embed)
  }
});
