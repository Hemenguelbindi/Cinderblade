mod components;
mod systems;

use crate::prelude::*;
use systems::initializate_camera;


pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, initializate_camera);
    }
}