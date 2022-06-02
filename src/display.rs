use macroquad::prelude::*;

use crate::{world::{actor::Actor, projectile::Projectile}, cd::CdBounds};

pub struct Renderer {
  pub debug: bool
}

impl Renderer {

  pub fn draw_debug() {
    let fps = get_fps();
    draw_text(format!("fps: {}", fps).as_str(), 4.0, 24.0, 32., WHITE);
    draw_text(format!("screen size: {}x{}", screen_width(), screen_height()).as_str(), 4.0, 56.0, 32., WHITE);
  }

  fn draw_cd_data(bounding_rect: &CdBounds) {
    let rect = bounding_rect.get_rect();
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::from_rgba(124, 255, 124, 124));
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
          flip_x: actor.movable.rotation == 1.,
          ..Default::default()
      },
    );

    if self.debug {
      Renderer::draw_cd_data(&actor.cd_bounds);
      draw_text(
        format!("velocity: {:?}", actor.movable.velocity).as_str(),
        actor.movable.position.x - 32.,
        actor.movable.position.y - 20.0,
        14.,
        WHITE
      );
      draw_text(
        format!("position: {:?}", actor.movable.position).as_str(),
        actor.movable.position.x - 32.,
        actor.movable.position.y - 30.0,
        14.,
        WHITE
      );
      draw_text(
        format!("impuls: {}/{:?}", actor.movable.impulses.len(), actor.movable.impulses).as_str(),
        actor.movable.position.x - 32.,
        actor.movable.position.y - 40.0,
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
          flip_x: projectile.movable.rotation == 1.,
          ..Default::default()
      },
    );

    if self.debug {
      Renderer::draw_cd_data(&projectile.cd_bounds);
    }
  }

}
