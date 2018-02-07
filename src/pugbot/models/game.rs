use ::models::team::Team;
use ::traits::pool_availability::PoolAvailability;
use typemap::Key;

pub struct Game<T: PoolAvailability> {
  pub teams: Option<Vec<Team>>,
  pub draft_pool: T
}

impl<T> Key for Game<T> where T: 'static + PoolAvailability {
  type Value = Game<T>;
}
