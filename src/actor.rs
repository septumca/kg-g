use std::sync::atomic::{AtomicUsize, Ordering};

use macroquad::{prelude::*};

use crate::{animation::Animation};

const SPEED: f32 = 100.;


static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_id() -> usize {
  COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn get_idle_animation() -> Animation {
  Animation::new(vec![Rect::new(0., 0., 16., 16.)], false)
}

fn get_walking_animation() -> Animation {
  Animation::new(
    vec![
      Rect::new(0., 0., 16., 16.),
      Rect::new(16., 0., 16., 16.),
      Rect::new(0., 0., 16., 16.),
      Rect::new(32., 0., 16., 16.),
    ],
    true
  )
}


#[derive(Debug, Clone)]
pub struct Movable {
  pub position: Vec2,
  pub target_position: Option<Vec2>,
  pub velocity: Vec2,
  pub rotation: f32,
  speed: f32,
}

impl Movable {
  pub fn new(position: Vec2) -> Self {
    Self {
      position,
      target_position: None,
      velocity: Vec2::ZERO,
      rotation: 0.,
      speed: SPEED,
    }
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
    self.position += self.velocity * delta_t;
  }
}

#[derive(Debug, Clone)]
pub struct Actor {
  id: usize,
  pub animation: Animation,
  pub movable: Movable,
}

impl Actor {
  pub fn new(position: Vec2) -> Self {
    Self {
      id: get_id(),
      animation: get_idle_animation(),
      movable: Movable::new(position),
    }
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_source(&self) -> Rect {
    self.animation.get_act_frame()
  }

  pub fn move_to(&mut self, target_position: Vec2) {
    self.animation = get_walking_animation();
    self.movable.set_moving_to(target_position);
  }

  pub fn stop(&mut self) {
    self.movable.stop();
    self.animation = get_idle_animation();
  }

  pub fn update(&mut self, delta_t: f32) {
    self.animation.update(delta_t);

    let is_moving = self.movable.is_moving();
    if is_moving {
      self.movable.update(delta_t);
      if self.movable.has_reached_target_position() {
        self.movable.set_to_target_position();
        self.stop();
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create() -> Actor {
    Actor::new(Vec2::ZERO)
  }

  #[test]
  fn update() {
    let mut actor = create();
    let tp = Vec2::new(6., 6.);
    let delta_time = 0.01;
    let delta_v = (tp - Vec2::ZERO).normalize();

    actor.move_to(tp);
    assert_eq!(actor.movable.is_moving(), true);

    actor.update(delta_time);
    assert_eq!(actor.movable.position, Vec2::ZERO + delta_v * SPEED * delta_time);

    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);

    assert_eq!(actor.movable.is_moving(), false);
    assert_eq!(actor.get_source(), Rect::new(0., 0., 16., 16.));
  }
}