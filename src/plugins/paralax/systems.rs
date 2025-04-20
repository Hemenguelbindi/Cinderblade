use crate::prelude::*;
use crate::plugins::paralax::components::ParalaxLayer;


pub struct ParallaxConfig<'a> {
    pub image_path: &'a str,
    pub z_index: f32,
    pub speed: f32,
    pub repeat_count: usize,
    pub offset_x: f32,
}


pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scale = Vec3::splat(4.0);
    let layer_width = 1280.0;

    let layers = [
        ParallaxConfig {
            image_path: "map/oak/background/background_layer_1.png",
            z_index: -3.0,
            speed: 10.0,
            repeat_count: 2,
            offset_x: layer_width,
        },
        ParallaxConfig {
            image_path: "map/oak/background/background_layer_2.png",
            z_index: -2.0,
            speed: 15.0,
            repeat_count: 2,
            offset_x: layer_width,
        },
        ParallaxConfig {
            image_path: "map/oak/background/background_layer_3.png",
            z_index: -1.0,
            speed: 20.0,
            repeat_count: 2,
            offset_x: layer_width,
        },
    ];
    
    spawn_parallax_background(&mut commands, &asset_server, &layers, scale);

}


pub fn spawn_parallax_background(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    config: &[ParallaxConfig],
    scale: Vec3,
) {
    for layer in config {
        for i in 0..layer.repeat_count {
            commands.spawn((
                Sprite {
                    image: asset_server.load(&*layer.image_path),
                    ..default()
                },
                Transform::from_xyz(i as f32 * layer.offset_x, 0.0, layer.z_index)
                    .with_scale(scale),
                ParalaxLayer { speed: layer.speed },
            ));
        }
    }
}



pub fn parallax_movement(time: Res<Time>, mut query: Query<(&ParalaxLayer, &mut Transform)>) {
    for (parallax, mut transform) in query.iter_mut() {
        transform.translation.x -= parallax.speed * time.delta_secs();
        if transform.translation.x <= -1280.0 {
            transform.translation.x = 1280.0;
        }
    }
}
