use schema::*;

#[primary_key(game_title_id)]
#[table_name="game_titles"]
#[derive(Debug, Insertable, Queryable, Associations)]
pub struct GameTitle {
  pub game_title_id: Option<i32>,
  pub game_name: String
}
