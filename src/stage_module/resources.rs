use macroquad::prelude::*;
use crate::utils::{ReplaceColors, customize_image};

pub struct Resources {
  pub texture_actor: Texture2D,
  pub texture_actor_flashing: Texture2D,
  pub texture_enemy: Texture2D,
  pub texture_fireball: Texture2D,
  pub viewport: (f32, f32)
}

impl Resources {
  pub fn new(image: Image) -> Self {
    let colors_flashing = ReplaceColors::new(
      Color::from_rgba(255, 255, 255, 255),
        Color::from_rgba(255, 255, 255, 255),
        Color::from_rgba(255, 255, 255, 255),
        Color::from_rgba(255, 255, 255, 255),
    );

    let colors_actor = ReplaceColors::new(
      Color::from_rgba(223, 113, 38, 255),
        Color::from_rgba(172, 50, 50, 255),
        Color::from_rgba(238, 195, 145, 255),
        Color::from_rgba(0, 0, 0, 255),
    );

    let colors_enemy = ReplaceColors::new(
        Color::from_rgba(57, 159, 50, 255),
        Color::from_rgba(66, 105, 29, 255),
        Color::from_rgba(66, 159, 50, 255),
        Color::from_rgba(0, 0, 0, 255),
    );

    let texture_actor = customize_image(image.sub_image(Rect::new(0., 0., 16. * 3., 16.)), colors_actor);
    let texture_actor_flashing = customize_image(image.sub_image(Rect::new(0., 0., 16. * 3., 16.)), colors_flashing);
    let texture_enemy = customize_image(image.sub_image(Rect::new(16. * 3., 0., 16. * 3., 16.)), colors_enemy);
    let texture_fireball =  Texture2D::from_image(&image.sub_image(Rect::new(16. * 6., 0., 16. * 4., 16.)));

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

    Self {
      viewport: (i, i / ratio),
      texture_actor,
      texture_actor_flashing,
      texture_enemy,
      texture_fireball,
    }
  }

  pub fn get_camera(&self) -> Camera2D {
    Camera2D::from_display_rect(Rect::new(0.0, 0.0, self.viewport.0, self.viewport.1))
  }
}