use crate::prelude::*;
use crate::player::PlayerPlugin;
use crate::states::GameState;
use crate::ui::MainMenuPlugin;
use crate::plugins::camera::CameraPlugin;

pub fn build_app() -> App {
    let mut app = App::new();

    app
    .add_plugins(DefaultPlugins)
    .init_state::<GameState>()
    .add_plugins(CameraPlugin)
    .add_plugins(MainMenuPlugin)
    .add_plugins(PlayerPlugin);


    app
}
