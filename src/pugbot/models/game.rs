use models::team::Team;
use rand::{thread_rng, Rng};
use serenity::model::user::User;
use traits::pool_availability::PoolAvailability;
use traits::phased::Phased;
use typemap::Key;

pub struct Game<T: PoolAvailability> {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: T,
  pub phase: Option<Phases>,
}

#[derive(PartialEq)]
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

  fn reset_phase(&mut self) {
    self.phase = Some(Phases::PlayerRegistration);
  }
}

impl<T> Game<T> where T: PoolAvailability {
  pub fn select_captains(&mut self) -> Vec<User> {
    let mut rng = thread_rng();
    let pool = self.draft_pool.members().clone();
    let teams: Vec<Team> = [1, 2].into_iter().map(|i| {
      let user: &User = rng.choose(&pool).unwrap();
      Team {
        id: *i,
        captain: Some(user.clone()),
        members: Vec::new()
      }
    }).collect();
    self.teams = Some(teams.clone());
    teams.clone().into_iter().map(|team| team.captain.unwrap()).collect()
  }
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
