use macroquad::prelude::*;

use crate::actor::{Rotation, Position};


pub fn draw_actor(texture: &Texture2D, position: &Position, rotation: &Rotation, source: Rect) {
  // println!("drawing from source: {:?}", source);
  draw_texture_ex(
    *texture,
    position.0.x - 32.,
    position.0.y - 32.,
    WHITE,
    DrawTextureParams {
        dest_size: Some(vec2(64., 64.)),
        source: Some(source),
        flip_x: rotation.0 == 1.,
        ..Default::default()
    },
);
}