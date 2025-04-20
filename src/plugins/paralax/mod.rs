mod components;
mod systems;

use crate::prelude::*;
use systems::{parallax_movement, setup_background};

pub struct ParalaxPlugin;

impl Plugin for ParalaxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .add_systems(Update, parallax_movement);
    }
}