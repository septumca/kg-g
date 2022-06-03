use ai::{WeightedStates, Ai};
use display::Renderer;
use macroquad::prelude::*;
use player::Player;
use world::{actor::Actor, world::World};
use utils::{customize_image, ReplaceColors};


mod animation;
mod world;
mod display;
mod utils;
mod timer;
mod ai;

mod cd;
mod player;

const ENEMIES_COUNT: usize = 5;

#[macroquad::main("kg-g")]
async fn main() {
  set_pc_assets_folder("assets");

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

  let colors_fireball = ReplaceColors::new(
    Color::from_rgba(226, 88, 34, 255),
    Color::from_rgba(226, 108, 34, 255),
    Color::from_rgba(226, 188, 134, 255),
    Color::from_rgba(226, 58, 34, 255),
);

  let mut renderer = Renderer { debug: false };
  let image = load_texture("frames.png").await.expect("frames.png should be loaded").get_texture_data();
  let texture_actor = customize_image(image.sub_image(Rect::new(0., 0., 16. * 3., 16.)), colors_actor);
  let texture_enemy = customize_image(image.sub_image(Rect::new(16. * 3., 0., 16. * 3., 16.)), colors_enemy);
  let texture_fireball = customize_image(image.sub_image(Rect::new(16. * 6., 0., 16. * 4., 16.)), colors_fireball);

  let player_actor = Actor::new(Vec2::new(screen_width() / 2., screen_height() / 2.), 100., 5);
  let player = Player::new(player_actor, 2.);

  let mut ai_actors: Vec<(Actor, Ai)> = vec![];
  for c in 0..ENEMIES_COUNT {
    let x_mod = (c % 12) as f32;
    let y_mod = (c / 12) as f32;
    let actor = Actor::new(Vec2::new(32. + x_mod * 64., 64. + y_mod * 64.), 80., 2);
    let ai = Ai::new(WeightedStates::new_idle_wandering(&[1, 5, 30]));
    ai_actors.push((actor, ai));
  }

  let mut paused = false;
  let mut world = World::new(player).with_ai_actors(ai_actors);

  loop {
    if is_key_pressed(KeyCode::D) {
      renderer.debug = !renderer.debug;
    }

    if is_key_pressed(KeyCode::P) {
      paused = !paused;
    }

    clear_background(DARKGRAY);

    if !paused {
      let delta_t = get_frame_time();
      if is_mouse_button_down(MouseButton::Left) {
        world.on_mouse_button_down(Vec2::from(mouse_position()));
      }

      world.update(delta_t);
    }

    renderer.draw_actor(&texture_actor, &world.get_player().actor);
    for actor in world.get_ai_actors() {
      renderer.draw_actor(&texture_enemy, actor);
    }
    for projectile in world.get_projectiles() {
      renderer.draw_projectile(&texture_fireball, projectile);
    }
    for particle in world.get_particles() {
      renderer.draw_particle(&texture_fireball, particle);
    }

    renderer.draw_debug(&world);

    if paused {
      draw_text("PAUSED", screen_width() / 2. - 40., screen_height() / 2. - 4., 32., WHITE);
    }

    next_frame().await
  }
}