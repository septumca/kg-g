use std::ops::Deref;

use hecs::{Entity, World};
use macroquad::{prelude::*};

use crate::{actor::{State, Position}, timer::Timer, animation::Animation};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AiState {
  Idle,
  Wandering,
  Following(Entity),
  // Attacking
}


pub struct WeightedStates {
  states: Vec<(i32, AiState)>,
  total: i32,
}

fn get_total_weight(states: &Vec<(i32, AiState)>) -> i32 {
  states.into_iter().fold(0, |acc, (w, _)| acc + w)
}

impl WeightedStates {
  pub fn new(states: Vec<(i32, AiState)>) -> Self {
    let total = get_total_weight(&states);
    Self {
      states,
      total
    }
  }

  fn get_state_by_weight(&self, weight: i32) -> AiState {
    let mut result = weight;
    for (w, s) in &self.states {
      result -= w;
      if result <= 0 {
        return *s;
      }
    }

    AiState::Idle
  }

  pub fn get_next_state(&self) -> AiState {
    let weight = rand::gen_range::<i32>(0, self.total);
    self.get_state_by_weight(weight)
  }
}

pub struct Ai {
  weighted_states: WeightedStates,
  timer: Timer
}

impl Ai {
  pub fn new(weighted_states: WeightedStates) -> Self {
    let timer = Timer::new(rand::gen_range::<f32>(0.5, 2.));
    Self {
      timer,
      weighted_states
    }
  }

  fn refresh_timer(&mut self) {
    self.timer = Timer::new(rand::gen_range::<f32>(0.5, 2.))
  }

  pub fn update(&mut self, world: &World, delta_time: f32, animation: &Animation) -> Option<State> {
    if self.timer.update(delta_time) || animation.is_finished() {
      let new_ai_state = self.weighted_states.get_next_state();

      let new_state = match new_ai_state {
        AiState::Following(id) => {
          let mut q = world.query_one::<&Position>(id).unwrap();
          if let Some(pos) = q.get() { State::Walking(pos.deref().0) } else { State::Idle }
        },
        AiState::Wandering => {
          let tp = Vec2::new(rand::gen_range::<f32>(32., screen_width() - 32.), rand::gen_range::<f32>(32., screen_height() - 32.));
          State::Walking(tp)
        },
        _ => State::Idle,
      };
      self.refresh_timer();
      return Some(new_state);
    }

    None
  }
}
