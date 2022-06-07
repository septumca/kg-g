use macroquad::prelude::*;

use crate::{world_module::{world::World, actor::Actor}, player::Player, systems::ai::{Ai, WeightedStates}, display::Renderer, utils::{WORLD_WIDTH, WORLD_HEIGHT}};

use super::{stage_stack::{Stage, StageAction}, resources::Resources};

const ENEMIES_COUNT: usize = 0;

pub struct PlayingStage {
  world: World,
  paused: bool,
  renderer: Renderer,
}

impl PlayingStage {
  pub fn new() -> Self {
    let scale = (screen_width() / WORLD_WIDTH).min(screen_height() / WORLD_HEIGHT);
    let player_actor = Actor::new(Vec2::new(WORLD_WIDTH / 2., WORLD_HEIGHT / 2.), 100., 5);
    let player = Player::new(player_actor, 1.);


    let mut ai_actors: Vec<(Actor, Ai)> = vec![];
    for c in 0..ENEMIES_COUNT {
      let x_mod = (c % 12) as f32;
      let y_mod = (c / 12) as f32;
      let actor = Actor::new(Vec2::new(32. + x_mod * 64., 64. + y_mod * 64.), 80., 2);
      let ai = Ai::new(WeightedStates::new_idle_wandering(&[1, 5, 30]));
      ai_actors.push((actor, ai));
    }

    Self {
      world: World::new(player).with_ai_actors(ai_actors),
      paused: false,
      renderer: Renderer { debug: false, scale },
    }
  }
}

impl Stage for PlayingStage {
  fn update(&mut self, _resources: &Resources) -> Option<StageAction> {
    if is_key_pressed(KeyCode::D) {
      self.renderer.debug = !self.renderer.debug;
    }

    if is_key_pressed(KeyCode::P) {
      self.paused = !self.paused;
    }

    if is_key_pressed(KeyCode::Escape) {
      return Some(StageAction::EndGame);
    }

    if !self.world.player.actor.is_alive() {
      return Some(StageAction::EndGame);
    }

    if !self.paused {
      let delta_t = get_frame_time();
      if is_mouse_button_pressed(MouseButton::Left) {
        let new_pos = Vec2::from(mouse_position()) / self.renderer.scale;

        if self.world.bounds.contains(new_pos) {
          self.world.on_mouse_button_down(new_pos);
        }
      }

      self.world.update(delta_t);
    }

    None
  }

  fn draw(&self, resources: &Resources) {
    clear_background(BLACK);

    draw_rectangle(0., 0., self.world.bounds.w * self.renderer.scale, self.world.bounds.h * self.renderer.scale, DARKGRAY);

    if self.paused {
      draw_text("PAUSED", (screen_width() / 2. - 40.) / self.renderer.scale, (screen_height() / 2. - 4.) / self.renderer.scale, 32. * self.renderer.scale, WHITE);
    }

    let player_texure = if self.world.player.invlunerable { &resources.texture_actor_flashing } else { &resources.texture_actor };
    self.renderer.draw_actor(player_texure, &self.world.get_player().actor);
    for actor in self.world.get_ai_actors() {
      self.renderer.draw_actor(&resources.texture_enemy, actor);
    }
    for projectile in self.world.get_projectiles() {
      self.renderer.draw_projectile(&resources.texture_fireball, projectile);
    }
    for particle in self.world.get_particles() {
      self.renderer.draw_particle(&resources.texture_fireball, particle);
    }

    self.renderer.draw_player_info(&self.world);
    self.renderer.draw_debug(&self.world);
  }
}