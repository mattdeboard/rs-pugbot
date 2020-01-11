use crate::schema::*;
use crate::models::game_title::GameTitle;

#[primary_key(game_title_id, map_name)]
#[table_name = "maps"]
#[belongs_to(GameTitle)]
#[derive(Clone, Debug, Identifiable, Insertable, Queryable, Associations, Deserialize)]
pub struct Map {
  pub game_title_id: i32,
  pub map_name: String,
}
