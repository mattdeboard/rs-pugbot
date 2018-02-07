use models::team::Team;
use team_count;
use rand::{ Rng, thread_rng };
use serenity::model::user::User;
use std::ops::Range;
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
  pub fn select_captains(&mut self) {
    let mut rng = thread_rng();
    let pool = self.draft_pool.members();

    if let Some(tc) = team_count() {
      let teams: Vec<Team> = Range { start: 1, end: tc + 1 }.map(|i| {
        let user: &User = rng.choose(&pool).unwrap();
        Team {
          id: (i as usize),
          captain: Some(user.clone()),
          members: Vec::new()
        }
      }).collect();
      self.teams = Some(teams.clone());
    }
  }
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
