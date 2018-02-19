use models::draft_pool::DraftPool;
use models::team::Team;
use rand::{ Rng, thread_rng };
use serenity::model::channel::{ Embed };
use serenity::utils::Colour;
use std::iter::Cycle;
use std::ops::Range;
use traits::phased::Phased;
use typemap::Key;
use team_id_range;

pub struct Game {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: DraftPool,
  pub phase: Option<Phases>,
  pub turn_taker: Cycle<Range<usize>>,
  pub turn_number: usize,
  pub game_mode_id: i32
}

#[derive(PartialEq, Debug)]
pub enum Phases {
  PlayerRegistration,
  CaptainSelection,
  PlayerDrafting,
  MapSelection,
  ResultRecording
}

#[derive(PartialEq)]
pub enum Outcome {
  Win,
  Loss,
  Draw
}

impl Game {
  pub fn new(teams: Option<Vec<Team>>, draft_pool: DraftPool, mode_id: i32) -> Game {
    Game {
      teams: teams,
      draft_pool: draft_pool,
      phase: Some(Phases::PlayerRegistration),
      turn_taker: team_id_range().cycle(),
      turn_number: 1,
      game_mode_id: mode_id
    }
  }

  pub fn next_team_to_draft(&mut self) -> Team {
    let next_team = self.turn_taker.next().unwrap();
    {
      self.team_by_id(next_team).unwrap()
    }
  }

  pub fn team_by_id(&self, id: usize) -> Option<Team> {
    match self.teams {
      Some(ref teams) => teams.iter().find(|t| t.id == id).cloned(),
      None => None
    }
  }

  pub fn select_captains(&mut self) -> Result<(), &'static str> {
    if self.phase != Some(Phases::CaptainSelection) {
      return Err("We aren't picking captains, yet!");
    }

    let mut rng = thread_rng();
    let teams: Vec<Team> = team_id_range()
      .map(
        |i| {
          let pool = self.draft_pool.available_players.clone();
          let keys: Vec<&usize> = pool.keys().collect();
          let random_key: &usize = rng.choose(&[keys]).unwrap().first().unwrap();

          if let Some(user) = self.draft_pool.pop_available_player(random_key) {
            Some(Team {
              id: i,
              captain: Some(user.clone()),
              members: vec![user],
              glicko2_ratings: vec![]
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
    self.next_phase();
    Ok(())
  }

  pub fn drafting_complete_embed(&mut self, r: u8, g: u8, b: u8) -> Option<Embed> {
    let roster: Vec<String> = self.teams.clone().unwrap().iter().map(|team| {
      let member_names: Vec<String> = team.members.iter().map(|user| user.clone().name).collect();
      format!("Team {} roster:\n{}", team.id, member_names.join("\n"))
    }).collect();

    if self.phase == Some(Phases::PlayerDrafting) {
      self.next_phase();
    }

    Some(Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(roster.join("\n---\n")),
      footer: None,
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some(String::from("Drafting has been completed!")),
      url: None,
      video: None
    })

  }
}

impl Phased for Game {
  fn next_phase(&mut self ) {
    self.phase = match self.phase {
      Some(Phases::PlayerRegistration) => {
        self.draft_pool.generate_available_players();
        self.draft_pool.members = Vec::new();
        Some(Phases::CaptainSelection)
      },
      Some(Phases::CaptainSelection) => Some(Phases::PlayerDrafting),
      Some(Phases::PlayerDrafting) => {
        self.turn_number = 1;
        self.turn_taker = team_id_range().cycle();
        Some(Phases::MapSelection)
      },
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
