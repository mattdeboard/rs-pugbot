use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::framework::standard::{ Args, Command, CommandError };
use serenity::model::channel::{ Embed, Message };
use std::marker::{ PhantomData, Send, Sync };
use ::traits::pool_availability::PoolAvailability;
use typemap::Key;

#[allow(non_camel_case_types)]
pub struct remove<T: Key<Value=T>> {
  pub phantom: PhantomData<fn(T)>
}

impl<T> Command for remove<T> where T: PoolAvailability + Key<Value=T> + Send + Sync {
  #[allow(unreachable_code, unused_mut)]
  fn execute(&self, mut ctx: &mut Context, msg: &Message, _: Args) ->
    Result<(), CommandError> {
      {
        let mut data = ctx.data.lock();
        let mut draft_pool = data.get_mut::<T>().unwrap();
        let author = msg.author.clone();
        let embed: Embed = draft_pool.remove_member(author);
        msg.channel_id.send_message(|m| m.embed(|_| CreateEmbed::from(embed)));
      }
      Ok(())
    }
}
