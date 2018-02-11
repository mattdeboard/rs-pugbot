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
  /// Implements thread-safe state changes to `Game.phase`.
  ///
  /// An example of the usage here is, when enough players have enrolled in the
  /// draft pool to create two full teams, we want to move from the
  /// `PlayerRegistration` phase to `CaptainSelection`. This will provide other
  /// parts of the code a way to explicitly check how to respond appropriately
  /// to the `add` command.
  ///
  /// In order to keep the code that signals a phase change should occur
  /// decoupled from the implementation for the phase-changing logic, only the
  /// 0-argument `next_phase` and `previous_phase` methods are exposed to
  /// callers.
  fn next_phase(&self) {
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

  fn previous_phase(&self) {}
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
