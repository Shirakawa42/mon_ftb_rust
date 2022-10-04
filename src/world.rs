use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::chunk::Chunk;
use crate::chunk_filling;
use crate::game_material::GameMaterial;

#[derive(Component)]
pub struct World {
    pub chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>,
    pub material: Handle<GameMaterial>,
    pub chunks_to_draw: Arc<RwLock<Vec<[i32; 3]>>>,
    pub thread_pool: rayon::ThreadPool,
    pub chunk_filling: chunk_filling::ChunkFilling,
}

impl World {
    pub fn new(mut materials: ResMut<Assets<GameMaterial>>, asset_server: Res<AssetServer>) -> Self {
        let chunks = Arc::new(RwLock::new(HashMap::new()));
        let material = materials.add(GameMaterial {
            color: Color::rgb(1.0, 1.0, 1.0),
            color_texture: asset_server.load("Textures/BlockAtlas.png"),
        });
        let chunks_to_draw = Arc::new(RwLock::new(Vec::new()));
        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
        let chunk_filling = chunk_filling::ChunkFilling::new();

        Self {
            chunks,
            material,
            chunks_to_draw,
            thread_pool,
            chunk_filling,
        }
    }

    // called each time player change chunk
    pub fn create_and_fill_chunks(&mut self) {
        for x in -5..5 {
            for y in -5..5 {
                for z in -5..5 {
                    let pos = [x, y, z];

                    if self.chunks.read().unwrap().contains_key(&pos) {
                        continue;
                    }
                    self.chunks.write().unwrap().insert(pos, Arc::new(RwLock::new(Chunk::new(pos))));

                    let chunk = Arc::clone(self.chunks.read().unwrap().get(&pos).unwrap());
                    let chunks_to_draw = Arc::clone(&self.chunks_to_draw);
                    let chunk_filling = self.chunk_filling.clone();

                    self.thread_pool.spawn(move || {
                        chunk.write().unwrap().fill_chunk(&chunk_filling);
                        chunk.write().unwrap().generate_mesh();
                        chunks_to_draw.write().unwrap().push(pos);
                    });
                }
            }
        }
    }
}
