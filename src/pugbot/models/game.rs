use models::draft_pool::DraftPool;
use models::team::Team;
use team_count;
use rand::{ Rng, thread_rng };
use serenity::model::user::User;
use std::ops::Range;
use traits::has_members::HasMembers;
use traits::phased::Phased;
use typemap::Key;

pub struct Game {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: DraftPool,
  pub phase: Option<Phases>,
}

#[derive(PartialEq, Debug)]
pub enum Phases {
  PlayerRegistration,
  CaptainSelection,
  PlayerDrafting,
  MapSelection,
  ResultRecording
}

impl Game {
  pub fn new(teams: Option<Vec<Team>>, draft_pool: DraftPool) -> Game {
    Game {
      teams: teams,
      draft_pool: draft_pool,
      phase: Some(Phases::PlayerRegistration),
    }
  }

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

impl Phased for Game {
  fn next_phase(&mut self ) {
    {
      self.draft_pool.generate_available_players();
    }
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

impl Key for Game {
  type Value = Game;
}
