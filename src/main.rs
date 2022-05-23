use actor::Actor;
use macroquad::prelude::*;


mod animation;
mod actor;
mod display;

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
    let mut actor = Actor::new(Vec2::new(screen_width() / 2., screen_height() / 2.), actor::State::Idle);

    loop {
        clear_background(DARKGRAY);

        let delta = get_frame_time();

        if is_mouse_button_released(MouseButton::Left) {
            actor.set_target_position(Vec2::from(mouse_position()));
        }

        actor.update(delta);

        display::draw_actor(&texture, &actor);
        next_frame().await
    }
}