use macroquad::prelude::*;

use crate::actor::Actor;

pub fn draw_actor(texture: &Texture2D, actor: &Actor) {
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
}