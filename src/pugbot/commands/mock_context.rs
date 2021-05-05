#[cfg(test)]
pub mod tests {
  use futures::channel::mpsc::unbounded;
  use serenity::cache::Cache;
  use serenity::http::Http;
  use serenity::prelude::{RwLock, TypeMap};
  use serenity::{self, client::bridge::gateway::ShardMessenger};
  use serenity::{gateway::InterMessage, prelude::Context};
  use std::sync::Arc;

  pub fn mock_context() -> Context {
    let (sender, _) = unbounded::<InterMessage>();
    Context {
      cache: Arc::new(Cache::new()),
      http: Arc::new(Http::new_with_token("abc123")),
      shard: ShardMessenger { tx: sender },
      shard_id: 1,
      data: Arc::new(RwLock::new(TypeMap::new())),
    }
  }
}
