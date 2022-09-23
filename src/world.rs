use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::chunk::Chunk;
use crate::game_material::GameMaterial;

#[derive(Component, Default)]
pub struct World {
    pub chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>,
    pub material: Handle<GameMaterial>,
    pub chunks_to_draw: Arc<RwLock<Vec<[i32; 3]>>>,
}

impl World {
    pub fn new(
        mut materials: ResMut<Assets<GameMaterial>>,
        asset_server: Res<AssetServer>,
    ) -> Self {
        let chunks = Arc::new(RwLock::new(HashMap::new()));
        let material = materials.add(GameMaterial {
            color: Color::rgb(1.0, 1.0, 1.0),
            color_texture: asset_server.load("Textures/BlockAtlas.png"),
        });
        let chunks_to_draw = Arc::new(RwLock::new(Vec::new()));

        Self {
            chunks,
            material,
            chunks_to_draw,
        }
    }

    pub fn generate_chunks(&mut self) {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();

        for x in -6..6 {
            for y in -6..6 {
                for z in -6..6 {
                    let pos = [x, y, z];

                    self.chunks
                        .write()
                        .unwrap()
                        .insert(pos, Arc::new(RwLock::new(Chunk::new(pos))));

                    let chunk = Arc::clone(self.chunks.read().unwrap().get(&pos).unwrap());
                    let chunks_to_draw = Arc::clone(&self.chunks_to_draw);

                    thread_pool.spawn(move || {
                        chunk.write().unwrap().fill_chunk();
                        chunk.write().unwrap().generate_mesh();
                        chunks_to_draw.write().unwrap().push(pos);
                    });
                }
            }
        }
    }
}
