use macroquad::prelude::*;

use crate::animation::Animation;

const SPEED: f32 = 100.0;

fn get_idle_animation() -> Animation {
  Animation::new(vec![Rect::new(0., 0., 16., 16.)], false)
}

fn get_walking_animation() -> Animation {
  Animation::new(
    vec![
      Rect::new(0., 0., 16., 16.),
      Rect::new(16., 0., 16., 16.),
      Rect::new(0., 0., 16., 16.),
      Rect::new(32., 0., 16., 16.),
    ],
    true
  )
}

#[derive(PartialEq, Debug)]
pub enum State {
  Idle,
  Walking(Vec2),
}

fn get_animation_by_state(state: &State) -> Animation {
  match state {
      State::Idle => get_idle_animation(),
      State::Walking(_) => get_walking_animation(),
  }
}

pub struct Actor {
  animation: Animation,
  pub position: Vec2,
  state: State,
  pub rotation: f32,
}

impl Actor {
  pub fn new(position: Vec2, state: State) -> Self {
    Self {
      animation: get_animation_by_state(&state),
      position,
      state,
      rotation: 0.
    }
  }

  pub fn get_source(&self) -> Rect {
    self.animation.get_act_frame()
  }

  pub fn set_target_position(&mut self, target_position: Vec2) {
    self.set_new_state(State::Walking(target_position));
    self.rotation = if self.position.x > target_position.x { 1. } else { 0. };
  }

  fn set_new_state(&mut self, state: State) {
    self.state = state;
    self.animation = get_animation_by_state(&self.state);
  }

  fn move_to_target_position(&mut self, delta_time: f32, target_position: Vec2) {
    let delta_v = (target_position - self.position).normalize() * SPEED * delta_time;
    self.position += delta_v;

    if self.position.distance_squared(target_position) < 10.0 {
      self.position = target_position;

      self.set_new_state(State::Idle);
    }
  }

  pub fn update(&mut self, delta_time: f32) {
    self.animation.update(delta_time);

    match self.state {
      State::Idle => (),
      State::Walking(target_position) => self.move_to_target_position(delta_time, target_position),
    };

  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create() -> Actor {
    Actor::new(Vec2::ZERO, State::Idle)
  }

  #[test]
  fn update() {
    let mut actor = create();
    let tp = Vec2::new(6., 6.);
    let delta_time = 0.01;
    let delta_v = (tp - Vec2::ZERO).normalize();

    actor.set_target_position(tp);
    assert_eq!(actor.state, State::Walking(tp));

    actor.update(delta_time);
    assert_eq!(actor.position, Vec2::ZERO + delta_v * SPEED * delta_time);

    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);
    actor.update(delta_time);

    assert_eq!(actor.state, State::Idle);
    assert_eq!(actor.get_source(), Rect::new(0., 0., 16., 16.));
  }
}