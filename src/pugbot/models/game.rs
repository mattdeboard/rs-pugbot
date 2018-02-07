use ::models::team::Team;
use ::models::draft_pool::DraftPool;
use typemap::Key;

pub struct Game {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: DraftPool
}

impl Game {}

impl Key for Game {
  type Value = Game;
}
