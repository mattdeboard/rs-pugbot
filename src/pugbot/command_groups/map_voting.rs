use crate::commands::mapvote::*;
use serenity::framework::standard::macros::group;

#[group("Map Voting")]
#[commands(mapvote)]
pub(crate) struct MapVoting;
