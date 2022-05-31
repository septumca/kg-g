use macroquad::prelude::*;

use crate::{actor::Actor, cd::BoundRect};

pub struct Renderer {
  pub debug: bool
}

impl Renderer {

  pub fn draw_debug() {
    let fps = get_fps();
    draw_text(format!("fps: {}", fps).as_str(), 4.0, 24.0, 32., WHITE);
  }

  fn draw_cd_data(bounding_rect: &BoundRect) {
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
      Renderer::draw_cd_data(&actor.bound_rect);
    }
  }

  pub fn draw_projectile() {
    todo!()
  }

}
