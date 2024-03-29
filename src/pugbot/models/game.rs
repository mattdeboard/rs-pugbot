use crate::models::draft_pool::DraftPool;
use crate::models::map::Map as GameMap;
use crate::models::team::Team;
use crate::team_id_range;
use crate::traits::has_members::HasMembers;
use crate::traits::phased::Phased;
use rand::{prelude::SliceRandom, thread_rng};
use serenity::model::id::UserId;
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::iter::Cycle;
use std::ops::Range;
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
  pub teams: Vec<Team>,
  pub turn_number: usize,
  pub turn_taker: Cycle<Range<u32>>,
}

pub struct GameContainer;

impl TypeMapKey for GameContainer {
  type Value = Game;
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
    teams: Vec<Team>,
    draft_pool: DraftPool,
    mode_id: i32,
    map_choices: Vec<GameMap>,
    team_count: u32,
    team_size: u32,
  ) -> Game {
    let members = draft_pool.members.clone();
    let mut map_votes: HashMap<i32, i32> = HashMap::new();

    for i in 1..(map_choices.len() + 1) {
      map_votes.insert(i as i32, 0);
    }

    Game {
      active_map: None,
      draft_pool: draft_pool,
      eligible_voter_ids: members.iter().map(|m| m.id).collect(),
      game_mode_id: mode_id,
      map_choices: map_choices,
      map_votes,
      phase: Some(Phases::PlayerRegistration),
      teams: teams,
      team_count,
      team_size,
      turn_number: 1,
      turn_taker: (0..team_count).cycle(),
    }
  }

  pub fn select_captains(&mut self) -> Result<(), &'static str> {
    if self.phase != Some(Phases::CaptainSelection) {
      return Err("We aren't picking captains, yet!");
    }

    let mut rng = thread_rng();
    let teams: Vec<Team> = team_id_range(self.team_count)
      .map(|i| {
        let pool = self.draft_pool.available_players.clone();
        let keys: Vec<&usize> = pool.keys().collect();
        let random_key: &usize = keys.choose(&mut rng).unwrap();

        if let Some(user) = self.draft_pool.pop_available_player(random_key) {
          Some(Team {
            id: i,
            captain: Some(user.clone()),
            members: vec![user],
            // glicko2_ratings: vec![],
          })
        } else {
          None
        }
      })
      .filter(|t| t.is_some())
      .map(|t| t.unwrap())
      .collect();

    self.teams = teams;
    self.next_phase();

    if self.teams.len() as u32 == self.team_count {
      return Ok(());
    }
    Err("Team creation failed unexpectedly")
  }

  pub fn roster(&self) -> Vec<String> {
    self
      .teams
      .clone()
      .iter()
      .map(|team| {
        let member_names: Vec<String> =
          team.members.iter().map(|user| user.clone().name).collect();
        format!("Team {} roster:\n{}", team.id, member_names.join("\n"))
      })
      .collect()
  }

  pub fn register_vote(&mut self, user_id: UserId) {
    self.eligible_voter_ids.retain(|&id| id != user_id);
  }
}

impl Phased for Game {
  fn next_phase(&mut self) {
    self.phase = match self.phase {
      None => Some(Phases::PlayerRegistration),
      Some(Phases::PlayerRegistration) => {
        // If the draft pool is full, move to the next phase.
        // Draft pool is full if the number of users in the pool equals the max
        // configured size of the pool. Max size of the draft pool is
        // expressed as `team_count * team_size`.

        // If the draft pool is NOT full, do not advance to the next phase. "Not
        // advancing to the next phase" is equivalent to returning
        // `Phases::PlayerRegistration` as the phase.
        if self.draft_pool.members().len() as u32
          == self.team_count * self.team_size
        {
          self.draft_pool.generate_available_players();
          // Reset draft pool membership to an empty Vec. The pool of players
          // available for drafting  (`available_players`) is distinct
          // from the pool of registered players (`members`).
          self.draft_pool.members = Vec::new();
          Some(Phases::CaptainSelection)
        } else {
          Some(Phases::PlayerRegistration)
        }
      }
      Some(Phases::CaptainSelection) => Some(Phases::PlayerDrafting),
      Some(Phases::PlayerDrafting) => {
        if self.draft_pool.available_players.len() == 0 {
          self.turn_number = 1;
          self.turn_taker = (0..self.team_count).cycle();
          Some(Phases::MapSelection)
        } else {
          self.turn_number += 1;
          Some(Phases::PlayerDrafting)
        }
      }
      Some(Phases::MapSelection) => {
        let vote_counts: i32 = self
          .map_votes
          .values()
          .clone()
          .fold(0, |acc, val| acc + *val);
        if vote_counts < self.draft_pool.max_members as i32 {
          Some(Phases::MapSelection)
        } else {
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
      }
      Some(Phases::ResultRecording) => None,
    };
  }

  fn previous_phase(&mut self) {
    self.phase = match self.phase {
      None => Some(Phases::ResultRecording),
      Some(Phases::CaptainSelection) => Some(Phases::PlayerRegistration),
      Some(Phases::PlayerDrafting) => Some(Phases::CaptainSelection),
      Some(Phases::MapSelection) => Some(Phases::PlayerDrafting),
      Some(Phases::ResultRecording) => Some(Phases::MapSelection),
      Some(Phases::PlayerRegistration) => None,
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
#[allow(unused_must_use)]
pub mod tests {
  use serde;
  use serde_json;
  use serenity::{self, client::Context};

  use self::serde::de::Deserialize;
  use self::serde_json::Value;
  use crate::models::game::{Game, Phased, Phases};
  use crate::models::{draft_pool::DraftPool, game::GameContainer};
  use crate::{commands, struct_from_json};
  use serenity::model::channel::Message;
  // use serenity::model::id::UserId;
  use serenity::model::user::User;
  use std::fs::File;

  async fn test_context(authors: Option<Vec<User>>) -> Box<Context> {
    let context = commands::mock_context::tests::mock_context();
    let users = match authors {
      Some(u) => u,
      None => vec![],
    };
    {
      let game = Game::new(
        vec![],
        DraftPool::new(users, 12),
        1,
        Vec::new(),
        // Draft pool max size: 12 (2 * 6)
        2,
        5,
      );
      let mut data = context.data.write().await;
      data.insert::<GameContainer>(game);
    }
    Box::new(context)
  }

  #[tokio::test]
  /// Test what should happen when next_phase is called in PlayerRegistration
  /// phase and there is still room in the queue.
  async fn test_game_next_phase_empty_queue() {
    let context = test_context(None).await;
    let mut data = context.data.write().await;
    if let Some(game) = data.get_mut::<GameContainer>() {
      assert_eq!(game.phase, Some(Phases::PlayerRegistration));
      game.next_phase();
      // Invoking next_phase should just keep returning PlayerRegistration since
      // there is still room in the queue.
      assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    }
  }

  #[test]
  /// Test what should happen when next_phase is called in PlayerRegistration
  /// phase and the queue is full.
  fn test_game_next_phase_full_queue() {
    let game =
      &mut Game::new(vec![], DraftPool::new(vec![], 0), 1, Vec::new(), 0, 0);
    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    game.next_phase();
    // Invoking next_phase should return CaptainSelection since the draft
    // pool/queue has filled
    assert_eq!(game.phase, Some(Phases::CaptainSelection));
  }

  #[tokio::test]
  async fn test_select_captains() {
    let authors: Vec<User> = struct_from_json!(Vec, "authors");
    let context = test_context(Some((&authors[1..]).to_vec())).await;
    let message = struct_from_json!(Message, "message");

    {
      let data = context.data.read().await;
      if let Some(game) = data.get::<GameContainer>() {
        assert_eq!(game.phase, Some(Phases::PlayerRegistration));
      }
    }
    // Invoking update_members invoke the `next_phase` call, which should
    // advance the phase.
    commands::add::update_members(&context, &message, false).await;

    {
      let mut data = context.data.write().await;
      if let Some(game) = data.get_mut::<GameContainer>() {
        assert_eq!(game.phase, Some(Phases::CaptainSelection));
        // This call should result in the available_players pool size
        // decreasing by `game.team_count`. This is because each team
        // needs a captain: The captains are randomly selected and drawn
        // from `game.draft_pool.available_players` pool.
        game.select_captains();
        assert_eq!(
          game.draft_pool.available_players.len(),
          (
            // Maximum pool size
            game.team_count * game.team_size -
            // One captain per team should be removed from the
            // `available_players` pool.
            game.team_count
          ) as usize
        );
        assert_eq!(game.teams.len(), game.team_count as usize);
      }
    }
  }

  #[test]
  fn test_team_creation() {
    let authors: Vec<User> = struct_from_json!(Vec, "authors");
    // Choosing 2 teams of 5 here since there are 10 authors in authors.json
    let (team_count, team_size) = (2, (authors.len() / 2) as u32);
    let game = &mut Game::new(
      vec![],
      DraftPool::new(authors, team_count * team_size),
      1,
      Vec::new(),
      team_count,
      team_size,
    );

    assert_eq!(game.phase, Some(Phases::PlayerRegistration));
    game.next_phase();
    assert_eq!(game.phase, Some(Phases::CaptainSelection));
    game.select_captains();

    assert_eq!(
      game.teams.len() as u32,
      game.team_count,
      "There were supposed to be {:?} teams but there are only {:?}",
      game.team_count,
      game.teams.len()
    );

    assert_eq!(game.phase, Some(Phases::PlayerDrafting));
  }
}
