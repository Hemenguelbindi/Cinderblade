use crate::prelude::*;
use crate::player::player::Player;

const BALL_FIGURE_SIZE: f32 = 15.;
const BALL_FIGURE_RADIUS: f32 = 5.;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    println!("Spawn capsul...");

    let shape = Capsule2d::new(BALL_FIGURE_RADIUS, BALL_FIGURE_SIZE);
    let color = Color::srgb(0., 1., 0.);

    let mesh = meshes.add(shape);
    let material = materials.add(color);
    
    commands.spawn((
        Player{
            speed: 200.,
            jump_force: 400.,
            is_grounded: false,
        },
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0., 0., 0.)
    ));
   
}