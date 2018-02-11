pub trait Phased {
  fn next_phase(&mut self);
  fn previous_phase(&mut self);
  fn reset_phase(&mut self);
}
