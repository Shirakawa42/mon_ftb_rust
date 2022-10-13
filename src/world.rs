use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use bevy::utils::HashMap;
use linked_hash_set::LinkedHashSet;

use crate::chunk::Chunk;
use crate::game_material::GameMaterial;
use crate::positions::ChunkPosition;

const NB_THREADS: usize = 8;
const NB_UPDATE_THREADS: usize = 4;

#[derive(Component)]
pub struct World {
    pub chunks: Arc<RwLock<HashMap<ChunkPosition, Arc<RwLock<Chunk>>>>>,
    pub material: RwLock<Handle<GameMaterial>>,
    pub chunks_to_draw: Arc<RwLock<LinkedHashSet<ChunkPosition>>>,
    pub chunks_to_update: Arc<RwLock<LinkedHashSet<ChunkPosition>>>,
    pub thread_pool: rayon::ThreadPool,
    pub update_thread_pool: rayon::ThreadPool,
    pub world_thread_pool: rayon::ThreadPool,
    pub nb_chunks_generating: Arc<RwLock<usize>>,
    pub natural_light_stopped_at: RwLock<HashMap<(i32, i32), i32>>, // key: (gx, gz), value: gy -> the highest y where the light_multiplier is not 0
}

impl World {
    pub fn new() -> Self {
        let chunks = Arc::new(RwLock::new(HashMap::new()));
        let material = RwLock::new(Handle::default());
        let chunks_to_draw = Arc::new(RwLock::new(LinkedHashSet::new()));
        let chunks_to_update = Arc::new(RwLock::new(LinkedHashSet::new()));
        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(NB_THREADS).build().unwrap();
        let update_thread_pool = rayon::ThreadPoolBuilder::new().num_threads(NB_UPDATE_THREADS).build().unwrap();
        let world_thread_pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap();

        Self {
            chunks,
            material,
            chunks_to_draw,
            chunks_to_update,
            thread_pool,
            update_thread_pool,
            world_thread_pool,
            nb_chunks_generating: Arc::new(RwLock::new(0)),
            natural_light_stopped_at: RwLock::new(HashMap::new()),
        }
    }

    // called each frame
    pub fn update_chunks_to_update(&self) {
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

    pub fn start_world(&self, world: Arc<RwLock<World>>) {
        self.world_thread_pool.spawn(move || {
            world.read().unwrap().create_and_fill_chunks(world.clone());
        });
    }

    // called each time player change chunk
    pub fn create_and_fill_chunks(&self, world: Arc<RwLock<World>>) {
        for y in (-4..2).rev() {
            for x in -8..9 {
                for z in -8..9 {
                    let pos = ChunkPosition { x, y, z };

                    {
                        let mut chunks_lock = self.chunks.write().unwrap();
                        if !chunks_lock.contains_key(&pos) {
                            chunks_lock.insert(pos, Arc::new(RwLock::new(Chunk::new(pos, world.clone()))));
                        }
                    }

                    let chunk = Arc::clone(self.chunks.read().unwrap().get(&pos).unwrap());
                    if chunk.read().unwrap().drawn {
                        continue;
                    }
                    let chunks_to_draw = Arc::clone(&self.chunks_to_draw);
                    let nb_chunks_generating = self.nb_chunks_generating.clone();
                    *self.nb_chunks_generating.write().unwrap() += 1;

                    self.thread_pool.spawn(move || {
                        if !*chunk.read().unwrap().filled.read().unwrap() {
                            chunk.read().unwrap().fill_chunk();
                            chunk.read().unwrap().modify_other_chunks();
                        }
                        chunk.write().unwrap().update_mesh();
                        chunks_to_draw.write().unwrap().insert_if_absent(pos);
                        *nb_chunks_generating.write().unwrap() -= 1;
                    });
                }
            }
        }
    }
}
