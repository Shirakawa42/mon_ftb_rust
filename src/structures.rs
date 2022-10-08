use block_mesh::ndshape::ConstShape;

use crate::{
    chunk::{Chunk, ChunkShape, REAL_CHUNK_SIZE},
    items::Items,
};

pub struct Modification {
    pub id: u16,
    pub force: bool,
    pub position: usize,
}

pub struct LightModification {
    pub light_level: u8,
    pub position: usize,
}

pub fn add_modification(modification: Modification, world_position: [i32; 3], current_chunk: &Chunk) {
    let chunk_position = world_position_to_chunk_position(world_position);

    current_chunk.add_modification_no_update(modification, chunk_position);
}

pub fn world_position_to_chunk_position(world_position: [i32; 3]) -> [i32; 3] {
    [
        f32::floor(world_position[0] as f32 / REAL_CHUNK_SIZE as f32) as i32,
        f32::floor(world_position[1] as f32 / REAL_CHUNK_SIZE as f32) as i32,
        f32::floor(world_position[2] as f32 / REAL_CHUNK_SIZE as f32) as i32,
    ]
}

pub fn world_position_to_position_in_chunk(world_position: [i32; 3]) -> [u32; 3] {
    [
        ((world_position[0]).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1,
        ((world_position[1]).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1,
        ((world_position[2]).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1,
    ]
}

pub fn generate_tree(world_position: [i32; 3], current_chunk: &Chunk) {
    let [x, y, z] = world_position;

    for i in 0..6 {
        add_modification(
            Modification {
                id: Items::Wood as u16,
                force: true,
                position: ChunkShape::linearize(world_position_to_position_in_chunk([x, y + i, z])) as usize,
            },
            [x, y + i, z],
            current_chunk,
        );
    }
    for i in -2..3 {
        for j in -2..3 {
            for k in -1..3 {
                add_modification(
                    Modification {
                        id: Items::Leave as u16,
                        force: false,
                        position: ChunkShape::linearize(world_position_to_position_in_chunk([x + i, y + 6 + k, z + j])) as usize,
                    },
                    [x + i, y + 6 + k, z + j],
                    current_chunk,
                );
            }
        }
    }
}
