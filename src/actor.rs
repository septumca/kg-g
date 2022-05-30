use hecs::{World, Entity};
use macroquad::{prelude::*};

use crate::{animation::Animation};

const SPEED: f32 = 100.;


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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum State {
  Idle,
  Walking(Vec2),
}


pub struct PlayerActor;
pub struct ActorState(State);
pub struct Position(pub Vec2);
pub struct Rotation(pub f32);
pub struct Velocity(Vec2);
pub struct NextActorState(pub State);
pub struct TargetPosition(Option<Vec2>);



pub fn spawn_actor(world: &mut World, texture: Texture2D, position: Vec2) -> Entity {
  world.spawn((
    Position(position),
    Rotation(0.),
    ActorState(State::Idle),
    Velocity(Vec2::ZERO),
    TargetPosition(None),
    texture,
    get_idle_animation(),
  ))
}

pub fn refresh_states(world: &mut World) {
  let mut updated_ids: Vec<Entity> = vec![];
  for (id, (state, velocity, target_position, rotation, position, animation, next_state)) in
    &mut world.query::<(&mut ActorState, &mut Velocity, &mut TargetPosition, &mut Rotation, &Position, &mut Animation, &NextActorState)>()
  {
    state.0 = next_state.0;
    updated_ids.push(id);

    match state.0 {
      State::Idle => {
        *animation = get_idle_animation();
        target_position.0 = None;
        velocity.0 = Vec2::ZERO;
      },
      State::Walking(tp) => {
        target_position.0 = Some(tp);
        velocity.0 = (tp - position.0).normalize() * SPEED;
        rotation.0 = if position.0.x > tp.x { 1. } else { 0. };
        *animation = get_walking_animation();
      }
    };
  }

  for id in updated_ids {
    let _ = world.remove_one::<NextActorState>(id);
  }
}

fn stop(vel: &mut Velocity, tp: &mut TargetPosition, animation: &mut Animation) {
  vel.0 = Vec2::ZERO;
  tp.0 = None;
  animation.set_idle();
}

fn walkto(pos: &mut Position, rotation: &mut Rotation, vel: &mut Velocity, tp: &mut TargetPosition, animation: &mut Animation) {
  vel.0 = (tp - position.0).normalize() * SPEED;
  tp.0 = None;
  animation.set_idle();
}

fn update_position(pos: &mut Position, vel: &Velocity, tp: &TargetPosition, delta: f32) -> bool {
  pos.0 += vel.0 * delta;

  if let Some(tp) = tp.0 {
    if pos.0.distance_squared(tp) < 10.0 {
      pos.0 = tp;
      return true;
    }
  }

  false
}

// pub fn update(world: &mut World, delta: f32) {
//   let mut reached_end: Vec<Entity> = vec![];
//   for (id, (velocity, position, target_position))
//     in &mut world.query::<(&Velocity, &mut Position, &TargetPosition)>()
//   {
//     if let Some(tp) = target_position {
//       position.0 += velocity.0 * delta;

//       if position.0.distance_squared(tp) < 10.0 {
//         position.0 = tp;
//         reached_end.push(id);
//       }
//     }

//   }

//   for id in reached_end {
//     let _ = world.insert_one(id, NextActorState(State::Idle));
//   }
// }


// #[cfg(test)]
// mod tests {
//   use super::*;

//   fn create() -> Actor {
//     Actor::new(Vec2::ZERO, State::Idle)
//   }

//   #[test]
//   fn update() {
//     let mut actor = create();
//     let tp = Vec2::new(6., 6.);
//     let delta_time = 0.01;
//     let delta_v = (tp - Vec2::ZERO).normalize();

//     actor.set_new_state(State::Walking(tp));
//     assert_eq!(actor.state, State::Walking(tp));

//     actor.update(delta_time);
//     assert_eq!(actor.position, Vec2::ZERO + delta_v * SPEED * delta_time);

//     actor.update(delta_time);
//     actor.update(delta_time);
//     actor.update(delta_time);
//     actor.update(delta_time);
//     actor.update(delta_time);
//     actor.update(delta_time);
//     actor.update(delta_time);
//     actor.update(delta_time);
//     actor.update(delta_time);

//     assert_eq!(actor.state, State::Idle);
//     assert_eq!(actor.get_source(), Rect::new(0., 0., 16., 16.));
//   }
// }