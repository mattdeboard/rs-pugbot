use serenity::builder::CreateEmbed;
use serenity::model::channel::{ Embed, Message };
use ::models::game::Game;
use ::traits::has_members::HasMembers;

command!(add(ctx, msg, _args) {
  fn consume_message(msg: &Message, embed: Embed) {
    match msg.channel_id.send_message(|m| m.embed(|_| CreateEmbed::from(embed))) {
      Ok(_) => (),
      Err(what) => panic!(what)
    }
  }

  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();

  if !game.draft_pool.is_open() {
    let embed: Embed = game.draft_pool.members_full_embed(165, 255, 241);
    consume_message(msg, embed);
  } else {
    let author = msg.author.clone();
    let embed: Embed = game.draft_pool.add_member(author);
    consume_message(msg, embed);
  }
});
