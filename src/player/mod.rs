mod player;
mod systems;
mod spawn;

use crate::prelude::*;

use spawn::spawn_player;
use systems::{player_jump, player_movement};


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update,(player_movement, player_jump));

    }
}