use macroquad::prelude::*;

#[macroquad::main("kg-g")]
async fn main() {
    let mut frames_img = Image::from_file_with_format(include_bytes!("../assets/frames.png"), Some(ImageFormat::Png));

    for x in 0..frames_img.width() as u32 {
        for y in 0..frames_img.height() as u32 {
            let c = frames_img.get_pixel(x, y);
            let new_c = match ((c.r*255.) as u32, (c.g*255.) as u32, (c.b*255.) as u32) {
                (245, 115, 147) => Color::from_rgba(223, 113, 38, 255),
                (245, 135, 147) => Color::from_rgba(172, 50, 50, 255),
                (245, 145, 147) => Color::from_rgba(238, 195, 145, 255),
                (245, 95, 147) => Color::from_rgba(0, 0, 0, 255),
                _ => c
            };

            if c != new_c {
                frames_img.set_pixel(x, y, new_c);
            }
        }
    }
    let texture: Texture2D = Texture2D::from_image(&frames_img);
    texture.set_filter(FilterMode::Nearest);
    let mut position = (screen_width() / 2., screen_height() / 2.,);

    loop {
        clear_background(DARKGRAY);

        // let delta = get_frame_time();

        if is_mouse_button_released(MouseButton::Left) {
            position = mouse_position();
        }

        draw_texture_ex(
            texture,
            position.0 - 32.,
            position.1 - 32.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64., 64.)),
                source: Some(Rect::new(0., 0., 16., 16.)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}