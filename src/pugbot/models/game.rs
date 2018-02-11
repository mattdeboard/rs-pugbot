use models::draft_pool::DraftPool;
use models::team::Team;
use team_count;
use rand::{ Rng, thread_rng };
use serenity::model::user::User;
use std::ops::Range;
use traits::has_members::HasMembers;
use traits::phased::Phased;
use typemap::Key;
use std::collections::HashMap;

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

  pub fn select_captains(&mut self) -> Result<(), &'static str> {
    if self.phase != Some(Phases::CaptainSelection) {
      return Err("We aren't picking captains, yet!");
    }

    let mut rng = thread_rng();
    let pool = self.draft_pool.available_players.clone();
    let tc = team_count().unwrap();
    let teams: Vec<Team> = (Range { start: 1, end: tc + 1 })
      .map(
        |i| {
          let keys: Vec<&usize> = pool.keys().collect();
          let random_key: &usize = rng.choose(&[keys]).unwrap().first().unwrap();

          if let Some(user) = self.draft_pool.pop_available_player(random_key) {
            Some(Team {
              id: (i as usize),
              captain: Some(user.clone()),
              members: vec![user]
            })
          } else {
            None
          }
        }
      )
      .filter(|t| t.is_some())
      .map(|t| t.unwrap())
      .collect();

    self.teams = Some(teams.clone());

    if self.phase == Some(Phases::PlayerRegistration) {
      self.next_phase();
    }
    Ok(())
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
