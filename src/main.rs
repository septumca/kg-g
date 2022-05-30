use actor::{PlayerActor, State, Position, Rotation, spawn_actor, NextActorState};
use ai::{WeightedStates, Ai, AiState};
use animation::Animation;
use hecs::{World, Entity};
use macroquad::prelude::*;
use utils::{customize_image, ReplaceColors};


mod animation;
mod actor;
mod display;
mod utils;
mod timer;
mod ai;

#[macroquad::main("kg-g")]
async fn main() {
    let mut world = World::new();

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

    let player_entity = spawn_actor(&mut world, texture_actor, Vec2::new(screen_width() / 2., screen_height() / 2.));
    let _ = world.insert_one(player_entity, PlayerActor);

    println!("player {:?}", player_entity);
    let a = spawn_actor(&mut world, texture_enemy, Vec2::new(screen_width() / 2. - 50., screen_height() / 2. + 100.));
    let _ = world.insert_one(a, Ai::new(WeightedStates::new(vec![
        (1, AiState::Idle),
        (3, AiState::Wandering),
        (23, AiState::Following(player_entity))
    ])));

    let a = spawn_actor(&mut world, texture_enemy, Vec2::new(screen_width() / 2. - 50., screen_height() / 2. - 50.));
    let _ = world.insert_one(a, Ai::new(WeightedStates::new(vec![
        (1, AiState::Idle),
        (4, AiState::Wandering),
        (19, AiState::Following(player_entity))
    ])));

    let a = spawn_actor(&mut world, texture_enemy, Vec2::new(screen_width() / 2. - 50., screen_height() / 2. - 100.));
    let _ = world.insert_one(a, Ai::new(WeightedStates::new(vec![
        (3, AiState::Idle),
        (4, AiState::Wandering),
        (33, AiState::Following(player_entity))
    ])));

    let a = spawn_actor(&mut world, texture_enemy, Vec2::new(screen_width() / 2. - 50., screen_height() / 2. + 50.));
    let _ = world.insert_one(a, Ai::new(WeightedStates::new(vec![
        (0, AiState::Idle),
        (5, AiState::Wandering),
        (21, AiState::Following(player_entity))
    ])));

    loop {
        clear_background(DARKGRAY);
        let delta = get_frame_time();

        // if is_mouse_button_released(MouseButton::Left) {
        //     let _  = world.insert_one(player_entity, NextActorState(State::Walking(Vec2::from(mouse_position()))));
        // }
        // actor::refresh_states(&mut world);
        // actor::update(&mut world, delta);
        // for (_, animation) in &mut world.query::<&mut Animation>() {
        //     animation.update(delta);
        // }
        // let mut ai_new_states: Vec<(Entity, State)> = vec![];
        // for (id, (animation, ai)) in &mut world.query::<(&mut Animation, &mut Ai)>() {
        //     let state = ai.update(&world, delta, animation);
        //     if let Some(state) = state {
        //         ai_new_states.push((id, state));
        //     }
        // }
        // for (id, state) in ai_new_states {
        //     let _ = world.insert_one(id, NextActorState(state));
        // }
        // for (_, (position, rotation, animation, texture)) in &mut world.query::<(&Position, &Rotation, &Animation, &Texture2D)>() {
        //     display::draw_actor(texture, position, rotation, animation.get_act_frame());
        // }


        next_frame().await
    }
}