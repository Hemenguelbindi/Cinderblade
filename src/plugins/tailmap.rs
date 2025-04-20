use crate::prelude::*;
use serde::Deserialize;
use std::fs;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AngleSpriteSheats>()
            .add_systems(Startup, spawn_map_from_tmj);
    }
}

impl FromWorld for AngleSpriteSheats {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid((32, 32).into(), 15, 15, None, None);
        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .expect("Missing Assets<TextureAtlasLayout>");
        let handle = texture_atlases.add(texture_atlas);
        Self(handle)
    }
}

#[derive(Resource)]
pub struct AngleSpriteSheats(pub Handle<TextureAtlasLayout>);

#[derive(Deserialize)]
struct TiledMap {
    tilewidth: u32,
    tileheight: u32,
    layers: Vec<Layer>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Layer {
    #[serde(rename = "tilelayer")]
    TileLayer {
        data: Vec<u32>,
        width: u32,
        height: u32,
        name: String,
    },
    // Можно добавить objectgroup или imagelayer при необходимости
}

fn spawn_map_from_tmj(
    mut commands: Commands,
    sprite_atlas: Res<AngleSpriteSheats>,
    asset_server: Res<AssetServer>,
) {
    // Загрузка и парсинг TMJ-файла
    let raw = fs::read_to_string("assets/demo.tmj").expect("Can't read TMJ file");
    let map: TiledMap = serde_json::from_str(&raw).expect("Invalid TMJ JSON");

    let tile_texture = asset_server.load("map/oak/oak_woods_tileset.png");

    for layer in map.layers {
        match layer {
            Layer::TileLayer { data, width, height, name, .. } => {
                for (i, gid) in data.iter().enumerate() {
                    if *gid == 0 {
                        continue;
                    }

                    let x = i as u32 % width;
                    let y = height - 1 - (i as u32 / width);

                    let tile_index = gid - 1; // Только если 1 tileset

                    let tile_pos = Vec3::new(
                        x as f32 * map.tilewidth as f32,
                        y as f32 * map.tileheight as f32,
                        match name.as_str() {
                            "background" => 0.0,
                            "foreground" => 10.0,
                            _ => 5.0,
                        },
                    );

                    commands.spawn((
                        Sprite {
                            image: tile_texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: sprite_atlas.0.clone(),
                                index: tile_index as usize,
                            }),
                            ..default()
                        },
                        Transform::from_translation(tile_pos),
                    ));
                }
            }
        }
    }
}