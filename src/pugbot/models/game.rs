use models::team::Team;
use traits::pool_availability::PoolAvailability;
use traits::phased::Phased;
use typemap::Key;

pub struct Game<T: PoolAvailability> {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: T,
  phase: Option<Phases>,
}

pub enum Phases {
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
      phase: Some(Phases::PlayerRegistration),
    }
  }

  pub fn current_phase(&self) -> &Phases {
    self.phase.as_ref().unwrap()
  }
}

impl<T> Phased for Game<T> where T: PoolAvailability {
  fn next_phase(&mut self ) {
    self.phase = match self.phase {
      Some(Phases::PlayerRegistration) => Some(Phases::CaptainSelection),
      Some(Phases::CaptainSelection) => Some(Phases::PlayerDrafting),
      Some(Phases::PlayerDrafting) => Some(Phases::MapSelection),
      Some(Phases::MapSelection) => Some(Phases:: ResultRecording),
      _ => None
    };
  }

  fn previous_phase(&mut self) {
    self.phase = match self.phase {
      Some(Phases::CaptainSelection) => Some(Phases::PlayerRegistration),
      Some(Phases::PlayerDrafting) => Some(Phases::CaptainSelection),
      Some(Phases::MapSelection) => Some(Phases::PlayerDrafting),
      Some(Phases::ResultRecording) => Some(Phases::MapSelection),
      _ => None
    };
  }
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
