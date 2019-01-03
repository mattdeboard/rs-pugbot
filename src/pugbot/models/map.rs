use crate::schema::*;

#[primary_key(game_title_id, map_name)]
#[table_name = "maps"]
#[belongs_to(game_titles)]
#[derive(Clone, Debug, Insertable, Queryable, Associations, Deserialize)]
pub struct Map {
  pub game_title_id: i32,
  pub map_name: String,
}
