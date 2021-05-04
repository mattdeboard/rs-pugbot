use crate::commands::pick::*;
use serenity::framework::standard::macros::group;

#[group("Player Drafting")]
#[description("Commands here are available to Captains only")]
#[commands(pick)]
pub(crate) struct PlayerDrafting;
