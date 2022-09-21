mod chunk;
mod cube_infos;
mod world;
mod items;

use bevy::{
    prelude::*,
    render::{render_resource::Face, texture::ImageSettings},
};
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use chunk::Chunk;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;

fn spawn_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let chunk = Chunk::new([0, 0, 0]);
    let mesh = chunk.generate_mesh();
    let texture_handle = asset_server.load("Textures/BlockAtlas.png");
    let mut material = StandardMaterial::default();
    material.base_color_texture = Some(texture_handle);
    material.cull_mode = Some(Face::Back);
    material.unlit = true;
    material.alpha_mode = AlphaMode::Opaque;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(material),
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
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .run();
}
