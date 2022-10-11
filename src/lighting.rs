use block_mesh::ndshape::ConstShape;

use crate::{
    chunk::{Chunk, ChunkShape, CHUNK_SIZE, REAL_CHUNK_SIZE},
    items::ITEMS,
    positions::{world_position_to_chunk_position, world_position_to_position_in_chunk, WorldPosition},
};

pub const MIN_LIGHT_LEVEL: u8 = 10;
const LIGHT_ATTENUATION_CHUNK_START_LEVEL: i32 = 0;
const LIGHT_ATTENUATION_POWER: u8 = 2;
const PROFOUND_CHUNK: i32 = -(256.0 / LIGHT_ATTENUATION_POWER as f64 / (REAL_CHUNK_SIZE as f64)) as i32 + LIGHT_ATTENUATION_CHUNK_START_LEVEL;

pub fn recalculate_natural_light(current_chunk: &Chunk) {
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            cast_natural_light(current_chunk, x, z);
        }
    }
}

fn no_light_column(current_chunk: &Chunk, x: u32, z: u32) {
    let mut cubes_lock = current_chunk.cubes.write().unwrap();
    for y in 1..CHUNK_SIZE - 1 {
        cubes_lock[ChunkShape::linearize([x, y, z]) as usize].natural_light_level = MIN_LIGHT_LEVEL;
    }
}

fn get_light_multiplier_at_world_position(current_chunk: &Chunk, world_position: WorldPosition) -> f32 {
    let chunk_position = world_position_to_chunk_position(world_position);
    let position_in_chunk = ChunkShape::linearize(world_position_to_position_in_chunk(world_position)) as usize;
    if current_chunk.position == chunk_position {
        return ITEMS[current_chunk.cubes.read().unwrap()[position_in_chunk].id as usize].light_multiplier;
    } else if current_chunk.world.read().unwrap().chunks.read().unwrap().contains_key(&chunk_position) {
        let chunk = current_chunk.world.read().unwrap().chunks.read().unwrap().get(&chunk_position).unwrap().clone();
        if *chunk.read().unwrap().filled.read().unwrap() {
            return ITEMS[chunk.read().unwrap().cubes.read().unwrap()[position_in_chunk].id as usize].light_multiplier;
        }
    }
    return ITEMS[current_chunk.world.read().unwrap().chunk_filling.read().unwrap().fill_block(world_position, current_chunk, false).id as usize].light_multiplier;
}

fn cast_natural_light(current_chunk: &Chunk, x: u32, z: u32) {
    if current_chunk.position.y <= PROFOUND_CHUNK {
        return no_light_column(current_chunk, x, z);
    }

    let [gx, gz] = [
        (x as i32 - 1) + (REAL_CHUNK_SIZE as i32 * current_chunk.position.x),
        (z as i32 - 1) + (REAL_CHUNK_SIZE as i32 * current_chunk.position.z),
    ];

    let sky_height = 32;
    let mut light_level = 255.0;
    let min_height = current_chunk.position.y * REAL_CHUNK_SIZE as i32;

    for current_height in (min_height..sky_height).rev() {
        let chunk_position = world_position_to_chunk_position(WorldPosition { x: gx, y: current_height, z: gz });

        light_level *= get_light_multiplier_at_world_position(current_chunk, WorldPosition { x: gx, y: current_height, z: gz });

        if chunk_position.y == current_chunk.position.y {
            current_chunk.cubes.write().unwrap()[ChunkShape::linearize([x, ((current_height).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1, z]) as usize].natural_light_level = light_level as u8;
        } else if light_level <= MIN_LIGHT_LEVEL as f32 {
            return no_light_column(current_chunk, x, z);
        }
    }
}
