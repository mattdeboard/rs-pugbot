use models::team::Team;
use traits::pool_availability::PoolAvailability;
use traits::thread_safe_phased::ThreadSafePhased;
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use typemap::Key;

#[allow(dead_code)]
pub struct Game<T: PoolAvailability> {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: T,
  phase_map: HashMap<i32, Phase>,
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

fn phase_map() -> HashMap<i32, Phase> {
  let mut phase_map = HashMap::new();
  phase_map.insert(Phase::PlayerRegistration as i32, Phase::PlayerRegistration);
  phase_map.insert(Phase::CaptainSelection as i32, Phase::CaptainSelection);
  phase_map.insert(Phase::PlayerDrafting as i32, Phase::PlayerDrafting);
  phase_map.insert(Phase::MapSelection as i32, Phase::MapSelection);
  phase_map.insert(Phase::ResultRecording as i32, Phase::ResultRecording);
  phase_map
}

impl<T> ThreadSafePhased for Game<T> where T: PoolAvailability {
  fn forward_phase(&self) {
    let phase_clone = self.phase.clone();
    // match self.phase_map.get(*phase_clone.lock().unwrap() as i32) {
    //   Some(phase) => {
    //     *self.phase.lock().unwrap() = phase;
    //   },
    //   None => {
    //     *self.phase.lock().unwrap() = *phase_clone.
    // *phase_clone.lock().unwrap() = current_phase;
    // let phase_key: Phase = *self.phase.lock().unwrap();
    // phase_key;
  }

  fn backward_phase(&self) {}
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
