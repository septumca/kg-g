
use macroquad::{prelude::*};

const EPSILON: f32 = 0.004;

#[derive(Debug, Clone)]
pub struct Movable {
  pub position: Vec2,
  pub target_position: Option<Vec2>,
  pub velocity: Vec2,
  pub impulses: Vec<Vec2>,
  pub fraction: f32,
  pub rotation: f32,
  speed: f32,
}

impl Movable {
  pub fn new(position: Vec2, speed: f32, fraction: f32) -> Self {
    Self {
      position,
      target_position: None,
      velocity: Vec2::ZERO,
      impulses: vec![],
      rotation: 0.,
      fraction,
      speed,
    }
  }

  pub fn with_velocity(mut self, velocity: Vec2) -> Self {
    self.velocity = velocity * self.speed;
    self
  }

  pub fn add_impuls(&mut self, implus: Vec2) {
    self.impulses.push(implus);
  }

  pub fn set_moving_to(&mut self, target_position: Vec2) {
    self.rotation = if self.position.x > target_position.x { 1. } else { 0. };
    self.velocity = (target_position - self.position).normalize() * self.speed;
    self.target_position = Some(target_position);
  }

  pub fn set_to_target_position(&mut self) {
    if let Some(tp) = self.target_position {
      self.position = tp;
    }
  }
  pub fn stop(&mut self) {
    self.velocity = Vec2::ZERO;
    self.target_position = None;
  }

  pub fn is_moving(&self) -> bool {
    self.velocity != Vec2::ZERO
  }

  pub fn has_reached_target_position(&self) -> bool {
    if let Some(tp) = self.target_position {
      return self.position.distance_squared(tp) < 10.0;
    }
    return false;
  }

  pub fn update(&mut self, delta_t: f32) {
     self.impulses = self.impulses
      .iter_mut()
      .filter_map(|v| {
        let new_v = *v * self.fraction;
        if new_v.length_squared() > EPSILON { Some(new_v) } else { None }
      })
      .collect();
    let implus_v =  self.impulses.iter()
      .fold(Vec2::ZERO, |acc, i| acc + *i);
    self.position += (self.velocity + implus_v) * delta_t;
  }
}