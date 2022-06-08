use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{world_module::{world::{World}, actor::Actor}, player::Player, systems::{ai::{Ai, WeightedStates}, timer::Timer}, display::Renderer};

use super::{stage_stack::{Stage, StageAction}, resources::Resources};

const ENEMIES_COUNT: usize = 0;
const BASE_SPAWN_TRESHOLD: f32 = 5.;

pub struct PlayingStage {
  world: World,
  difficulty: usize,
  paused: bool,
  renderer: Renderer,
  camera: Camera2D,
  spawn_timer: Timer,
  difficulty_timer: Timer,
}

impl PlayingStage {
  pub fn new(resources: &Resources) -> Self {
    let player_position = Vec2::new(0., 0.);
    let player_actor = Actor::new(player_position, 100., 5);
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
      difficulty: 0,
      world: World::new(player).with_ai_actors(ai_actors),
      paused: false,
      renderer: Renderer { debug: false },
      camera: resources.get_camera(),
      spawn_timer: Timer::new(2.),
      difficulty_timer: Timer::new(BASE_SPAWN_TRESHOLD),
    }
  }
}

impl PlayingStage {
  fn get_lrtb(&self, resources: &Resources) -> (f32, f32, f32, f32) {
    (
      self.camera.target.x - (resources.viewport.0 / 2.),
      self.camera.target.x + (resources.viewport.0 / 2.),
      self.camera.target.y - (resources.viewport.1 / 2.),
      self.camera.target.y + (resources.viewport.1 / 2.),
    )
  }
}

impl Stage for PlayingStage {
  fn update(&mut self, resources: &Resources) -> Option<StageAction> {
    self.camera.target = self.world.player.actor.movable.position;

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
      return Some(StageAction::GameOver(self.world.score));
    }

    if !self.paused {
      let delta_t = get_frame_time();
      if is_mouse_button_pressed(MouseButton::Left) {
        let m_vec = self.camera.screen_to_world(Vec2::from(mouse_position()));
        if self.world.bounds.contains(m_vec) {
          self.world.on_mouse_button_down(m_vec);
        }
      }

      self.world.update(delta_t);

      let (vp_left, vp_right, vp_top, vp_bottom) = self.get_lrtb(resources);
      let (left, right, top, bottom) = (vp_left.max(self.world.bounds.left()), vp_right.min(self.world.bounds.right()), vp_top.max(self.world.bounds.top()), vp_bottom.min(self.world.bounds.bottom()));

      self.spawn_timer.update(delta_t);
      self.difficulty_timer.update(delta_t);

      if self.spawn_timer.is_just_over() {
        if let Some(pos) = vec![
          Vec2::new(left, rand::gen_range::<f32>(top, bottom)),
          Vec2::new(right, rand::gen_range::<f32>(top, bottom)),
          Vec2::new(rand::gen_range::<f32>(left, right), top),
          Vec2::new(rand::gen_range::<f32>(left, right), bottom),
        ].choose() {
          let actor = Actor::new(*pos, 70. + self.difficulty as f32, 2);
          let ai = Ai::new(WeightedStates::new_idle_wandering(&[1, 5, 7+self.difficulty as i32]));
          self.world.add_ai_actor(actor, ai);
        }
      }

      if self.difficulty_timer.is_just_over() {
        self.difficulty += 1;

        let spawn_treshold = match self.difficulty {
          1 => 4.,
          2 => 3.,
          3..=4 => 2.,
          5..=7 => 1.,
          8..=11 => 0.9,
          12..=16 => 0.7,
          _ => 0.5,
        };
        self.spawn_timer.set_treshold(spawn_treshold);
        self.spawn_timer.reset();
      }
    }

    None
  }

  fn draw(&self, resources: &Resources) {
    clear_background(BLACK);

    draw_rectangle(self.world.bounds.x, self.world.bounds.x, self.world.bounds.w, self.world.bounds.h, DARKGRAY);

    set_camera(&self.camera);

    if self.paused {
      draw_text("PAUSED", screen_width() / 2. - 40., screen_height() / 2. - 4., 32., WHITE);
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

    let (left, _r, top, _b) = self.get_lrtb(resources);

    self.renderer.draw_player_info(left, top, &self.world, self.difficulty);
    self.renderer.draw_debug(left, top, &self.world);
  }
}