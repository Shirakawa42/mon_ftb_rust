use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::chunk::Chunk;
use crate::chunk_filling;
use crate::game_material::GameMaterial;

const NB_THREADS: usize = 8;
const NB_UPDATE_THREADS: usize = 4;

#[derive(Component)]
pub struct World {
    pub chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>,
    pub material: Handle<GameMaterial>,
    pub chunks_to_draw: Arc<RwLock<Vec<[i32; 3]>>>,
    pub chunks_to_update: Arc<RwLock<Vec<[i32; 3]>>>,
    pub thread_pool: rayon::ThreadPool,
    pub update_thread_pool: rayon::ThreadPool,
    pub chunk_filling: chunk_filling::ChunkFilling,
}

impl World {
    pub fn new() -> Self {
        let chunks = Arc::new(RwLock::new(HashMap::new()));
        let material = Handle::default();
        let chunks_to_draw = Arc::new(RwLock::new(Vec::new()));
        let chunks_to_update = Arc::new(RwLock::new(Vec::new()));
        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(NB_THREADS).build().unwrap();
        let update_thread_pool = rayon::ThreadPoolBuilder::new().num_threads(NB_UPDATE_THREADS).build().unwrap();
        let chunk_filling = chunk_filling::ChunkFilling::new(chunks.clone());

        Self {
            chunks,
            material,
            chunks_to_draw,
            chunks_to_update,
            thread_pool,
            update_thread_pool,
            chunk_filling,
        }
    }

    // called each frame
    pub fn update_chunks_to_update(&mut self) {
        let chunks = self.chunks.clone();

        while self.chunks_to_update.read().unwrap().len() > 0 {
            let chunk_position = self.chunks_to_update.read().unwrap()[0];
            let chunk = chunks.read().unwrap().get(&chunk_position).unwrap().clone();
            let chunks_to_draw = Arc::clone(&self.chunks_to_draw);
            
            self.update_thread_pool.spawn(move || {
                chunk.write().unwrap().update_mesh();
                chunks_to_draw.write().unwrap().insert(0, chunk_position);
            });
            self.chunks_to_update.write().unwrap().remove(0);
        }
    }

    // called each time player change chunk
    pub fn create_and_fill_chunks(&mut self) {
        for x in -5..5 {
            for y in -3..3 {
                for z in -5..5 {
                    let pos = [x, y, z];

                    if !self.chunks.read().unwrap().contains_key(&pos) {
                        self.chunks.write().unwrap().insert(pos, Arc::new(RwLock::new(Chunk::new(pos))));
                    }
                    let chunk = Arc::clone(self.chunks.read().unwrap().get(&pos).unwrap());
                    if chunk.read().unwrap().drawn {
                        continue;
                    }
                    let chunks_to_draw = Arc::clone(&self.chunks_to_draw);
                    let chunk_filling = self.chunk_filling.clone();

                    self.thread_pool.spawn(move || {
                        if !chunk.read().unwrap().filled {
                            chunk.write().unwrap().fill_chunk(&chunk_filling);
                        }
                        chunk.write().unwrap().update_mesh();
                        chunks_to_draw.write().unwrap().push(pos);
                    });
                }
            }
        }
    }
}
