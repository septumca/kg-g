use macroquad::{prelude::*, rand::ChooseRandom};


const EPSILON: f32 = 0.004;
const DANGER_EPSILON: f32 = 0.5;
const SHIP_SPEED: f32 = 150.;
const SHIP_TURN_SPEED: f32 = 360.;
const PROJECTILE_SPEED: f32 = 500.;

pub enum Collider {
  Ship(Vec2, Vec2, Vec2),
  Asteroid(Vec2, f32, u8),
  Projectile(Rect, Vec2)
}

pub fn line_line_collision(
  x1: f32, y1: f32, x2: f32, y2: f32,
  x3: f32, y3: f32, x4: f32, y4: f32
) -> Option<Vec2> {
  // calculate the direction of the lines
  let ua = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) /
    ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
  let ub = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) /
    ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));

  // if uA and uB are between 0-1, lines segments are colliding, over 0 lines are colliding
  if ua >= 0. && ub >= 0. {
    Some(vec2(x1 + (ua * (x2-x1)), y1 + (ua * (y2-y1))))
  } else {
    None
  }
}

// based on https://stackoverflow.com/questions/23016676/line-segment-and-circle-intersection
pub fn line_circle_intersection(pos: Vec2, r: f32, l1: f32, l2: f32, l3: f32, l4: f32) -> (Option<Vec2>, Option<Vec2>) {
  let dx = l3 - l1;
  let dy = l4 - l2;
  let a = dx * dx + dy * dy;
  let b = 2. * (dx * (l1 - pos.x) + dy * (l2 - pos.y));
  let c = (l1 - pos.x) * (l1 - pos.x) + (l2 - pos.y) * (l2 - pos.y) - r * r;
  let det = b * b - 4. * a * c;

  if a < EPSILON || det < 0. {
    return (None, None)
  }
  if det == 0. {
    let t = -b / (2. * a);
    return (Some(vec2(l1 + t * dx, l2 + t * dy)), None)
  }
  let t1 = (-b + det.sqrt()) / (2. * a);
  let t2 = (-b - det.sqrt()) / (2. * a);
  (
    Some(vec2(l1 + t1 * dx, l2 + t1 * dy)),
    Some(vec2(l1 + t2 * dx, l2 + t2 * dy)),
  )
}

fn is_point_in_radius(pos: Vec2, radius: f32, point: Vec2) -> bool {
  (pos - point).length() < radius
}

fn get_rect_points(rect: &Rect) -> (Vec2, Vec2, Vec2, Vec2) {
  (
    vec2(rect.x, rect.y),
    vec2(rect.x + rect.w, rect.y),
    vec2(rect.x + rect.w, rect.y + rect.h),
    vec2(rect.x, rect.y + rect.h),
  )
}

impl Collider {
  pub fn collide_with(&self, other: &Collider) -> bool {
    match (self, other) {
        (Collider::Ship(v1, v2, v3), Collider::Asteroid(pos, r, _)) |
        (Collider::Asteroid(pos, r, _), Collider::Ship(v1, v2, v3)) => {
          is_point_in_radius(*pos, *r, *v1) ||
          is_point_in_radius(*pos, *r, *v2) ||
          is_point_in_radius(*pos, *r, *v3)
        },
        (Collider::Ship(v1, v2, v3), Collider::Projectile(rect, _)) |
        (Collider::Projectile(rect, _), Collider::Ship(v1, v2, v3)) => {
          rect.contains(*v1) || rect.contains(*v2) || rect.contains(*v3)
        },
        (Collider::Asteroid(pos, r, _), Collider::Projectile(rect, _)) |
        (Collider::Projectile(rect, _), Collider::Asteroid(pos, r, _)) => {
          let (v1, v2, v3, v4) = get_rect_points(rect);
          is_point_in_radius(*pos, *r, v1) ||
          is_point_in_radius(*pos, *r, v2) ||
          is_point_in_radius(*pos, *r, v3) ||
          is_point_in_radius(*pos, *r, v4)
        },
        (_, _) => false
    }
  }
}

pub fn get_vector_rotation(v: &Vec2) -> f32 {
  v.y.atan2(v.x)
}

pub fn rotate_vec2_by_rad(v: &Vec2, rad: f32) -> Vec2 {
  let c = rad.cos();
  let s = rad.sin();
  vec2(c*v.x - s*v.y, s*v.x + c*v.y)
}


#[derive(Debug)]
pub struct Timer {
  act: f32,
  threshold: f32,
  just_over: bool,
  repeat: bool,
}

impl Timer {
  pub fn new(threshold: f32) -> Self {
    Self { act: 0., threshold, repeat: true, just_over: false }
  }

  pub fn new_timeout(threshold: f32) -> Self {
    Self { act: 0., threshold, repeat: false, just_over: false }
  }

  pub fn reset(&mut self) {
    self.act = 0.;
    self.just_over = false;
  }

  pub fn is_over(&self) -> bool {
    self.act > self.threshold
  }

  pub fn is_just_over(&self) -> bool {
    self.just_over
  }

  pub fn update(&mut self, dt: f32) {
    if self.is_over() && !self.repeat {
      return;
    }
    let updated_time = self.act + dt;
    let over_threshold = updated_time > self.threshold;

    if self.just_over && !over_threshold {
      self.just_over = false;
    } else if over_threshold && !self.just_over {
      self.just_over = true;
    }
    self.act = if over_threshold && self.repeat { 0. } else { updated_time };
  }
}

#[derive(Debug)]
pub struct Movable {
  pos: Vec2,
  vel: Vec2,
  rot: f32,
}

impl Movable {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      vel: Vec2::ZERO,
      rot: 0.,
    }
  }

  pub fn change_speed(&mut self, delta_speed: f32) {
    let vel = rotate_vec2_by_rad(&vec2(1., 0.), self.rot) * delta_speed;
    self.vel += vel;
  }

  pub fn change_rot(&mut self, rot: f32) {
    self.rot += rot;
  }

  fn wrap_around(&mut self) {
    if self.pos.x > screen_width() {
      self.pos.x = 0.;
    }
    if self.pos.x < 0. {
      self.pos.x = screen_width();
    }
    if self.pos.y > screen_height() {
      self.pos.y = 0.;
    }
    if self.pos.y < 0. {
      self.pos.y = screen_height();
    }
  }

  pub fn update(&mut self, dt: f32) {
    self.pos += self.vel * dt;
  }
}

pub trait Updatable {
  fn update(&mut self, dt: f32);
}

pub trait Drawable {
  fn draw(&self);
}

pub trait Collidable {
  fn collider(&self) -> Collider;
}

pub trait CleanupAble {
  fn kill(&mut self);
  fn is_alive(&self) -> bool;
}

pub trait GameObject: Updatable + Drawable + CleanupAble + Collidable {}

pub struct Ship {
  mov: Movable,
  health: isize,
  shoot_cd: Timer,
}

impl Ship {
  pub fn new(pos: Vec2) -> Self {
    Self {
      mov: Movable::new(pos),
      health: 1,
      shoot_cd: Timer::new_timeout(0.5),
    }
  }

  pub fn throttle_up(&mut self, dt: f32) {
    self.mov.change_speed(SHIP_SPEED * dt);
  }

  pub fn stop(&mut self, dt: f32) {
    self.mov.vel -= self.mov.vel.normalize_or_zero() * SHIP_SPEED * dt;
    if self.mov.vel.length_squared() < EPSILON {
      self.mov.vel = Vec2::ZERO;
    }
  }

  pub fn turn_left(&mut self, dt: f32) {
    self.mov.change_rot(-SHIP_TURN_SPEED.to_radians() * dt);
  }

  pub fn turn_right(&mut self, dt: f32) {
    self.mov.change_rot(SHIP_TURN_SPEED.to_radians() * dt);
  }

  pub fn shoot(&mut self) -> Option<Projectile> {
    if self.shoot_cd.is_over() {
      self.shoot_cd.reset();
      Some(Projectile::new(self.mov.pos, self.mov.rot, self.mov.vel))
    } else {
      None
    }
  }

  fn points(&self) -> (Vec2, Vec2, Vec2) {
    let v = vec2(10., 0.);
    (
      self.mov.pos + rotate_vec2_by_rad(&v, self.mov.rot),
      self.mov.pos + rotate_vec2_by_rad(&v, self.mov.rot + 135_f32.to_radians()),
      self.mov.pos + rotate_vec2_by_rad(&v, self.mov.rot - 135_f32.to_radians()),
    )
  }
}

impl Updatable for Ship {
  fn update(&mut self, dt: f32) {
    self.shoot_cd.update(dt);
    self.mov.update(dt);
    self.mov.wrap_around();
  }
}

impl CleanupAble for Ship {
  fn kill(&mut self) {
    self.health -= 1;
  }

  fn is_alive(&self) -> bool {
    self.health > 0
  }
}

impl Drawable for Ship {
  fn draw(&self) {
    let (v1, v2, v3) = self.points();
    draw_triangle_lines(v1, v2, v3, 2., WHITE);
    draw_text(&format!("pos: [{:.1},{:.1}]", self.mov.pos.x, self.mov.pos.y), self.mov.pos.x - 10., self.mov.pos.y - 36., 12., WHITE);
    draw_text(&format!("vel: [{:.1},{:.1}]", self.mov.vel.x, self.mov.vel.y), self.mov.pos.x - 10., self.mov.pos.y - 24., 12., WHITE);
    draw_text(&format!("rot: {:.3}", self.mov.rot), self.mov.pos.x - 10., self.mov.pos.y - 12., 12., WHITE);
  }
}

impl Collidable for Ship {
  fn collider(&self) -> Collider {
    let (v1, v2, v3) = self.points();
    Collider::Ship(v1, v2, v3)
  }
}

impl GameObject for Ship {}

pub struct Asteroid {
  mov: Movable,
  sides: u8,
  radius: f32,
  is_alive: bool,
  dangerous: Option<f32>,
}

impl Asteroid {
  pub fn new_random(pos: Vec2) -> Self {
    let sides = rand::gen_range::<u8>(4, 10);
    let rot = *vec![
      rand::gen_range::<f32>(15., 75.),
      rand::gen_range::<f32>(105., 165.),
      rand::gen_range::<f32>(-75., -15.),
      rand::gen_range::<f32>(-165., -105.)
    ].choose().expect("should be able to choose random rotation for asteroid");
    Self::new_with_movement(
      pos,
      sides as f32 * rand::gen_range(8., 16.),
      sides,
      rot,
      rand::gen_range(10., 100.)
    )
  }

  pub fn new_with_movement(pos: Vec2, radius: f32, sides: u8, rot: f32, speed: f32) -> Self {
    let mut a = Self {
      mov: Movable::new(pos),
      sides,
      radius,
      is_alive: true,
      dangerous: None,
    };
    a.mov.change_rot(rot.to_radians());
    a.mov.change_speed(speed);
    a
  }

  fn get_eta_to_movable(&self, mov: &Movable, step: f32, step_count: i32) -> Option<f32> {
    //calculate line between asteroid pos + vel * 10 and ship pos + vel * 10 + vel.norm * asteroid radius
    //find intersection - decide how much time would asteroid get to got to intersection,
    //and ship to ship pos + vel * 10 position
    //then compare times ==> PROFIT?

    //when ship doesnt have any velocity, then calculate how much the asteroid would take to contain ship position (how?)


    for i in 1..=step_count {
      let mov_pos = mov.pos + mov.vel * step * i as f32;
      let ast_pos = self.mov.pos + self.mov.vel * step * i as f32;

      if mov_pos.distance(ast_pos) < self.radius + 7. {
        return Some(step * i as f32);
      }
    }

    None
  }

  pub fn determine_danger(&mut self, ship: &Ship) {
    self.dangerous = self.get_eta_to_movable(&ship.mov, 0.5, 10);
  }
}

impl Updatable for Asteroid {
  fn update(&mut self, dt: f32) {
    self.mov.update(dt);
    self.mov.wrap_around();
  }
}

impl Drawable for Asteroid {
  fn draw(&self) {
    let color = if self.dangerous.is_some() { RED } else { GRAY };
    draw_poly_lines(self.mov.pos.x, self.mov.pos.y, self.sides, self.radius, self.mov.rot, 2., color);
    if let Some(eta) = self.dangerous {
      draw_text(
        &format!("ETA: {:.1}", eta),
        self.mov.pos.x - self.radius / 2., self.mov.pos.y - self.radius - 20.,
        24., RED
      );
    }
  }
}

impl CleanupAble for Asteroid {
  fn kill(&mut self) {
    self.is_alive = false;
  }

  fn is_alive(&self) -> bool {
    self.is_alive
  }
}

impl Collidable for Asteroid {
  fn collider(&self) -> Collider {
    Collider::Asteroid(self.mov.pos.clone(), self.radius - 2., self.sides)
  }
}

impl GameObject for Asteroid {}

pub struct Projectile {
  mov: Movable,
  has_collided: bool,
}

impl Projectile {
  pub fn new(pos: Vec2, rot: f32, vel: Vec2) -> Self {
    let offset = rotate_vec2_by_rad(&vec2(12., 0.), rot);
    let mut mov = Movable::new(pos + offset);
    mov.vel = vel;
    mov.change_rot(rot);
    mov.change_speed(PROJECTILE_SPEED);

    Self {
      mov,
      has_collided: false
    }
  }

  pub fn rect(&self) -> Rect {
    let projectile_size = 4.;
    Rect::new(
      self.mov.pos.x - projectile_size / 2.,
      self.mov.pos.y - projectile_size / 2.,
      projectile_size,
      projectile_size,
    )
  }
}

impl Updatable for Projectile {
  fn update(&mut self, dt: f32) {
    self.mov.update(dt);
  }
}

impl Drawable for Projectile {
  fn draw(&self) {
    let r = self.rect();
    draw_rectangle(r.x, r.y, r.w, r.h, GRAY);
  }
}

impl CleanupAble for Projectile {
  fn kill(&mut self) {
    self.has_collided = true;
  }

  fn is_alive(&self) -> bool {
    !self.has_collided &&
      self.mov.pos.x <= screen_width() &&
      self.mov.pos.x >= 0. &&
      self.mov.pos.y <= screen_height() &&
      self.mov.pos.y >= 0.
  }
}

impl Collidable for Projectile {
  fn collider(&self) -> Collider {
    Collider::Projectile(self.rect(), self.mov.vel.clone())
  }
}

impl GameObject for Projectile {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn line_circle_intersect() {
    let (p1, p2) = line_circle_intersection(
      vec2(2., 1.), 5.,
      -10., 7., 10., 7.
    );

    assert!(p1.is_none());
    assert!(p2.is_none());

    let (p1, p2) = line_circle_intersection(
      vec2(2., 1.), 5.,
      -10., 1., 10., 1.
    );

    assert_eq!(p1, Some(vec2(7., 1.)));
    assert_eq!(p2, Some(vec2(-3., 1.)));
  }

  fn create() -> Timer {
    Timer {
      act: 0.,
      threshold: 0.5,
      repeat: true,
      just_over: false,
    }
  }

  #[test]
  fn update() {
    let mut time = create();
    time.update(0.1);
    assert_eq!(time.act, 0.1);
    time.update(0.1);
    assert_eq!(time.act, 0.2);
    time.update(0.1);
    assert_eq!(time.act, 0.3);
  }

  #[test]
  fn update_over() {
    let mut time = create();
    time.update(0.4);
    assert_eq!(time.act, 0.4);
    time.update(0.1);
    assert_eq!(time.act, 0.5);
    time.update(0.1);
    assert_eq!(time.act, 0.);
  }

  #[test]
  fn update_over_not_repeat() {
    let mut time = create();
    time.repeat = false;
    time.update(0.4);
    assert_eq!(time.act, 0.4);
    time.update(0.1);
    assert_eq!(time.act, 0.5);
    time.update(0.1);
    assert_eq!(time.act, 0.6);
    assert_eq!(time.is_just_over(), true);
    time.update(0.2);
    assert_eq!(time.act, 0.6);
    assert_eq!(time.is_just_over(), true);
  }

  #[test]
  fn update_just_over() {
    let mut time = create();
    time.repeat = true;
    time.update(0.4);
    assert_eq!(time.act, 0.4);
    time.update(0.1);
    assert_eq!(time.act, 0.5);
    time.update(0.1);
    assert_eq!(time.act, 0.);
    assert_eq!(time.is_just_over(), true);
    time.update(0.2);
    assert_eq!(time.act, 0.2);
    assert_eq!(time.is_just_over(), false);
    time.update(0.4);
    assert_eq!(time.act, 0.);
    assert_eq!(time.is_just_over(), true);
    time.update(0.1);
    assert_eq!(time.act, 0.1);
    assert_eq!(time.is_just_over(), false);
    time.update(0.1);
    assert_eq!(time.act, 0.2);
    assert_eq!(time.is_just_over(), false);
  }
}
