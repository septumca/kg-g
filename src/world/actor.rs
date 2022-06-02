use std::sync::atomic::{AtomicUsize, Ordering};
use macroquad::{prelude::*};

use crate::{animation::Animation, world::{movable::Movable, damage::Hp}, cd::CdBounds};


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
pub struct Actor {
  id: usize,
  pub animation: Animation,
  pub movable: Movable,
  pub cd_bounds: CdBounds,
  pub hp: Hp
}

impl Actor {
  pub fn new(position: Vec2, speed: f32, health: isize) -> Self {
    Self {
      id: get_id(),
      animation: get_idle_animation(),
      movable: Movable::new(position.clone(), speed, 0.8),
      cd_bounds: CdBounds::new(position, 24., 32.),
      hp: Hp::new(health)
    }
  }

  pub fn get_id(&self) -> usize {
    self.id
  }

  pub fn get_source(&self) -> Rect {
    self.animation.get_act_frame()
  }

  pub fn is_alive(&self) -> bool {
    self.hp.is_alive()
  }

  pub fn move_to(&mut self, target_position: Vec2) {
    if !self.movable.is_moving() {
      self.animation = get_walking_animation();
    }
    self.movable.set_moving_to(target_position);
  }

  pub fn stop(&mut self) {
    self.movable.stop();
    self.animation = get_idle_animation();
  }

  pub fn update(&mut self, delta_t: f32) {
    self.animation.update(delta_t);
    self.movable.update(delta_t);
    self.cd_bounds.update_position(&self.movable.position);

    if self.movable.is_moving() {
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

  const SPEED: f32 = 100.;

  fn create() -> Actor {
    Actor::new(Vec2::ZERO, SPEED, 2)
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