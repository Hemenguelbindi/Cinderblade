mod components;
mod systems;

use crate::prelude::*;
use crate::states::GameState;
use crate::ui::systems::spawn_main_menu;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
    }
}