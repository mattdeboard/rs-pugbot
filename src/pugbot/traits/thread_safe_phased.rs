pub trait ThreadSafePhase {
  fn forward_phase(&self);
  fn backward_phase(&self);
}
