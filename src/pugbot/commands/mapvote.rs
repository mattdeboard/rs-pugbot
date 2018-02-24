use models::game::{ Game, Phases };
use traits::phased::Phased;

command!(mapvote(ctx, msg, args) {
  let mut data = ctx.data.lock();
  let game = data.get_mut::<Game>().unwrap();

  if game.phase != Some(Phases::MapSelection) {
    return panic!("We're not picking maps right now!");
  }

  if !game.draft_pool.members.contains(&msg.author) {
    match msg.author.direct_message(|m| m.content(
      "Sorry, but you're not allowed to map vote because you're not registered to play!"
    )) {
      Ok(_) => {
        let _ = msg.reply("You're welcome");
      },
      Err(why) => {
        println!("Error sending message: {:?}", why);
        let _ = msg.reply("Had some kind of problem sending you a message.");
      }
    }
  } else {
    let map_index = args.single::<usize>().unwrap() as i32;

    if let Some(vote_count) = game.map_votes.clone().get(&map_index) {
      game.map_votes.insert(map_index, vote_count + 1);
      game.register_vote(msg.author.id);

      if game.eligible_voter_ids.len() == 0 {
        game.next_phase();
      }
    } else {
      return panic!("Invalid map key");
    }
  }
});
