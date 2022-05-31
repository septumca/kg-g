use macroquad::{prelude::*};

use crate::{actor::{Actor}, timer::Timer};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AiState {
  Idle,
  Wandering,
  Following,
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

  pub fn new_idle_wandering(weights: &[i32; 3]) -> Self {
    Self::new(vec![
      (weights[0], AiState::Idle),
      (weights[1], AiState::Wandering),
      (weights[2], AiState::Following),
    ])
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
  state: AiState,
  weighted_states: WeightedStates,
  timer: Timer,
  pub actor_id: usize,
}

impl Ai {
  pub fn new(weighted_states: WeightedStates, actor_id: usize) -> Self {
    let timer = Timer::new(rand::gen_range::<f32>(0.5, 2.));
    Self {
      state: AiState::Idle,
      timer,
      weighted_states,
      actor_id
    }
  }

  fn refresh_timer(&mut self) {
    self.timer = Timer::new(rand::gen_range::<f32>(0.5, 2.))
  }

  pub fn update(&mut self, delta_time: f32, actor: &mut Actor, player_actor: &Actor) {
    if self.timer.update(delta_time) || actor.animation.is_finished() {
      let new_state = self.weighted_states.get_next_state();
      self.state = match new_state {
        AiState::Following => {
          if actor.bound_rect.collide_with(&player_actor.bound_rect) {
            let x = 32.0_f32.max(player_actor.movable.position.x + rand::gen_range::<f32>(-100., 100.)).min(screen_width() - 32.);
            let y = 32.0_f32.max(player_actor.movable.position.y + rand::gen_range::<f32>(-100., 100.)).min(screen_height() - 32.);
            actor.move_to_and_animate(Vec2::new(x, y));
            AiState::Wandering
          } else {
            new_state
          }
        },
        AiState::Wandering => {
          let tp = Vec2::new(rand::gen_range::<f32>(32., screen_width() - 32.), rand::gen_range::<f32>(32., screen_height() - 32.));
          actor.move_to_and_animate(tp);
          new_state
        },
        AiState::Idle => {
          actor.stop();
          new_state
        }
      };
      self.refresh_timer();
    };

    match self.state {
      AiState::Following => {
        let tp = Vec2::new(player_actor.movable.position.x, player_actor.movable.position.y);
        actor.move_to(tp);
      }
      _ => (),
    };
  }
}
