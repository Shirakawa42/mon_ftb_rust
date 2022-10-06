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

#[rustfmt::skip]
pub fn add_modification(modification: Modification, world_position: [i32; 3], current_chunk: &mut Chunk) {
    let chunk_position = world_position_to_chunk_position(world_position);
    if chunk_position == current_chunk.position {
        current_chunk.modifications.write().unwrap().push(modification);
    } else {
        let difference = [
            current_chunk.position[0] - chunk_position[0],
            current_chunk.position[1] - chunk_position[1],
            current_chunk.position[2] - chunk_position[2],
        ];
        let count_zeros = difference.iter().filter(|&&x| x == 0).count();
        if count_zeros == 2 {
            let is_border = world_position_to_chunk_position([world_position[0] + difference[0], world_position[1] + difference[1], world_position[2] + difference[2]]);
            if is_border == current_chunk.position {
                let [mut x, mut y, mut z] = ChunkShape::delinearize(modification.position as u32);
                x = if difference[0] == 1 { 0 } else if difference[0] == -1 { REAL_CHUNK_SIZE + 1 } else { x };
                y = if difference[1] == 1 { 0 } else if difference[1] == -1 { REAL_CHUNK_SIZE + 1 } else { y };
                z = if difference[2] == 1 { 0 } else if difference[2] == -1 { REAL_CHUNK_SIZE + 1 } else { z };

                current_chunk.modifications.write().unwrap().push(Modification {
                    id: modification.id,
                    force: modification.force,
                    position: ChunkShape::linearize([x, y, z]) as usize,
                });
            }
        }
        current_chunk.other_chunks_modifications.write().unwrap().push((chunk_position, modification));
    }
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

pub fn generate_tree(world_position: [i32; 3], current_chunk: &mut Chunk) {
    let [x, y, z] = world_position;

    for i in 0..6 {
        add_modification(
            Modification {
                id: Items::Wood as u16,
                force: false,
                position: ChunkShape::linearize(world_position_to_position_in_chunk([x, y + i, z])) as usize,
            },
            [x, y + i, z],
            current_chunk,
        );
    }
    for i in -2..3 {
        for j in -2..3 {
            for k in 0..4 {
                add_modification(
                    Modification {
                        id: Items::Leave as u16,
                        force: false,
                        position: ChunkShape::linearize(world_position_to_position_in_chunk([x + i, y + 5 + k, z + j])) as usize,
                    },
                    [x + i, y + 5 + k, z + j],
                    current_chunk,
                );
            }
        }
    }
}
