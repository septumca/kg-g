use macroquad::prelude::*;
use crate::utils::{ReplaceColors, customize_image};

pub struct Resources {
  pub texture_actor: Texture2D,
  pub texture_enemy: Texture2D,
  pub texture_fireball: Texture2D,
}

impl Resources {
  pub fn new(image: Image) -> Self {
    let colors_actor = ReplaceColors::new(
      Color::from_rgba(223, 113, 38, 255),
        Color::from_rgba(172, 50, 50, 255),
        Color::from_rgba(238, 195, 145, 255),
        Color::from_rgba(0, 0, 0, 255),
    );

    let colors_enemy = ReplaceColors::new(
        Color::from_rgba(57, 99, 50, 255),
        Color::from_rgba(66, 55, 29, 255),
        Color::from_rgba(66, 99, 50, 255),
        Color::from_rgba(0, 0, 0, 255),
    );

    let texture_actor = customize_image(image.sub_image(Rect::new(0., 0., 16. * 3., 16.)), colors_actor);
    let texture_enemy = customize_image(image.sub_image(Rect::new(16. * 3., 0., 16. * 3., 16.)), colors_enemy);
    let texture_fireball =  Texture2D::from_image(&image.sub_image(Rect::new(16. * 6., 0., 16. * 4., 16.)));

    Self {
      texture_actor,
      texture_enemy,
      texture_fireball,
    }
  }
}