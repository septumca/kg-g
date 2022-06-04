use std::collections::HashMap;

use macroquad::{prelude::*, rand::ChooseRandom};


use crate::{player::Player, systems::{ai::{Ai, WeightedStates}, timer::Timer}};

use super::{projectile::Projectile, actor::Actor, particle::{ParticleSystem, Particle}};

pub struct World {
  pub player: Player,
  ai_actors: Vec<Actor>,
  ai_controllers: HashMap<usize, Ai>,
  projectiles: Vec<Projectile>,
  pub particle_system: ParticleSystem,
  bounds: Rect,
  spawn_timer: Timer,
}

impl World {
  pub fn new(player: Player) -> Self {
    Self {
      player,
      ai_actors: vec![],
      ai_controllers: HashMap::new(),
      projectiles: vec![],
      particle_system: ParticleSystem::new(),
      bounds: Rect::new(0., 0., screen_width(), screen_height()),
      spawn_timer: Timer::new(0.8),
    }
  }

  pub fn with_ai_actors(mut self, ai_actors: Vec<(Actor, Ai)>) -> Self {
    for (actor, ai) in ai_actors {
      let id = actor.get_id();
      self.ai_actors.push(actor);
      self.ai_controllers.insert(id, ai);
    }
    self
  }

  pub fn on_mouse_button_down(&mut self, position: Vec2) {
    self.player.actor.move_to(position);
  }

  pub fn get_player(&self) -> &Player {
    &self.player
  }

  pub fn get_ai_actors(&self) -> &Vec<Actor> {
    &self.ai_actors
  }

  pub fn get_projectiles(&self) -> &Vec<Projectile> {
    &self.projectiles
  }

  pub fn get_particles(&self) -> &Vec<Particle> {
    &self.particle_system.particles
  }

  fn cleanup(&mut self) {
    self.projectiles = self.projectiles.clone().into_iter().filter(|p| p.is_alive).collect();
    let (alive, dead) = self.ai_actors
      .clone()
      .into_iter()
      .partition(|a| a.is_alive());

    self.ai_actors = alive;
    for actor in dead {
      self.ai_controllers.remove(&actor.get_id());
    }
  }

  pub fn update(&mut self, delta_t: f32) {
    self.player.update(delta_t, &mut self.projectiles, &self.ai_actors);
    self.player.actor.update(delta_t);

    self.particle_system.update(delta_t);

    for projectile in &mut self.projectiles {
      projectile.update(delta_t);
      if !self.bounds.contains(projectile.movable.position) {
        projectile.is_alive = false;
        continue;
      }
      if projectile.particles_timer.is_just_over() {
        let frames = vec![
          Rect::new(16., 0., 16., 16.),
          Rect::new(32., 0., 16., 16.),
          Rect::new(48., 0., 16., 16.),
        ];
        self.particle_system.add_particle(Particle::new(projectile.movable.position, Vec2::ZERO, frames, 0.3))
      }
      if let Some(collided_actor) = self.ai_actors.iter_mut().find(|actor| actor.cd_bounds.collide_with(&projectile.cd_bounds)) {
        projectile.apply(collided_actor);
      }
    }

    let actors_clone = self.ai_actors.clone();
    for actor_a in self.ai_actors.iter_mut() {
      if let Some(ai) = self.ai_controllers.get_mut(&actor_a.get_id()) {
        ai.update(delta_t, actor_a, &self.player.actor);

        if actor_a.cd_bounds.collide_with(&self.player.actor.cd_bounds) {
          ai.set_state(crate::systems::ai::AiState::Wandering, actor_a, &self.player.actor);
          self.player.modify_hp(actor_a.get_id(), -1);
        }
      }
      actor_a.update(delta_t);
      let mut impuls: Option<Vec2> = None;
      for actor_b in &actors_clone {
        if actor_a.get_id() != actor_b.get_id() && actor_a.cd_bounds.collide_with(&actor_b.cd_bounds) {
          impuls = Some((actor_a.movable.position - actor_b.movable.position).normalize() * 120.);
        }
      }
      if let Some(imp) = impuls {
        actor_a.movable.add_impuls(imp);
      }
    }

    self.spawn_timer.update(delta_t);
    if self.spawn_timer.is_just_over() {
      if let Some(pos) = vec![
        Vec2::new(self.bounds.left(), rand::gen_range::<f32>(self.bounds.top(), self.bounds.bottom())),
        Vec2::new(self.bounds.right(), rand::gen_range::<f32>(self.bounds.top(), self.bounds.bottom())),
        Vec2::new(rand::gen_range::<f32>(self.bounds.left(), self.bounds.right()), self.bounds.top()),
        Vec2::new(rand::gen_range::<f32>(self.bounds.left(), self.bounds.right()), self.bounds.bottom()),
      ].choose() {
        let actor = Actor::new(*pos, 75., 2);
        let ai = Ai::new(WeightedStates::new_idle_wandering(&[1, 5, 7]));

        self.ai_controllers.insert(actor.get_id(), ai);
        self.ai_actors.push(actor);
      }
    }

    self.cleanup();
  }
}