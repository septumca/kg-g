use std::collections::{HashMap};

use actor::{Actor};
use ai::{WeightedStates, Ai};
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

  let mut player_actor = Actor::new(Vec2::new(screen_width() / 2., screen_height() / 2.));
  let ai_actor1 = Actor::new(Vec2::new(screen_width() / 2. - 50., screen_height() / 2.));
  let ai_actor2 = Actor::new(Vec2::new(screen_width() / 2. - 100., screen_height() / 2.));

  let mut ai_controllers = vec![
    Ai::new(WeightedStates::new_idle_wandering(&[1, 10, 30]), ai_actor1.get_id()),
    Ai::new(WeightedStates::new_idle_wandering(&[2, 10, 30]), ai_actor2.get_id())
  ];

  let mut ai_actors: HashMap<usize, Actor> = HashMap::new();
  ai_actors.insert(ai_actor1.get_id(), ai_actor1);
  ai_actors.insert(ai_actor2.get_id(), ai_actor2);

  loop {
    clear_background(DARKGRAY);
    let delta = get_frame_time();

    if is_mouse_button_released(MouseButton::Left) {
      player_actor.move_to_and_animate(Vec2::from(mouse_position()));
    }

    for ai in &mut ai_controllers {
      if let Some(actor) = ai_actors.get_mut(&ai.actor_id) {
        ai.update(delta, actor, &player_actor);
      };
    }

    player_actor.update(delta);
    for actor in ai_actors.values_mut() {
      actor.update(delta);
    }


    display::draw_actor(&texture_actor, &player_actor);
    for actor in ai_actors.values() {
      display::draw_actor(&texture_enemy, &actor);
    }

    next_frame().await
    }
}