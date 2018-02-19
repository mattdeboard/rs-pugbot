use schema::*;

#[primary_key(game_mode_id)]
#[table_name="game_modes"]
#[belongs_to(game_titles)]
#[derive(Debug, Insertable, Queryable, Associations)]
pub struct GameMode {
  pub game_mode_id: Option<i32>,
  pub game_title_id: i32,
  pub mode_name: String,
  pub team_size: i32
}
