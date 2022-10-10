use block_mesh::ndshape::ConstShape;

use crate::{
    chunk::{Chunk, ChunkShape},
    items::Items,
    positions::{WorldPosition, world_position_to_chunk_position, world_position_to_position_in_chunk},
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

pub fn add_modification(modification: Modification, world_position: WorldPosition, current_chunk: &Chunk) {
    let chunk_position = world_position_to_chunk_position(world_position);

    current_chunk.add_modification_no_update(modification, chunk_position);
}

pub fn generate_tree(world_position: WorldPosition, current_chunk: &Chunk) {
    for i in 0..6 {
        add_modification(
            Modification {
                id: Items::Wood as u16,
                force: true,
                position: ChunkShape::linearize(world_position_to_position_in_chunk(WorldPosition {
                    x: world_position.x,
                    y: world_position.y + i,
                    z: world_position.z,
                })) as usize,
            },
            WorldPosition {
                x: world_position.x,
                y: world_position.y + i,
                z: world_position.z,
            },
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
                        position: ChunkShape::linearize(world_position_to_position_in_chunk(WorldPosition {
                            x: world_position.x + i,
                            y: world_position.y + 6 + k,
                            z: world_position.z + j,
                        })) as usize,
                    },
                    WorldPosition {
                        x: world_position.x + i,
                        y: world_position.y + 6 + k,
                        z: world_position.z + j,
                    },
                    current_chunk,
                );
            }
        }
    }
}
