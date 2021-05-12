use crate::models::game_title::GameTitle;
use crate::schema::*;

#[derive(
  Clone, Debug, Identifiable, Insertable, Queryable, Associations, Deserialize,
)]
#[primary_key(game_title_id, map_name)]
#[table_name = "maps"]
#[belongs_to(GameTitle, foreign_key = "game_title_id")]
pub struct Map {
  pub game_title_id: i32,
  pub map_name: String,
}
