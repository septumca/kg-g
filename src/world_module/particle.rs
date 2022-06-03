use macroquad::prelude::*;

use crate::systems::{animation::Animation, timer::Timer};

use super::movable::Movable;



#[derive(Debug, Clone)]
pub struct Particle {
  pub movable: Movable,
  pub animation: Animation,
  timeout: Timer
}

impl Particle {
  pub fn new(pos: Vec2, vel: Vec2, frames: Vec<Rect>, ttl: f32) -> Self {
    Self {
      movable: Movable::new(pos, 50., 0.).with_velocity(vel),
      animation: Animation::new(frames, false),
      timeout: Timer::new_timeout(ttl)
    }
  }

  pub fn update(&mut self, delta_t: f32) {
    self.movable.update(delta_t);
    self.animation.update(delta_t);
    self.timeout.update(delta_t);
  }

  pub fn is_alive(&self) -> bool {
    !self.timeout.is_over()
  }
}


pub struct ParticleSystem {
  pub particles: Vec<Particle>
}

impl ParticleSystem {
  pub fn new() -> Self {
    Self { particles: vec![] }
  }

  pub fn add_particle(&mut self, particle: Particle) {
    self.particles.push(particle);
  }

  pub fn update(&mut self, delta_t: f32) {
    let new_particles: Vec<Particle> = self.particles.clone();
    self.particles.clear();

    for mut p in new_particles {
      p.update(delta_t);
      if p.is_alive() {
        self.particles.push(p);
      }
    }
  }
}