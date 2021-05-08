use crate::models::user_rating::UserRating;
use crate::schema::users;
use diesel::dsl::Eq;
use diesel::pg::Pg;
use diesel::prelude::{Insertable, Queryable};
use diesel::ExpressionMethods;
use serenity::model::id::UserId;
use serenity::model::user::User;

#[derive(Debug)]
pub struct DiscordUser {
  pub user_id: i32,
  pub discord_user_id: UserId,
  pub bot: bool,
  pub discriminator: u16,
  pub name: String,
  pub avatar: Option<String>,
}

impl<'a> Insertable<users::table> for &'a User {
  type Values = <(
    Eq<users::bot, bool>,
    Eq<users::discriminator, i32>,
    Eq<users::name, &'a String>,
    Eq<users::discord_user_id, i32>,
  ) as Insertable<users::table>>::Values;

  fn values(self) -> Self::Values {
    (
      users::bot.eq(self.bot),
      users::discriminator.eq(self.discriminator as i32),
      users::name.eq(&self.name),
      // XXX: seems a little risky to cast this.
      users::discord_user_id.eq(*self.id.as_u64() as i32),
    )
      .values()
  }
}

impl From<DiscordUser> for User {
  fn from(discord_user: DiscordUser) -> User {
    // `User` is set as non-exhaustive, which means the compiler enforces the
    // assumption that *we* can never know what all the fields will be.
    // To construct this we need to start from the empty `Default` to mitigate
    // the risk of having uninitialized fields and satisfy the compiler.
    let mut user = User::default();

    // XXX: seems like `UserId::to_user()` could be used to avoid keeping all
    // this data in the local DB. I think the Http Client supports some kind of
    // cache...

    user.id = discord_user.discord_user_id;
    user.bot = discord_user.bot;
    user.discriminator = discord_user.discriminator;
    user.name = discord_user.name;
    user.avatar = discord_user.avatar;

    user
  }
}

impl Queryable<users::SqlType, Pg> for DiscordUser {
  type Row = (i32, bool, i32, String, i32);

  fn build((id, bot, discriminator, name, discord_user_id): Self::Row) -> Self {
    DiscordUser {
      user_id: id,
      bot,
      name,
      discriminator: discriminator as u16,
      avatar: None,
      discord_user_id: UserId(discord_user_id as u64),
    }
  }
}

impl From<DiscordUser> for UserRating {
  fn from(record: DiscordUser) -> UserRating {
    UserRating {
      id: None,
      user_id: record.user_id,
      rating: None,
      deviation: None,
      volatility: None,
      game_mode_id: 0,
    }
  }
}
