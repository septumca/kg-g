use macroquad::{prelude::*};

#[derive(Debug, Clone, Copy)]
pub struct HpModification {
  source: usize,
  pub source_origin: usize,
  pub amount: isize,
}

impl HpModification {
  pub fn new(source: usize, source_origin: usize, amount: isize) -> Self {
    Self {
      source,
      source_origin,
      amount,
    }
  }

  pub fn is_of_source(&self, source: usize) -> bool {
    self.source == source
  }
}

#[derive(Debug, Clone)]
pub struct Hp {
  modifications: Vec<HpModification>,
  pub act_hp: isize,
}

impl Hp {
    pub fn new(initial_hp: isize) -> Self {
      Self {
        act_hp: initial_hp,
        modifications: vec![HpModification::new(0, 0, initial_hp)],
      }
    }

    pub fn is_alive(&self) -> bool {
      self.act_hp > 0
    }

    pub fn modify(&mut self, source: usize, source_origin: usize, amount: isize) {
      self.modifications.push(HpModification::new(source, source_origin, amount));
      self.act_hp += amount;
    }

    pub fn get_last_modification(&self) -> Option<HpModification> {
      self.modifications.last().and_then(|m| Some(*m))
    }

    pub fn has_been_modified_by_source(&self, source: usize) -> bool {
      self.modifications.iter().any(|hp_mod| hp_mod.is_of_source(source))
    }
}