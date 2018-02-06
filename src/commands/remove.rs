use serenity::builder::CreateEmbed;
use serenity::model::channel::{ Embed };
use ::models::draft_pool::*;
use ::traits::has_members::HasMembers;

command!(remove(ctx, msg, _args) {
  let mut data = ctx.data.lock();
  let mut draft_pool = data.get_mut::<DraftPool>().unwrap();
  let author = msg.author.clone();
  let embed: Embed = draft_pool.remove_member(author);
  msg.channel_id.send_message(|m| m.embed(|_| CreateEmbed::from(embed)));
});
