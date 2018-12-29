use models::draft_pool::DraftPool;
use models::map::Map as GameMap;
use models::team::Team;
use rand::{thread_rng, Rng};
use serenity::model::channel::Embed;
use serenity::model::id::UserId;
use serenity::utils::Colour;
use std::collections::HashMap;
use std::iter::Cycle;
use std::ops::Range;
use team_id_range;
use traits::has_members::HasMembers;
use traits::phased::Phased;
use typemap::Key;

pub struct Game {
  pub active_map: Option<GameMap>,
  pub eligible_voter_ids: Vec<UserId>,
  pub map_choices: Vec<GameMap>,
  pub map_votes: HashMap<i32, i32>,
  pub draft_pool: DraftPool,
  pub game_mode_id: i32,
  pub phase: Option<Phases>,
  pub team_count: u32,
  pub team_size: u32,
  pub teams: Option<Vec<Team>>,
  pub turn_number: usize,
  pub turn_taker: Cycle<Range<usize>>,
}

#[derive(PartialEq, Debug)]
pub enum Phases {
  PlayerRegistration,
  CaptainSelection,
  PlayerDrafting,
  MapSelection,
  ResultRecording,
}

#[derive(PartialEq)]
pub enum Outcome {
  Win,
  Loss,
  Draw,
}

impl Game {
  pub fn new(
    teams: Option<Vec<Team>>,
    draft_pool: DraftPool,
    mode_id: i32,
    map_choices: Vec<GameMap>,
    team_count: u32,
    team_size: u32,
  ) -> Game {
    let members = draft_pool.members.clone();

    Game {
      active_map: None,
      draft_pool: draft_pool,
      eligible_voter_ids: members.iter().map(|m| m.id).collect(),
      game_mode_id: mode_id,
      map_choices: map_choices,
      map_votes: [(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]
        .iter()
        .cloned()
        .collect(),
      phase: Some(Phases::PlayerRegistration),
      teams: teams,
      team_count: team_count,
      team_size: team_size,
      turn_number: 1,
      turn_taker: team_id_range().cycle(),
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
      None => None,
    }
  }

  pub fn select_captains(&mut self) -> Result<(), &'static str> {
    if self.phase != Some(Phases::CaptainSelection) {
      return Err("We aren't picking captains, yet!");
    }

    let mut rng = thread_rng();
    let teams: Vec<Team> = team_id_range()
      .map(|i| {
        let pool = self.draft_pool.available_players.clone();
        let keys: Vec<&usize> = pool.keys().collect();
        let random_key: &usize = rng.choose(&[keys]).unwrap().first().unwrap();

        if let Some(user) = self.draft_pool.pop_available_player(random_key) {
          Some(Team {
            id: i,
            captain: Some(user.clone()),
            members: vec![user],
            glicko2_ratings: vec![],
          })
        } else {
          None
        }
      })
      .filter(|t| t.is_some())
      .map(|t| t.unwrap())
      .collect();

    self.teams = Some(teams.clone());
    self.next_phase();
    Ok(())
  }

  pub fn drafting_complete_embed(
    &mut self,
    r: u8,
    g: u8,
    b: u8,
  ) -> Option<Embed> {
    let roster: Vec<String> = self
      .teams
      .clone()
      .unwrap()
      .iter()
      .map(|team| {
        let member_names: Vec<String> =
          team.members.iter().map(|user| user.clone().name).collect();
        format!("Team {} roster:\n{}", team.id, member_names.join("\n"))
      })
      .collect();

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
      video: None,
    })
  }

  pub fn register_vote(&mut self, user_id: UserId) {
    self.eligible_voter_ids.retain(|&id| id != user_id);
  }

  pub fn map_winner_embed(&self, r: u8, g: u8, b: u8) -> Option<Embed> {
    let map_name = &self.active_map;
    Some(Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(format!("The winning map is {:?}!", map_name)),
      footer: None,
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some(format!("The winning map is {:?}!", map_name)),
      url: None,
      video: None,
    })
  }

  pub fn map_selection_embed(&self, r: u8, g: u8, b: u8) -> Option<Embed> {
    let maps: Vec<String> = self.map_choices.iter().enumerate().fold(
      vec![String::from(
        "Typing `~mv <#>` will register your map vote (You must be on a team to vote)",
      )],
      |mut acc, (index, map)| {
        acc.push(format!("{} -> {}", index + 1, map.map_name));
        acc
      },
    );
    Some(Embed {
      author: None,
      colour: Colour::from_rgb(r, g, b),
      description: Some(maps.join("\n")),
      footer: None,
      fields: Vec::new(),
      image: None,
      kind: "rich".to_string(),
      provider: None,
      thumbnail: None,
      timestamp: None,
      title: Some("Time to pick a map!".to_string()),
      url: None,
      video: None,
    })
  }
}

impl Phased for Game {
  fn next_phase(&mut self) {
    self.phase = match self.phase {
      Some(Phases::PlayerRegistration) => {
        // If the draft pool is full, move to the next phase.
        // Draft pool is full if the number of users in the pool equals the max configured size of
        // the pool. Max size of the draft pool is expressed as `team_count * team_size`.

        // If the draft pool is NOT full, do not advance to the next phase. "Not advancing to the
        // next phase" is equivalent to returning `Phases::PlayerRegistration` as the phase.
        if self.draft_pool.members().len() as u32
          == self.team_count * self.team_size
        {
          self.draft_pool.generate_available_players();
          // Reset draft pool membership to an empty Vec. The pool of players available for drafting
          //  (`available_players`) is distinct from the pool of registered players (`members`).
          self.draft_pool.members = Vec::new();
          Some(Phases::CaptainSelection)
        } else {
          Some(Phases::PlayerRegistration)
        }
      }
      Some(Phases::CaptainSelection) => Some(Phases::PlayerDrafting),
      Some(Phases::PlayerDrafting) => {
        self.turn_number = 1;
        self.turn_taker = team_id_range().cycle();
        Some(Phases::MapSelection)
      }
      Some(Phases::MapSelection) => {
        let mut winning_map_index: i32 = 0;
        let mut winning_vote_amount: i32 = 0;
        for (key, val) in self.map_votes.iter() {
          if *val > winning_vote_amount {
            winning_map_index = *key;
            winning_vote_amount = *val;
          }
        }
        let choice = &self.map_choices[winning_map_index as usize - 1];
        self.active_map = Some(GameMap {
          game_title_id: choice.game_title_id,
          map_name: choice.map_name.clone(),
        });
        Some(Phases::ResultRecording)
      }
      _ => None,
    };
  }

  fn previous_phase(&mut self) {
    self.phase = match self.phase {
      Some(Phases::CaptainSelection) => Some(Phases::PlayerRegistration),
      Some(Phases::PlayerDrafting) => Some(Phases::CaptainSelection),
      Some(Phases::MapSelection) => Some(Phases::PlayerDrafting),
      Some(Phases::ResultRecording) => Some(Phases::MapSelection),
      _ => None,
    };
  }

  fn reset_phase(&mut self) {
    self.phase = Some(Phases::PlayerRegistration);
  }
}

impl Key for Game {
  type Value = Game;
}

#[cfg(test)]
mod tests {
  extern crate kankyo;
  extern crate serde;
  extern crate serde_json;
  extern crate serenity;

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use commands;
  use models::draft_pool::DraftPool;
  use models::game::Phased;
  use models::game::{Game, Phases};
  use serenity::model::channel::Message;
  use serenity::model::id::UserId;
  use serenity::model::user::User;
  use std::fs::File;

  macro_rules! p {
    ($s:ident, $filename:expr) => {{
      let f =
        File::open(concat!("./tests/resources/", $filename, ".json")).unwrap();
      let v = serde_json::from_reader::<File, Value>(f).unwrap();

      $s::deserialize(v).unwrap()
    }};
  }

  fn gen_test_user() -> User {
    User {
      id: UserId(210),
      avatar: Some("abc".to_string()),
      bot: false,
      discriminator: 1432,
      name: "TestUser".to_string(),
    }
  }

  #[test]
  fn test_game_next_phase_empty_queue() {
    // Test what should happen when next_phase is called in PlayerRegistration phase and there is
    // still room in the queue.
    let game =
      &mut Game::new(None, DraftPool::new(vec![]), 1, Vec::new(), 2, 6);
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    game.next_phase();
    // Invoking next_phase should just keep returning PlayerRegistration since there is still
    // room in the queue.
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
  }

  #[test]
  fn test_game_next_phase_full_queue() {
    // Test what should happen when next_phase is called in PlayerRegistration phase and the queue
    // is full.
    let game =
      &mut Game::new(None, DraftPool::new(vec![]), 1, Vec::new(), 0, 0);
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    game.next_phase();
    // Invoking next_phase should return CaptainSelection since the draft pool/queue has filled
    assert_eq!(game.phase, Some(Phases::CaptainSelection));
  }

  #[test]
  fn test_select_captains() {
    let message = p!(Message, "message");
    let game = &mut Game::new(
      None,
      DraftPool::new(vec![gen_test_user()]),
      1,
      Vec::new(),
      // Draft pool max size: 2 (1 * 2)
      1,
      2,
    );
    // game.draft_pool.add_member(message.author);
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    // Invoking update_members invoke the `next_phase` call, which should advance the phase.
    commands::add::update_members(game, &message, false);
    assert_eq!(game.phase, Some(Phases::CaptainSelection));
    // Advancing to `CaptainSelection` should build the available_players HashMap.
    assert_eq!(game.draft_pool.available_players.len(), 2);
    assert_eq!(game.select_captains(), Ok(()));
    // Selecting captains successfully should consume all the entries in available_players
    assert_eq!(game.draft_pool.available_players.len(), 0);
    // There should now be two teams built.
    assert_eq!(game.teams.clone().unwrap().len(), 2);
  }
}
