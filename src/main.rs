use macroquad::prelude::*;
use stage_module::{stage_stack::StageStack, resources::Resources, playing::PlayingStage, main_menu::MainMenu};


mod systems;
mod world_module;
mod display;
mod utils;
mod player;
mod stage_module;


fn window_conf() -> Conf {
  Conf {
      window_title: "kg-g".to_owned(),
      window_width: 640,
      window_height: 480,
      high_dpi: false,
      ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  set_pc_assets_folder("assets");
  let image = load_texture("frames.png").await.expect("frames.png should be loaded").get_texture_data();

  let resources = Resources::new(image);
  let playing_stage = PlayingStage::new(&resources);
  let mainmenu_stage = MainMenu {};
  let mut stage_stack = StageStack::new().with_stages(vec![Box::new(mainmenu_stage), Box::new(playing_stage)]);

  loop {
    if stage_stack.is_empty() {
      break;
    }

    stage_stack.update(&resources);
    stage_stack.draw(&resources);

    #[cfg(debug_assertions)]
    macroquad_profiler::profiler(Default::default());

    next_frame().await
  }
}