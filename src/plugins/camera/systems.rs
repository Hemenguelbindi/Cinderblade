use crate::prelude::*;
use crate::plugins::camera::components::MainCamera;

pub fn initializate_camera(mut commands: Commands){
    commands.spawn(MainCamera);
}
