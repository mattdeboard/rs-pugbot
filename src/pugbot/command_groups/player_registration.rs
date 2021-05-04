use crate::commands::{add::*, remove::*};
use serenity::framework::standard::macros::group;

#[group("Player Registration")]
#[commands(add, remove)]
pub(crate) struct PlayerRegistration;
