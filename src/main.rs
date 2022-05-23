use actor::{Actor, PlayerActor, AiActor};
use macroquad::prelude::*;
use utils::{customize_image, ReplaceColors};


mod animation;
mod actor;
mod display;
mod utils;
mod timer;

#[macroquad::main("kg-g")]
async fn main() {
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

    let image = Image::from_file_with_format(include_bytes!("../assets/frames.png"), Some(ImageFormat::Png));
    let texture_actor = customize_image(image.sub_image(Rect::new(0., 0., 16. * 3., 16.)), colors_actor);
    let texture_enemy = customize_image(image.sub_image(Rect::new(16. * 3., 0., 16. * 3., 16.)), colors_enemy);
    let mut player = PlayerActor{ actor: Actor::new(Vec2::new(screen_width() / 2., screen_height() / 2.), actor::State::Idle) };
    let mut enemy = AiActor::new(Actor::new(Vec2::new(screen_width() / 2. + 50., screen_height() / 2.), actor::State::Idle));

    loop {
        clear_background(DARKGRAY);

        let delta = get_frame_time();

        if is_mouse_button_released(MouseButton::Left) {
            player.actor.set_target_position(Vec2::from(mouse_position()));
        }

        player.actor.update(delta);
        enemy.update(delta);

        display::draw_actor(&texture_actor, &player.actor);
        display::draw_actor(&texture_enemy, &enemy.actor);
        next_frame().await
    }
}