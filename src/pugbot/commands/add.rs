use serenity::builder::CreateEmbed;
use serenity::model::channel::{ Embed, Message };
use serenity::model::user::User;
use ::models::game::Game;
use ::traits::has_members::HasMembers;

command!(add(ctx, msg, _args) {
  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();
  update_members(game, msg, true);
});

pub fn update_members(game: &mut Game, msg: &Message, send_embed: bool) -> Vec<User> {
  // The `send_embed` parameter exists only as a way to avoid trying to hit the Discord
  // API during testing.
  if !game.draft_pool.is_open() {
    let embed: Embed = game.draft_pool.members_full_embed(165, 255, 241);
    if send_embed {
      consume_message(msg, embed);
    }
  } else {
    let author = msg.author.clone();
    let embed: Embed = game.draft_pool.add_member(author);
    if send_embed {
      consume_message(msg, embed);
    }
  }
  game.draft_pool.members.clone()
}

fn consume_message(msg: &Message, embed: Embed) {
  msg.channel_id.send_message(|m| m.embed(|_| CreateEmbed::from(embed))).unwrap();
}
