//use std::sync::{Arc, RwLock};
//
//use bevy::utils::HashMap;
//use block_mesh::ndshape::ConstShape;
//
//use crate::chunk::{Chunk, ChunkShape, REAL_CHUNK_SIZE};

#[derive(Clone)]
pub struct Modification {
    pub id: u16,
    pub force: bool,
    pub position: usize,
}

//pub fn add_modification(modification: Modification, world_position: [i32; 3], chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>) {
//    let chunk_position = [world_position[0] / REAL_CHUNK_SIZE as i32, world_position[1] / REAL_CHUNK_SIZE as i32, world_position[2] / REAL_CHUNK_SIZE as i32];
//    if !chunks.read().unwrap().contains_key(&chunk_position) {
//        chunks.write().unwrap().insert(chunk_position, Arc::new(RwLock::new(Chunk::new(chunk_position))));
//    }
//    let chunk = chunks.read().unwrap().get(&chunk_position).unwrap().clone();
//    let [x, y, z] = ChunkShape::delinearize(modification.position as u32);
//    chunk.write().unwrap().modifications.push(modification.clone());
//}
