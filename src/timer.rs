use macroquad::prelude::*;


#[derive(Debug, Clone)]
pub struct Timer {
  act: f32,
  threshold: f32,
}

impl Timer {
  pub fn new(threshold: f32) -> Self {
    Self { act: 0., threshold }
  }

  pub fn update(&mut self, delta: f32) -> bool {
    let updated_time = self.act + delta;
    let over_threshold = updated_time > self.threshold;

    self.act = if over_threshold { 0. } else { updated_time };
    over_threshold
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create() -> Timer {
    Timer {
      act: 0.,
      threshold: 0.5,
    }
  }

  #[test]
  fn update() {
    let mut time = create();
    time.update(0.1);
    assert_eq!(time.act, 0.1);
    time.update(0.1);
    assert_eq!(time.act, 0.2);
    time.update(0.1);
    assert_eq!(time.act, 0.3);
  }

  #[test]
  fn update_over() {
    let mut time = create();
    time.update(0.4);
    assert_eq!(time.act, 0.4);
    time.update(0.1);
    assert_eq!(time.act, 0.5);
    time.update(0.1);
    assert_eq!(time.act, 0.);
  }
}
