use diesel::ExpressionMethods;
use diesel::dsl::Eq;
use diesel::pg::Pg;
use diesel::prelude::{ Insertable, Queryable };
use schema::*;

#[primary_key(game_title_id, map_name)]
#[table_name="maps"]
#[belongs_to(game_titles)]
#[derive(Debug, Associations)]
pub struct Map {
  pub game_title_id: i32,
  pub map_name: String
}

impl<'a> Insertable<maps::table> for &'a Map {
  type Values = <(
    Eq<maps::game_title_id, i32>,
    Eq<maps::map_name, &'a String>,
  ) as Insertable<maps::table>>::Values;

  fn values(self) -> Self::Values {
    (
      maps::game_title_id.eq(self.game_title_id),
      maps::map_name.eq(&self.map_name),
    ).values()
  }
}

impl Queryable<maps::SqlType, Pg> for Map {
  type Row = (i32, String);

  fn build((game_title_id, map_name): Self::Row) -> Self {
    Map {
      game_title_id,
      map_name
    }
  }
}
