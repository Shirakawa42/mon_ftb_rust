mod chunk;
mod cube_infos;
mod game_material;
mod items;
mod world;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
};
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use game_material::GameMaterial;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;
const CHUNK_PER_FRAME: usize = 8;

fn create_world(materials: ResMut<Assets<GameMaterial>>, asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut world = world::World::new(materials, asset_server);
    world.create_and_fill_chunks();
    commands.insert_resource(world);
}

fn draw_chunks_to_draw(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, world: ResMut<world::World>) {
    let mut chunks_to_draw = world.chunks_to_draw.write().unwrap();
    for _ in 0..CHUNK_PER_FRAME {
        if chunks_to_draw.len() >= 1 {
            let pos = chunks_to_draw[0];
            world.chunks.read().unwrap().get(&pos).unwrap().write().unwrap().draw_mesh(&mut commands, &mut meshes, world.material.clone());
            chunks_to_draw.remove(0);
        } else {
            break;
        }
    }
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
        .add_startup_system(create_world)
        .add_system(draw_chunks_to_draw)
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<GameMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
