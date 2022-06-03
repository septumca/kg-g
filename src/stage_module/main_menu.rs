use macroquad::{prelude::*, ui::root_ui};

use super::stage_stack::{Stage, StageAction};

pub struct MainMenu {}

impl Stage for MainMenu {
  fn update(&mut self, _resources: &super::resources::Resources) -> Option<StageAction> {
    if root_ui().button(Some(Vec2::new(screen_width() / 2. - 24., screen_height() / 2. - 50.)), "Start") {
      return Some(StageAction::StartGame);
    }

    if root_ui().button(Some(Vec2::new(screen_width() / 2. - 20., screen_height() / 2. + 50.)), "Quit") {
      return Some(StageAction::GameQuit);
    }

    None
  }

  fn draw(&self, _resources: &super::resources::Resources) {
    clear_background(DARKGRAY);

    root_ui().label(Some(Vec2::new(screen_width() / 2.- 20., screen_height() / 2. - 100.)), "kg-g");
  }
}


