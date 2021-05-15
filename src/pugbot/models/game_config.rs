use crate::models::game_title::GameTitle;
use crate::schema::*;

#[derive(
  Clone, Debug, Identifiable, Insertable, Queryable, Associations, Deserialize,
)]
#[primary_key(game_config_id)]
#[table_name = "game_configs"]
#[belongs_to(GameTitle, foreign_key = "game_title_id")]
pub struct GameConfig {
  game_config_id: i32,
  game_title_id: i32,
  team_count: u32,
  team_size: u32,
}
