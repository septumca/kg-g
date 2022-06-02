use macroquad::{prelude::*};

pub struct HpModification {
  source: usize,
  target: usize,
  amount: isize,
}

impl HpModification {
  pub fn new(source: usize, target: usize, amount: isize) -> Self {
    Self {
      source,
      target,
      amount,
    }
  }

  pub fn is_of_source(&self, source: usize) -> bool {
    self.source == source
  }
}
