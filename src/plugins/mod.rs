mod paralax;
mod tailmap;
pub mod camera;

use crate::prelude::*;
use paralax::ParalaxPlugin;
use camera::CameraPlugin;


pub struct DevTools;

impl Plugin for DevTools {
    fn build(&self, app: &mut App) {
        app.
        add_plugins(ParalaxPlugin)
        .add_plugins(CameraPlugin);
    }
}
