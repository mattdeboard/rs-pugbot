use ::models::team::Team;
use ::traits::pool_availability::PoolAvailability;
use typemap::Key;

enum Phase {
  PlayerRegistration,
  CaptainSelection,
  PlayerDrafting,
  MapSelection,
  ResultRecording
}

pub struct Game<T: PoolAvailability> {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: T,
  phase: Phase
}

impl<T> Game<T> where T: PoolAvailability {
  fn new(teams: Option<Vec<Team>>, draft_pool: T) -> Game<T>{
    Game {
      teams: teams,
      draft_pool: draft_pool,
      phase: Phase::PlayerRegistration
    }
  }
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
