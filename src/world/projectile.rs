use std::sync::atomic::{AtomicUsize, Ordering};

use macroquad::{prelude::*};

use crate::{world::{movable::Movable, actor::Actor}, animation::Animation, cd::CdBounds};


static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_id() -> usize {
  COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn get_flying_animation() -> Animation {
  Animation::new(
    vec![
      Rect::new(0., 0., 16., 16.),
      Rect::new(16., 0., 16., 16.),
      Rect::new(32., 0., 16., 16.),
      Rect::new(48., 0., 16., 16.),
    ],
    true
  )
}

#[derive(Debug, Clone)]
pub struct Projectile {
  id: usize,
  pub movable: Movable,
  pub animation: Animation,
  pub cd_bounds: CdBounds,
  pub is_alive: bool,
}

impl Projectile {
  pub fn new(position: Vec2, velocity: Vec2) -> Self {
    Self {
      id: get_id(),
      movable: Movable::new(position, 150., 1.).with_velocity(velocity),
      animation: get_flying_animation(),
      cd_bounds: CdBounds::new(position, 16., 16.),
      is_alive: true,
    }
  }

  pub fn apply(&mut self, actor: &mut Actor) {
    if !actor.hp.has_been_modified_by_source(self.id) {
      actor.hp.modify(self.id, -1);
    }
    self.is_alive = false;
  }

  pub fn get_source(&self) -> Rect {
    self.animation.get_act_frame()
  }

  pub fn update(&mut self, delta_t: f32) {
    self.animation.update(delta_t);
    self.movable.update(delta_t);
    self.cd_bounds.update_position(&self.movable.position);
  }
}

pub fn spawn_projectile_from_actor(position_from: &Vec2, position_to: &Vec2) -> Projectile {
  let velocity = (*position_to - *position_from).normalize();
  let position = *position_from + (velocity * 32.);

  Projectile::new(position, velocity)
}