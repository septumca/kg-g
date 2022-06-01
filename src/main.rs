use std::collections::{HashMap};

use actor::{Actor};
use ai::{WeightedStates, Ai};
use display::Renderer;
use macroquad::prelude::*;
use player::Player;
use projectile::Projectile;
use utils::{customize_image, ReplaceColors};


mod animation;
mod actor;
mod display;
mod utils;
mod timer;
mod ai;
mod movable;
mod projectile;
mod cd;
mod player;

const ENEMIES_COUNT: usize = 24;

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
    Color::from_rgba(226, 184, 34, 255),
    Color::from_rgba(34, 124, 226, 255),
    Color::from_rgba(255, 255, 255, 255),
);

  let mut renderer = Renderer { debug: false };
  let image = load_texture("frames.png").await.expect("frames.png should be loaded").get_texture_data();
  let texture_actor = customize_image(image.sub_image(Rect::new(0., 0., 16. * 3., 16.)), colors_actor);
  let texture_enemy = customize_image(image.sub_image(Rect::new(16. * 3., 0., 16. * 3., 16.)), colors_enemy);
  let texture_fireball = customize_image(image.sub_image(Rect::new(16. * 6., 0., 16. * 4., 16.)), colors_fireball);

  let player_actor = Actor::new(Vec2::new(screen_width() / 2., screen_height() / 2.), 100.);
  let mut player = Player::new(player_actor, 2.);

  let mut ai_actors: HashMap<usize, Actor> = HashMap::new();
  let mut ai_controllers: HashMap<usize, Ai> = HashMap::new();
  for c in 0..ENEMIES_COUNT {
    let x_mod = (c % 12) as f32;
    let y_mod = (c / 12) as f32;
    let actor = Actor::new(Vec2::new(32. + x_mod * 64., 64. + y_mod * 64.), 80.);
    let ai = Ai::new(WeightedStates::new_idle_wandering(&[1, 5, 30]));
    ai_controllers.insert(actor.get_id(), ai);
    ai_actors.insert(actor.get_id(), actor);
  }

  let mut projectiles: Vec<Projectile> = vec![];
  loop {
    clear_background(DARKGRAY);
    let delta_t = get_frame_time();
    let mut ai_actor_ids_to_remove: Vec<usize> = vec![];

    if is_key_down(KeyCode::D) {
      renderer.debug = !renderer.debug;
    }
    if is_mouse_button_released(MouseButton::Left) {
      player.actor.move_to_and_animate(Vec2::from(mouse_position()));
    }

    for (id, ai) in &mut ai_controllers.iter_mut() {
      if let Some(mut actor) = ai_actors.get_mut(id) {
        ai.update(delta_t, &mut actor, &player.actor);
      }
    }

    player.update(delta_t, &mut projectiles, &ai_actors);
    player.actor.update(delta_t);

    let mut impulses: Vec<(usize, Vec2)> = vec![];
    for actor_a in ai_actors.values() {
      for actor_b in ai_actors.values() {
        if actor_a.get_id() != actor_b.get_id() && actor_a.bound_rect.collide_with(&actor_b.bound_rect) {
          let impuls = (actor_a.movable.position - actor_b.movable.position).normalize() * 120.;
          impulses.push((actor_a.get_id(), impuls));
        }
      }
    }
    for (id, impuls) in impulses {
      if let Some(actor) = ai_actors.get_mut(&id) {
        actor.movable.add_impuls(impuls);
      }
    }
    for actor in ai_actors.values_mut() {
      actor.update(delta_t);
    }

    for projectile in &mut projectiles {
      projectile.update(delta_t);
      if let Some(collided_actor) = ai_actors.values().find(|actor| actor.bound_rect.collide_with(&projectile.bound_rect)) {
        projectile.is_alive = false;
        ai_actor_ids_to_remove.push(collided_actor.get_id());
      }
    }

    renderer.draw_actor(&texture_actor, &player.actor);
    for actor in ai_actors.values() {
      renderer.draw_actor(&texture_enemy, &actor);
    }
    for projectile in &projectiles {
      renderer.draw_projectile(&texture_fireball, projectile);
    }

    Renderer::draw_debug();

    projectiles = projectiles.into_iter().filter(|p| p.is_alive).collect();
    for id in ai_actor_ids_to_remove {
      ai_actors.remove(&id);
      ai_controllers.remove(&id);
    }

    next_frame().await
  }
}