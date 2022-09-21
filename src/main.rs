mod chunk;
mod cube_infos;
mod items;
mod world;
mod game_material;

use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use chunk::Chunk;
use game_material::GameMaterial;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

fn spawn_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GameMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let chunk = Chunk::new([0, 0, 0]);
    let mesh = chunk.generate_mesh();

    let material = materials.add(GameMaterial {
        color: Color::rgb(1.0, 1.0, 1.0),
        color_texture: asset_server.load("Textures/BlockAtlas.png"),
    });

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "FTB".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_chunk)
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<GameMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .run();
}
