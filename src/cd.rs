use macroquad::{prelude::*};


#[derive(Debug, Clone)]
pub struct BoundRect {
  position: Vec2,
  w: f32,
  h: f32
}

impl BoundRect {
  pub fn new(position: Vec2, w: f32, h: f32) -> Self {
    Self { position, w, h, }
  }

  pub fn update_position(&mut self, position: &Vec2) {
    self.position.x = position.x;
    self.position.y = position.y;
  }

  pub fn get_rect(&self) -> Rect {
    Rect::new(self.position.x - self.w / 2., self.position.y - self.h / 2., self.w, self.h)
  }

  pub fn collide_with(&self, other: &BoundRect) -> bool {
    self.get_rect().intersect(other.get_rect()).is_some()
  }
}