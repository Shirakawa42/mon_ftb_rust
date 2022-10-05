mod chunk;
mod chunk_filling;
mod game_material;
mod greedy_meshing_inits;
mod items;
mod world;

use bevy::{
    asset::LoadState,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::{
        render_resource::{AddressMode, SamplerDescriptor},
        texture::ImageSettings,
    },
    window::PresentMode,
};
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use game_material::GameMaterial;

const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;
const CHUNK_PER_FRAME: usize = 16;
const TEXTURE_ARRAY_SIZE: u32 = 18;

struct LoadingTexture {
    is_loaded: bool,
    handle: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut world = world::World::new();
    world.create_and_fill_chunks();
    commands.insert_resource(world);

    commands.insert_resource(LoadingTexture {
        is_loaded: false,
        handle: asset_server.load("Textures/BlockAtlas.png"),
    });
}

fn create_material(mut images: ResMut<Assets<Image>>, mut world: ResMut<world::World>, mut materials: ResMut<Assets<GameMaterial>>, mut loading_texture: ResMut<LoadingTexture>, asset_server: Res<AssetServer>) {
    if loading_texture.is_loaded || asset_server.get_load_state(loading_texture.handle.clone()) != LoadState::Loaded {
        return;
    }
    loading_texture.is_loaded = true;
    let image = images.get_mut(&loading_texture.handle).unwrap();
    image.sampler_descriptor = bevy::render::texture::ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        ..Default::default()
    });
    image.reinterpret_stacked_2d_as_array(TEXTURE_ARRAY_SIZE);
    let material_handle = materials.add(GameMaterial {
        array_texture: loading_texture.handle.clone(),
    });
    world.material = material_handle;
}

fn draw_chunks_to_draw(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, world: ResMut<world::World>, loading_texture: Res<LoadingTexture>) {
    if !loading_texture.is_loaded {
        return;
    }
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
            present_mode: PresentMode::Immediate,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_system(create_material)
        .add_system(draw_chunks_to_draw)
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<GameMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
