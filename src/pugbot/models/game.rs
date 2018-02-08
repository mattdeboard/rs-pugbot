use ::models::team::Team;
use ::traits::pool_availability::PoolAvailability;
use std::collections::BTreeMap;
use std::sync::{ Arc, Mutex };
use typemap::Key;

#[allow(dead_code)]
pub struct Game<T: PoolAvailability> {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: T,
  phase_map: BTreeMap<i32, Phase>,
  phase: Arc<Mutex<Phase>>
}

enum Phase {
  PlayerRegistration,
  CaptainSelection,
  PlayerDrafting,
  MapSelection,
  ResultRecording
}

impl<T> Game<T> where T: PoolAvailability {
  pub fn new(teams: Option<Vec<Team>>, draft_pool: T) -> Game<T>{
    Game {
      teams: teams,
      draft_pool: draft_pool,
      phase_map: phase_map(),
      phase: Arc::new(Mutex::new(Phase::PlayerRegistration))
    }
  }
}

fn phase_map() -> BTreeMap<i32, Phase> {
  let mut phase_map = BTreeMap::new();
  phase_map.insert(Phase::PlayerRegistration as i32, Phase::PlayerRegistration);
  phase_map.insert(Phase::CaptainSelection as i32, Phase::CaptainSelection);
  phase_map.insert(Phase::PlayerDrafting as i32, Phase::PlayerDrafting);
  phase_map.insert(Phase::MapSelection as i32, Phase::MapSelection);
  phase_map.insert(Phase::ResultRecording as i32, Phase::ResultRecording);
  phase_map
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
