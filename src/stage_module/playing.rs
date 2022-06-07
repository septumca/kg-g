use macroquad::{prelude::*, rand::ChooseRandom};

use crate::{world_module::{world::{World}, actor::Actor}, player::Player, systems::ai::{Ai, WeightedStates}, display::Renderer};

use super::{stage_stack::{Stage, StageAction}, resources::Resources};

const ENEMIES_COUNT: usize = 2;

pub struct PlayingStage {
  world: World,
  paused: bool,
  renderer: Renderer,
  camera: Camera2D,
  viewport: (f32, f32)
}

impl PlayingStage {
  pub fn new() -> Self {
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

    let ratio = screen_width() / screen_height();
    let mut i = 1.;
    if screen_width() > 1000. || screen_height() > 1000. {
      loop {
        i += 1.;

        if i > 1000. || (i / ratio) > 1000. {
          i -=1.;
          break;
        }
      }
    } else {
      i = screen_width();
    }

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, i, i / ratio));

    Self {
      viewport: (i, i / ratio),
      world: World::new(player).with_ai_actors(ai_actors),
      paused: false,
      renderer: Renderer { debug: false },
      camera
    }
  }
}

impl PlayingStage {
  fn get_lrtb(&self) -> (f32, f32, f32, f32) {
    (
      self.camera.target.x - (self.viewport.0 / 2.),
      self.camera.target.x + (self.viewport.0 / 2.),
      self.camera.target.y - (self.viewport.1 / 2.),
      self.camera.target.y + (self.viewport.1 / 2.),
    )
  }
}

impl Stage for PlayingStage {
  fn update(&mut self, _resources: &Resources) -> Option<StageAction> {
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
      return Some(StageAction::EndGame);
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

      let (vp_left, vp_right, vp_top, vp_bottom) = self.get_lrtb();
      let (left, right, top, bottom) = (vp_left.max(self.world.bounds.left()), vp_right.min(self.world.bounds.right()), vp_top.max(self.world.bounds.top()), vp_bottom.min(self.world.bounds.bottom()));

      if self.world.spawn_timer.is_just_over() {
        if let Some(pos) = vec![
          Vec2::new(left, rand::gen_range::<f32>(top, bottom)),
          Vec2::new(right, rand::gen_range::<f32>(top, bottom)),
          Vec2::new(rand::gen_range::<f32>(left, right), top),
          Vec2::new(rand::gen_range::<f32>(left, right), bottom),
        ].choose() {
          self.world.spawn_enemy(*pos);
        }
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

    let (left, right, top, _) = self.get_lrtb();

    self.renderer.draw_player_info(right, top, &self.world);
    self.renderer.draw_debug(left, top, &self.world);
  }
}