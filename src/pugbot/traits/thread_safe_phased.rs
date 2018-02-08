pub trait ThreadSafePhased {
  fn forward_phase(&self);
  fn backward_phase(&self);
}
