use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::chunk::Chunk;
use crate::game_material::GameMaterial;

#[derive(Component)]
pub struct World {
    pub chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>,
    pub material: Handle<GameMaterial>,
    pub chunks_to_draw: Arc<RwLock<Vec<[i32; 3]>>>,
    pub filled_chunks: Arc<RwLock<Vec<[i32; 3]>>>,
    pub thread_pool_fill: rayon::ThreadPool,
    pub thread_pool_mesh: Arc<RwLock<rayon::ThreadPool>>,
}

impl World {
    pub fn new(mut materials: ResMut<Assets<GameMaterial>>, asset_server: Res<AssetServer>) -> Self {
        let chunks = Arc::new(RwLock::new(HashMap::new()));
        let material = materials.add(GameMaterial {
            color: Color::rgb(1.0, 1.0, 1.0),
            color_texture: asset_server.load("Textures/BlockAtlas.png"),
        });
        let chunks_to_draw = Arc::new(RwLock::new(Vec::new()));
        let filled_chunks = Arc::new(RwLock::new(Vec::new()));
        let thread_pool_fill = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
        let thread_pool_mesh = Arc::new(RwLock::new(rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap()));

        Self {
            chunks,
            material,
            chunks_to_draw,
            filled_chunks,
            thread_pool_fill,
            thread_pool_mesh,
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
                    let filled_chunks = Arc::clone(&self.filled_chunks);
                    let chunks = Arc::clone(&self.chunks);

                    self.thread_pool_fill.spawn(move || {
                        chunk.write().unwrap().fill_chunk();
                        filled_chunks.write().unwrap().push(pos);
                        for chunk_pos in filled_chunks.read().unwrap().iter() {
                            //if chunks
                        }
                    });
                }
            }
        }
    }
}
