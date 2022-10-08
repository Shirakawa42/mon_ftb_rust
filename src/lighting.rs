use std::sync::{Arc, RwLock};

use bevy::utils::HashMap;
use block_mesh::ndshape::ConstShape;

use crate::{
    chunk::{Chunk, ChunkShape, CHUNK_SIZE, REAL_CHUNK_SIZE},
    items::ITEMS,
    structures::{world_position_to_chunk_position, LightModification},
};

pub fn recalculate_natural_light(chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>, current_chunk: &Chunk, sky_heights: &RwLock<[i32; (CHUNK_SIZE * CHUNK_SIZE) as usize]>) {
    for x in 1..CHUNK_SIZE - 1 {
        for z in 1..CHUNK_SIZE - 1 {
            cast_natural_light(chunks.clone(), current_chunk, x, z, sky_heights);
        }
    }
}

fn cast_natural_light(chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>, current_chunk: &Chunk, x: u32, z: u32, sky_heights: &RwLock<[i32; (CHUNK_SIZE * CHUNK_SIZE) as usize]>) {
    let [gx, gz] = [
        (x as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * current_chunk.position[0]) as f64,
        (z as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * current_chunk.position[2]) as f64,
    ];

    let mut start_y = sky_heights.read().unwrap()[((x + (z * CHUNK_SIZE)) as usize)] + 16;
    let mut light_level = 255.0;

    while light_level > 0.0 && start_y > (current_chunk.position[1] * REAL_CHUNK_SIZE as i32 - REAL_CHUNK_SIZE as i32) {
        let chunk_pos = world_position_to_chunk_position([gx as i32, start_y, gz as i32]);
        let mut exist_and_filled = false;
        if chunks.read().unwrap().contains_key(&chunk_pos) {
            let chunk = chunks.read().unwrap().get(&chunk_pos).unwrap().clone();
            exist_and_filled = *chunk.read().unwrap().filled.read().unwrap();
        }
        let mut local_y = (start_y.rem_euclid(REAL_CHUNK_SIZE as i32)) as i32 + 2;
        if exist_and_filled {
            while local_y >= 0 && light_level > 0.0 {
                light_level *= ITEMS[current_chunk.cubes.read().unwrap()[ChunkShape::linearize([x, local_y as u32, z]) as usize].id as usize].light_multiplier;
                if chunk_pos[1] == current_chunk.position[1] {
                    current_chunk.modify_light_at_pos_no_update(
                        LightModification {
                            position: ChunkShape::linearize([x, local_y as u32, z]) as usize,
                            light_level: light_level as u8,
                        },
                        current_chunk.position,
                    );
                }
                local_y -= 1;
                start_y -= 1;
            }
        } else if chunk_pos[1] == current_chunk.position[1] {
            light_level *= ITEMS[current_chunk.cubes.read().unwrap()[ChunkShape::linearize([x, local_y as u32, z]) as usize].id as usize].light_multiplier;
            current_chunk.modify_light_at_pos_no_update(
                LightModification {
                    position: ChunkShape::linearize([x, local_y as u32, z]) as usize,
                    light_level: light_level as u8,
                },
                current_chunk.position,
            );
            start_y -= 1;
        } else {
            let chunk_filling = current_chunk.world.read().unwrap().chunk_filling.clone();
            light_level *= ITEMS[chunk_filling.read().unwrap().fill_block(gx, start_y as f64, gz, current_chunk, false).id as usize].light_multiplier;
            start_y -= 1;
        }
    }
}
