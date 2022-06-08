use macroquad::{prelude::*, ui::root_ui};

use super::stage_stack::{Stage, StageAction};

pub struct GameOver {
  camera: Camera2D,
  score: usize,
}

impl GameOver {
  pub fn new(score: usize) -> Self {
    Self {
      score,
      camera: Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()))
    }
  }
}

impl Stage for GameOver {
  fn update(&mut self, _resources: &super::resources::Resources) -> Option<StageAction> {
    if root_ui().button(Some(Vec2::new(screen_width() / 2. - 24., screen_height() / 2. - 50.)), "Again") {
      return Some(StageAction::StartGame);
    }

    if root_ui().button(Some(Vec2::new(screen_width() / 2. - 100., screen_height() / 2. + 50.)), "Back to main menu") {
      return Some(StageAction::EndGame);
    }

    None
  }

  fn draw(&self, _resources: &super::resources::Resources) {
    clear_background(DARKGRAY);

    set_camera(&self.camera);

    draw_text(format!("Score: {}", self.score).as_str(), 5., 60., 50., WHITE);
  }
}
