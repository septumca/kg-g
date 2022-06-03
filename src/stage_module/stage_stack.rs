use std::collections::HashMap;

use super::resources::{Resources};

pub trait Stage {
  fn get_id(&self) -> usize;
  fn update(&mut self, resources: &Resources) -> Option<usize>;
  fn draw(&self, resources: &Resources);
}

pub struct StageStack {
  pub stack: HashMap<usize, Box<dyn Stage>>,
  pub active_stage_id: usize,
}

impl StageStack {
  pub fn new() -> Self {
    Self {
      stack: HashMap::new(),
      active_stage_id: 0,
    }
  }

  pub fn with_stages(mut self, stages: Vec<Box<dyn Stage>>) -> Self {
    for s in stages {
      if self.stack.is_empty() {
        self.active_stage_id = s.get_id();
      }
      self.stack.insert(s.get_id(), s);
    }

    self
  }

  pub fn activate_stage(&mut self, stage_id: usize) {
    self.active_stage_id = stage_id;
  }

  pub fn update(&mut self, resources: &Resources) {
    if let Some(stage) = self.stack.get_mut(&self.active_stage_id) {
      if let Some(new_stage_id) = stage.update(resources) {
        self.activate_stage(new_stage_id);
       }
    }
  }

  pub fn draw(&mut self, resources: &Resources) {
    if let Some(stage) = self.stack.get_mut(&self.active_stage_id) {
      stage.draw(resources);
    }
  }
}