use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use bevy::utils::{HashMap};
use linked_hash_set::LinkedHashSet;

use crate::chunk::Chunk;
use crate::chunk_filling;
use crate::game_material::GameMaterial;

const NB_THREADS: usize = 8;
const NB_UPDATE_THREADS: usize = 4;

#[derive(Component)]
pub struct World {
    pub chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>,
    pub material: Handle<GameMaterial>,
    pub chunks_to_draw: Arc<RwLock<LinkedHashSet<[i32; 3]>>>,
    pub chunks_to_update: Arc<RwLock<LinkedHashSet<[i32; 3]>>>,
    pub thread_pool: rayon::ThreadPool,
    pub update_thread_pool: rayon::ThreadPool,
    pub chunk_filling: chunk_filling::ChunkFilling,
}

impl World {
    pub fn new() -> Self {
        let chunks = Arc::new(RwLock::new(HashMap::new()));
        let material = Handle::default();
        let chunks_to_draw = Arc::new(RwLock::new(LinkedHashSet::new()));
        let chunks_to_update = Arc::new(RwLock::new(LinkedHashSet::new()));
        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(NB_THREADS).build().unwrap();
        let update_thread_pool = rayon::ThreadPoolBuilder::new().num_threads(NB_UPDATE_THREADS).build().unwrap();
        let chunk_filling = chunk_filling::ChunkFilling::new();

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
            let chunk_position = self.chunks_to_update.write().unwrap().pop_front().unwrap();
            let chunk = chunks.read().unwrap().get(&chunk_position).unwrap().clone();
            let chunks_to_draw = Arc::clone(&self.chunks_to_draw);
            
            self.update_thread_pool.spawn(move || {
                chunk.write().unwrap().update_mesh();
                chunks_to_draw.write().unwrap().insert_if_absent(chunk_position);
            });
        }
    }

    // called each time player change chunk
    pub fn create_and_fill_chunks(&mut self) {
        for x in -16..16 {
            for y in -2..1 {
                for z in -16..16 {
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
                    let chunks = self.chunks.clone();
                    let chunks_to_update = self.chunks_to_update.clone();

                    self.thread_pool.spawn(move || {
                        if !chunk.read().unwrap().filled {
                            chunk.write().unwrap().fill_chunk(&chunk_filling);
                            chunk.read().unwrap().modify_other_chunks(chunks, chunks_to_update);
                        }
                        chunk.write().unwrap().update_mesh();
                        chunks_to_draw.write().unwrap().insert(pos);
                    });
                }
            }
        }
    }
}
