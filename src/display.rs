use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{world_module::{actor::Actor, projectile::Projectile, movable::Movable, particle::Particle, world::World}, systems::{cd::CdBounds}};

pub struct Renderer {
  pub debug: bool
}

impl Renderer {

  pub fn draw_player_info(&self, right: f32, top: f32, world: &World) {
    draw_text(format!("HP: {}", world.player.actor.hp.act_hp).as_str(), right - 150., top + 16., 24., WHITE);
    draw_text(format!("SCORE: {}", world.score).as_str(), right - 150., top + 40., 24., WHITE);
  }

  pub fn draw_debug(&self, left: f32, top: f32, world: &World) {
    if self.debug {
      let font_size: f32 = 16.;
      let o_x: f32 = left + 4.;
      let o_y: f32 = top + 12.;
      let fps = get_fps();

      draw_text(format!("fps: {}", fps).as_str(), o_x, o_y, font_size, WHITE);
      draw_text(format!("screen size: {}x{}", screen_width(), screen_height()).as_str(), o_x, o_y + font_size, font_size, WHITE);
      draw_text(format!("ai actors count: {}", world.get_ai_actors().len()).as_str(), o_x, o_y + font_size * 2., font_size, WHITE);
      draw_text(format!("projecties count: {}", world.get_projectiles().len()).as_str(), o_x, o_y + font_size * 3., font_size, WHITE);
      draw_text(format!("particles count: {}", world.get_particles().len()).as_str(), o_x, o_y + font_size * 4., font_size, WHITE);
      draw_text(format!("fire ready: {}", world.get_player().projectile_timer.is_over()).as_str(), o_x, o_y + font_size * 5., font_size, WHITE);
    }
  }

  fn draw_cd_data(bounding_rect: &CdBounds) {
    let rect = bounding_rect.get_rect();
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::from_rgba(124, 255, 124, 124));
  }

  fn draw_movable_data(movable: &Movable, o_x: f32, o_y: f32) {
    draw_text(
      format!("vel: {:.2}x{:.2}", movable.velocity.x, movable.velocity.y,).as_str(),
      o_x,
      o_y,
      14.,
      WHITE
    );
    draw_text(
      format!("pos: {:.2}x{:.2}", movable.position.x, movable.position.y,).as_str(),
      o_x,
      o_y - 10.0,
      14.,
      WHITE
    );
    draw_text(
      format!("imp: {:.2}x{:.2}", movable.impuls.x, movable.impuls.y,).as_str(),
      o_x,
      o_y - 20.0,
      14.,
      WHITE
    );
    draw_text(
      format!("rotation: {:.2}", movable.rotation * 180. / PI).as_str(),
      o_x,
      o_y - 30.0,
      14.,
      WHITE
    );
  }

  pub fn draw_actor(&self, texture: &Texture2D, actor: &Actor) {
    draw_texture_ex(
      *texture,
      actor.movable.position.x - 32.,
      actor.movable.position.y - 32.,
      WHITE,
      DrawTextureParams {
          dest_size: Some(vec2(64., 64.)),
          source: Some(actor.get_source()),
          flip_x: actor.movable.rotation > PI / 2. || actor.movable.rotation < -PI / 2.,
          ..Default::default()
      },
    );

    if self.debug {
      Renderer::draw_cd_data(&actor.cd_bounds);
      Renderer::draw_movable_data(&actor.movable, actor.movable.position.x - 32., actor.movable.position.y - 20.);
      draw_text(
        format!("hp: {}", actor.hp.act_hp).as_str(),
        actor.movable.position.x - 32.,
        actor.movable.position.y - 60.0,
        14.,
        WHITE
      );
    }
  }

  pub fn draw_projectile(&self, texture: &Texture2D, projectile: &Projectile) {
    draw_texture_ex(
      *texture,
      projectile.movable.position.x - 16.,
      projectile.movable.position.y - 16.,
      WHITE,
      DrawTextureParams {
          dest_size: Some(vec2(32., 32.)),
          source: Some(projectile.get_source()),
          rotation: projectile.movable.rotation,
          ..Default::default()
      },
    );

    if self.debug {
      Renderer::draw_movable_data(&projectile.movable, projectile.movable.position.x - 32., projectile.movable.position.y - 20.);
      Renderer::draw_cd_data(&projectile.cd_bounds);
    }
  }

  pub fn draw_particle(&self, texture: &Texture2D, particle: &Particle) {
    draw_texture_ex(
      *texture,
      particle.movable.position.x - 24.,
      particle.movable.position.y - 24.,
      WHITE,
      DrawTextureParams {
          dest_size: Some(vec2(48., 48.)),
          source: Some(particle.animation.get_act_frame()),
          rotation: particle.movable.rotation,
          ..Default::default()
      },
    );
  }

}
