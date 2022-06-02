use macroquad::{prelude::*};

use crate::{world::{actor::Actor, projectile::{Projectile, spawn_projectile_from_actor}}, timer::Timer};

pub struct Player {
  pub actor: Actor,
  projectile_timer: Timer,
}


impl Player {
  pub fn new(actor: Actor, projectile_timeout: f32) -> Self {
    Self {
      actor,
      projectile_timer: Timer::new(projectile_timeout)
    }
  }

  pub fn update(&mut self, delta_t: f32, projectiles: &mut Vec<Projectile>, enemies: &Vec<Actor>) {
    if self.projectile_timer.update(delta_t) {
      let player_position = &self.actor.movable.position;

      if let Some(closest) = enemies.iter().min_by(|e_a, e_b| {
        let d_a = player_position.distance_squared(e_a.movable.position);
        let d_b = player_position.distance_squared(e_b.movable.position);

        d_a.partial_cmp(&d_b).expect(format!("{} and {} should be comparable", d_a, d_b).as_str())
      }) {
        projectiles.push(spawn_projectile_from_actor(player_position, &closest.movable.position));
      }
    }
  }
}