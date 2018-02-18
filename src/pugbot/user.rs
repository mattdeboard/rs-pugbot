use diesel::ExpressionMethods;
use diesel::prelude::Insertable;
use diesel::dsl::Eq;
use schema::users::dsl::*;
use serenity::model::user::User;

impl<'a> Insertable<users> for &'a User {
  type Values = <(
    Eq<bot, bool>,
    Eq<discriminator, i32>,
    Eq<name, &'a String>,
    Eq<discord_user_id, i32>
  ) as Insertable<users>>::Values;

  fn values(self) -> Self::Values {
    (
      bot.eq(self.bot),
      discriminator.eq(self.discriminator as i32),
      name.eq(&self.name),
      discord_user_id.eq(self.id.0 as i32),
    ).values()
  }
}
