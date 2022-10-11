mod chunk;
mod chunk_filling;
mod game_material;
mod greedy_meshing_inits;
mod items;
mod lighting;
mod positions;
mod structures;
mod world;

use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

use bevy::{
    asset::LoadState,
    diagnostic::LogDiagnosticsPlugin,
    input::{keyboard::KeyboardInput, ButtonState},
    pbr::wireframe::{WireframeConfig, WireframePlugin},
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = false;

    let world = Arc::new(RwLock::new(world::World::new()));
    world.read().unwrap().start_world(world.clone());
    commands.insert_resource(world);

    commands.insert_resource(LoadingTexture {
        is_loaded: false,
        handle: asset_server.load("Textures/BlockAtlas.png"),
    });
    commands.insert_resource(Instant::now());
}

fn create_material(mut images: ResMut<Assets<Image>>, world: ResMut<Arc<RwLock<world::World>>>, mut materials: ResMut<Assets<GameMaterial>>, mut loading_texture: ResMut<LoadingTexture>, asset_server: Res<AssetServer>) {
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
    *world.read().unwrap().material.write().unwrap() = material_handle;
}

fn draw_chunks_to_draw(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, world: ResMut<Arc<RwLock<world::World>>>, loading_texture: Res<LoadingTexture>, time: Res<Instant>) {
    if !loading_texture.is_loaded {
        return;
    }
    let chunks_to_draw = world.read().unwrap().chunks_to_draw.clone();
    for _ in 0..CHUNK_PER_FRAME {
        if chunks_to_draw.read().unwrap().len() >= 1 {
            let pos = chunks_to_draw.write().unwrap().pop_front().unwrap();
            let chunks = world.read().unwrap().chunks.clone();
            let chunk = chunks.read().unwrap().get(&pos).unwrap().clone();
            chunk.write().unwrap().draw_mesh(&mut commands, &mut meshes, world.read().unwrap().material.read().unwrap().clone());
        } else {
            break;
        }
    }
    if *world.read().unwrap().nb_chunks_generating.read().unwrap() == 0 {
        println!("All chunks generated in {} ms", time.elapsed().as_millis());
        *world.read().unwrap().nb_chunks_generating.write().unwrap() = 1;
    }
}

fn update_chunks_to_update(world: ResMut<Arc<RwLock<world::World>>>) {
    world.read().unwrap().update_chunks_to_update();
}

fn force_update_all_chunks(world: ResMut<Arc<RwLock<world::World>>>, mut keys: EventReader<KeyboardInput>) {
    for key in keys.iter() {
        if key.key_code == Some(KeyCode::F) && key.state == ButtonState::Pressed {
            println!("updating");
            world.read().unwrap().chunks.write().unwrap().iter_mut().for_each(|(_, chunk)| {
                if chunk.read().unwrap().drawn {
                    let chunks_to_update = world.read().unwrap().chunks_to_update.clone();
                    chunks_to_update.write().unwrap().insert(chunk.read().unwrap().position);
                }
            });
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
        .add_system(update_chunks_to_update)
        .add_system(force_update_all_chunks)
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<GameMaterial>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WireframePlugin::default())
        .run();
}
