use macroquad::{prelude::*};

use crate::{movable::Movable, animation::Animation, cd::BoundRect};


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

pub struct Projectile {
  pub movable: Movable,
  pub animation: Animation,
  pub bound_rect: BoundRect,
  pub is_alive: bool,
}

impl Projectile {
  pub fn new(position: Vec2, velocity: Vec2) -> Self {
    Self {
      movable: Movable::new(position, 300.).add_velocity(velocity),
      animation: get_flying_animation(),
      bound_rect: BoundRect::new(position, 16., 16.),
      is_alive: true,
    }
  }

  pub fn get_source(&self) -> Rect {
    self.animation.get_act_frame()
  }

  pub fn update(&mut self, delta_t: f32) {
    self.animation.update(delta_t);
    self.movable.update(delta_t);
    self.bound_rect.update_position(&self.movable.position);
  }
}

pub fn spawn_projectile_from_actor(position_from: &Vec2, position_to: &Vec2) -> Projectile {
  let velocity = (*position_to - *position_from).normalize();
  let position = *position_from + (velocity * 32.);

  Projectile::new(position, velocity)
}