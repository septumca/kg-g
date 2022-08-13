use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use utils::{Ship, Asteroid, Timer, GameObject, Collider, get_vector_rotation, update_camera};
use std::cell::RefCell;
use std::rc::{Rc, Weak};


mod utils;

fn window_conf() -> Conf {
  Conf {
      window_title: "kg-g".to_owned(),
      window_width: 1280,
      window_height: 860,
      high_dpi: false,
      ..Default::default()
  }
}

fn spawn_random_asteroid() -> Rc<RefCell<Asteroid>> {
  let locations = vec![
    vec2(0., rand::gen_range(0., screen_height())),
    vec2(screen_width(), rand::gen_range(0., screen_height())),
    vec2(rand::gen_range(0., screen_width()), 0.),
    vec2(rand::gen_range(0., screen_width()), screen_height()),
  ];

  Rc::new(RefCell::new(Asteroid::new_random(*locations.choose().expect("should be able to choose spawn location for asteroid"))))
}

fn asteroid_on_hit(pos: Vec2, radius: f32, sides: u8, rota: f32, speeda: f32, rotb: f32, speedb: f32) -> (Rc<RefCell<Asteroid>>, Rc<RefCell<Asteroid>>) {
  let aa = Asteroid::new_with_movement(pos, radius, sides, rota, speeda);
  let ab = Asteroid::new_with_movement(pos, radius, sides, rotb, speedb);
  (
    Rc::new(RefCell::new(aa)),
    Rc::new(RefCell::new(ab)),
  )
}

#[macroquad::main(window_conf)]
async fn main() {
  set_pc_assets_folder("assets");
  let player = Rc::new(RefCell::new(Ship::new(vec2(screen_width() / 2., screen_height() / 2.))));
  let mut asteroid_spawn_timer = Timer::new(5.);
  let mut c = Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));

  let mut asteroids: Vec<Weak<RefCell<Asteroid>>> = vec![];
  let mut game_objects: Vec<Rc<RefCell<dyn GameObject>>> = vec![player.clone()];

  for _ in 0..5 {
    let a = spawn_random_asteroid();
    asteroids.push(Rc::downgrade(&a));
    game_objects.push(a);
  }

  let mut paused = false;

  loop {
    let dt = get_frame_time();
    clear_background(BLACK);

    game_objects.retain(|o| o.borrow().is_alive());
    asteroids.retain(|a| a.strong_count() > 0);

    if is_key_released(KeyCode::P) {
      paused = !paused;
      println!("PAUSED: {}", paused);
    }

    {
      let mut player = player.borrow_mut();
      update_camera(&player, &mut c);
      set_camera(&c);

      if is_key_down(KeyCode::W) {
        player.throttle_up(dt);
      }
      if is_key_down(KeyCode::S) {
        player.stop(dt);
      }
      if is_key_down(KeyCode::A) {
        player.turn_left(dt);
      }
      if is_key_down(KeyCode::D) {
        player.turn_right(dt);
      }
      if is_key_down(KeyCode::Space) {
        if let Some(p) = player.shoot() {
          game_objects.push(Rc::new(RefCell::new(p)));
        }
      }
    }

    if !paused {
      for e in &game_objects {
        e.borrow_mut().update(dt);
      }

      // asteroid_spawn_timer.update(dt);
      if asteroid_spawn_timer.is_just_over() {
        let a = spawn_random_asteroid();
        asteroids.push(Rc::downgrade(&a));
        game_objects.push(a);
      }

      {
        let mut new_game_objects: Vec<Rc<RefCell<dyn GameObject>>> = vec![];
        for ia in 0..game_objects.len() {
          for ib in (ia+1)..game_objects.len() {
            let mut goa = game_objects[ia].borrow_mut();
            let mut gob = game_objects[ib].borrow_mut();
            let ca = goa.collider();
            let cb = gob.collider();
            if ca.collide_with(&cb) {
              goa.kill();
              gob.kill();
              match (ca, cb) {
                (Collider::Projectile(_, vel), Collider::Asteroid(pos, radius, sides)) |
                (Collider::Asteroid(pos, radius, sides), Collider::Projectile(_, vel)) =>
                {
                  if sides > 4 && radius > 20. {
                    let rot = get_vector_rotation(&vel).to_degrees();
                    let (aa, ab) = asteroid_on_hit(
                      pos, radius / 2., sides - 1,
                      rot + rand::gen_range(30., 120.), rand::gen_range(40., 90.),
                      rot - rand::gen_range(30., 120.), rand::gen_range(40., 90.)
                    );
                    asteroids.push(Rc::downgrade(&aa));
                    asteroids.push(Rc::downgrade(&ab));
                    new_game_objects.push(aa);
                    new_game_objects.push(ab);
                  }
                },
                _ => {},
              }
            }
          }
        }
        game_objects.extend(new_game_objects);
      }

      for a in &asteroids {
        if let Some(a) = a.upgrade() {
          a.borrow_mut().determine_danger(&player.borrow());
        }
      }
    }

    for e in &game_objects {
      e.borrow().draw();
    }
    // draw_text(&format!("Game Objects: {}, asteroid: {}, paused: {}", game_objects.len(), asteroids.len(), paused), 2., screen_height() - 24., 22., WHITE);

    #[cfg(debug_assertions)]
    macroquad_profiler::profiler(Default::default());

    next_frame().await
  }
}