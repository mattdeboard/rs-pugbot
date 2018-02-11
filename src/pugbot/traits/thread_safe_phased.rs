pub trait ThreadSafePhased {
  fn next_phase(&self);
  fn previous_phase(&self);
}
