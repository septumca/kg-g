use macroquad::prelude::*;


pub struct ReplaceColors {
  armor: Color,
  detail: Color,
  skin: Color,
  eyes: Color,
}

impl ReplaceColors {
  pub fn new(armor: Color, detail: Color, skin: Color, eyes: Color) -> Self {
    Self { armor, detail, skin, eyes }
  }

  fn replace(&self, c: Color) -> Color {
    match ((c.r*255.) as u32, (c.g*255.) as u32, (c.b*255.) as u32) {
      (245, 115, 147) => self.armor,
      (245, 135, 147) => self.detail,
      (245, 145, 147) => self.skin,
      (245, 95, 147) => self.eyes,
      _ => c
    }
  }
}

pub fn customize_image(mut image: Image, colors: ReplaceColors) -> Texture2D {
  for x in 0..image.width() as u32 {
    for y in 0..image.height() as u32 {
        let c = image.get_pixel(x, y);
        let new_c = colors.replace(c);

        if c != new_c {
          image.set_pixel(x, y, new_c);
        }
    }
  }

  let texture: Texture2D = Texture2D::from_image(&image);
  texture.set_filter(FilterMode::Nearest);

  texture
}
