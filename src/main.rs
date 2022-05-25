use actor::{Actor, PlayerActor, State};
use ai::{WeightedStates, Ai, AiState};
use hecs::World;
use macroquad::prelude::*;
use physics::KgPhysics;
use utils::{customize_image, ReplaceColors};


mod animation;
mod actor;
mod display;
mod utils;
mod timer;
mod ai;
mod physics;

#[macroquad::main("kg-g")]
async fn main() {
    let mut world = World::new();
    let mut physics = KgPhysics::new();

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

    world.spawn((
        Actor::new(Vec2::new(screen_width() / 2., screen_height() / 2.), actor::State::Idle),
        PlayerActor,
        texture_actor
    ));
    world.spawn((
        Actor::new(Vec2::new(screen_width() / 2. - 50., screen_height() / 2.), actor::State::Idle),
        Ai::new(WeightedStates::new(vec![
            (2, AiState::Idle),
            (8, AiState::Wandering),
        ])),
        texture_enemy
    ));
    world.spawn((
        Actor::new(Vec2::new(screen_width() / 2. - 50., screen_height() / 2. + 50.), actor::State::Idle),
        Ai::new(WeightedStates::new(vec![
            (2, AiState::Idle),
            (8, AiState::Wandering),
        ])),
        texture_enemy
    ));
    world.spawn((
        Actor::new(Vec2::new(screen_width() / 2. - 50., screen_height() / 2. - 50.), actor::State::Idle),
        Ai::new(WeightedStates::new(vec![
            (2, AiState::Idle),
            (8, AiState::Wandering),
        ])),
        texture_enemy
    ));

    loop {
        clear_background(DARKGRAY);
        let delta = get_frame_time();

        if is_mouse_button_released(MouseButton::Left) {
            if let Some((_id, (actor, _))) = &mut world.query::<(&mut Actor, &PlayerActor)>().into_iter().next() {
                actor.set_new_state(State::Walking(Vec2::from(mouse_position())));
            }
        }

        for (_id, (actor, ai)) in &mut world.query::<(&mut Actor, &mut Ai)>() {
            let state = ai.update(delta, actor);
            if let Some(state) = state {
                actor.set_new_state(state);
            }
        }

        for (_id, actor) in &mut world.query::<&mut Actor>() {
            actor.update(delta);
        }

        physics.update();

        for (_id, (actor, texture)) in &mut world.query::<(&Actor, &Texture2D)>() {
            display::draw_actor(texture, actor);
        }

        next_frame().await
    }
}