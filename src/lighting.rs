use std::{cmp::max, sync::{RwLockWriteGuard, RwLockReadGuard, RwLock, Arc}};

use bevy::utils::HashMap;
use block_mesh::ndshape::ConstShape;

use crate::{
    chunk::{Chunk, ChunkShape, Cube, CHUNK_SIZE, REAL_CHUNK_SIZE},
    items::ITEMS,
    positions::{world_position_to_chunk_position, world_position_to_position_in_chunk, ChunkPosition, WorldPosition},
};

pub const MIN_LIGHT_LEVEL: u8 = 5;
const LIGHT_ATTENUATION_CHUNK_START_LEVEL: i32 = 0;
const LIGHT_ATTENUATION_POWER: u8 = 2;
const PROFOUND_CHUNK: i32 = -(256.0 / LIGHT_ATTENUATION_POWER as f64 / (REAL_CHUNK_SIZE as f64)) as i32 + LIGHT_ATTENUATION_CHUNK_START_LEVEL;
pub const DIFFUSE_ATTENUATION: u8 = 10;

pub struct NaturalLightModification {
    pub light_level: u8,
    pub position: usize,
}

pub fn recalculate_natural_light(current_chunk: &Chunk) {
    let world_read_lock = current_chunk.world.read().unwrap();
    let chunks_read_lock = world_read_lock.chunks.read().unwrap();
    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            cast_natural_light(current_chunk, x, z, true, &chunks_read_lock);
        }
    }
}

pub fn recalculate_diffuse_light(current_chunk: &Chunk) {
    let mut cubes_lock = current_chunk.cubes.write().unwrap();
    let mut other_chunks_natural_lock = current_chunk.other_chunks_natural_light_modifications.write().unwrap();
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let linearized = ChunkShape::linearize([x, y, z]);
                if cubes_lock[linearized as usize].natural_light_level as u16 > MIN_LIGHT_LEVEL as u16 + DIFFUSE_ATTENUATION as u16 {
                    let light_level = cubes_lock[linearized as usize].natural_light_level;
                    diffuse_light_from_pos(current_chunk, &mut cubes_lock, x, y, z, light_level, true, &mut other_chunks_natural_lock);
                }
            }
        }
    }
}

pub fn diffuse_light_from_pos(
    current_chunk: &Chunk,
    cubes_lock: &mut RwLockWriteGuard<[Cube; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize]>,
    x: u32,
    y: u32,
    z: u32,
    mut parent_light: u8,
    first: bool,
    other_chunks_natural_lock: &mut RwLockWriteGuard<Vec<(NaturalLightModification, ChunkPosition)>>,
) {
    let linearized = ChunkShape::linearize([x, y, z]);
    parent_light = (parent_light as f32 * ITEMS[cubes_lock[linearized as usize].id as usize].light_multiplier) as u8;
    if (cubes_lock[linearized as usize].natural_light_level < parent_light || first) && parent_light > MIN_LIGHT_LEVEL && ITEMS[cubes_lock[linearized as usize].id as usize].is_transparent {
        cubes_lock[linearized as usize].natural_light_level = parent_light;
        if DIFFUSE_ATTENUATION > parent_light {
            return;
        }
        if x > 0 {
            diffuse_light_from_pos(current_chunk, cubes_lock, x - 1, y, z, parent_light - DIFFUSE_ATTENUATION, false, other_chunks_natural_lock);
        }
        if x == 1 {
            other_chunks_natural_lock.push((
                NaturalLightModification {
                    light_level: parent_light - DIFFUSE_ATTENUATION,
                    position: ChunkShape::linearize([CHUNK_SIZE - 1, y, z]) as usize,
                },
                ChunkPosition {
                    x: current_chunk.position.x - 1,
                    y: current_chunk.position.y,
                    z: current_chunk.position.z,
                },
            ));
        }
        if x < CHUNK_SIZE - 1 {
            diffuse_light_from_pos(current_chunk, cubes_lock, x + 1, y, z, parent_light - DIFFUSE_ATTENUATION, false, other_chunks_natural_lock);
        }
        if x == CHUNK_SIZE - 2 {
            other_chunks_natural_lock.push((
                NaturalLightModification {
                    light_level: parent_light - DIFFUSE_ATTENUATION,
                    position: ChunkShape::linearize([0, y, z]) as usize,
                },
                ChunkPosition {
                    x: current_chunk.position.x + 1,
                    y: current_chunk.position.y,
                    z: current_chunk.position.z,
                },
            ));
        }
        if y > 0 {
            diffuse_light_from_pos(current_chunk, cubes_lock, x, y - 1, z, parent_light - DIFFUSE_ATTENUATION, false, other_chunks_natural_lock);
        }
        if y == 1 {
            other_chunks_natural_lock.push((
                NaturalLightModification {
                    light_level: parent_light - DIFFUSE_ATTENUATION,
                    position: ChunkShape::linearize([x, CHUNK_SIZE - 1, z]) as usize,
                },
                ChunkPosition {
                    x: current_chunk.position.x,
                    y: current_chunk.position.y - 1,
                    z: current_chunk.position.z,
                },
            ));
        }
        if y < CHUNK_SIZE - 1 {
            diffuse_light_from_pos(current_chunk, cubes_lock, x, y + 1, z, parent_light - DIFFUSE_ATTENUATION, false, other_chunks_natural_lock);
        }
        if y == CHUNK_SIZE - 2 {
            other_chunks_natural_lock.push((
                NaturalLightModification {
                    light_level: parent_light - DIFFUSE_ATTENUATION,
                    position: ChunkShape::linearize([x, 0, z]) as usize,
                },
                ChunkPosition {
                    x: current_chunk.position.x,
                    y: current_chunk.position.y + 1,
                    z: current_chunk.position.z,
                },
            ));
        }
        if z > 0 {
            diffuse_light_from_pos(current_chunk, cubes_lock, x, y, z - 1, parent_light - DIFFUSE_ATTENUATION, false, other_chunks_natural_lock);
        }
        if z == 1 {
            other_chunks_natural_lock.push((
                NaturalLightModification {
                    light_level: parent_light - DIFFUSE_ATTENUATION,
                    position: ChunkShape::linearize([x, y, CHUNK_SIZE - 1]) as usize,
                },
                ChunkPosition {
                    x: current_chunk.position.x,
                    y: current_chunk.position.y,
                    z: current_chunk.position.z - 1,
                },
            ));
        }
        if z < CHUNK_SIZE - 1 {
            diffuse_light_from_pos(current_chunk, cubes_lock, x, y, z + 1, parent_light - DIFFUSE_ATTENUATION, false, other_chunks_natural_lock);
        }
        if z == CHUNK_SIZE - 2 {
            other_chunks_natural_lock.push((
                NaturalLightModification {
                    light_level: parent_light - DIFFUSE_ATTENUATION,
                    position: ChunkShape::linearize([x, y, 0]) as usize,
                },
                ChunkPosition {
                    x: current_chunk.position.x,
                    y: current_chunk.position.y,
                    z: current_chunk.position.z + 1,
                },
            ));
        }
    }
}

fn no_light_column(current_chunk: &Chunk, x: u32, z: u32) {
    let mut cubes_lock = current_chunk.cubes.write().unwrap();
    for y in 1..CHUNK_SIZE - 1 {
        cubes_lock[ChunkShape::linearize([x, y, z]) as usize].natural_light_level = MIN_LIGHT_LEVEL;
    }
}

fn get_light_multiplier_at_world_position(current_chunk: &Chunk, world_position: WorldPosition, chunks_read_lock: &RwLockReadGuard<HashMap<ChunkPosition, Arc<RwLock<Chunk>>>>) -> f32 {
    let chunk_position = world_position_to_chunk_position(world_position);
    let position_in_chunk = ChunkShape::linearize(world_position_to_position_in_chunk(world_position)) as usize;
    if current_chunk.position == chunk_position {
        return ITEMS[current_chunk.cubes.read().unwrap()[position_in_chunk].id as usize].light_multiplier;
    } else if chunks_read_lock.contains_key(&chunk_position) {
        let chunk = chunks_read_lock.get(&chunk_position).unwrap().clone();
        if *chunk.read().unwrap().filled.read().unwrap() {
            return ITEMS[chunk.read().unwrap().cubes.read().unwrap()[position_in_chunk].id as usize].light_multiplier;
        }
    }
    return ITEMS[current_chunk.chunk_filling.fill_block(world_position, current_chunk, false).id as usize].light_multiplier;
}

fn cast_natural_light(current_chunk: &Chunk, x: u32, z: u32, filling_chunk: bool, chunks_read_lock: &RwLockReadGuard<HashMap<ChunkPosition, Arc<RwLock<Chunk>>>>) {
    if current_chunk.position.y <= PROFOUND_CHUNK {
        return no_light_column(current_chunk, x, z);
    }

    let [gx, gz] = [
        (x as i32 - 1) + (REAL_CHUNK_SIZE as i32 * current_chunk.position.x),
        (z as i32 - 1) + (REAL_CHUNK_SIZE as i32 * current_chunk.position.z),
    ];

    let sky_height = *current_chunk.world.read().unwrap().natural_light_stopped_at.read().unwrap().get(&(gx, gz)).unwrap();
    let mut light_level = 255.0;
    let min_height = current_chunk.position.y * REAL_CHUNK_SIZE as i32;

    for current_height in (min_height..sky_height).rev() {
        let chunk_position = world_position_to_chunk_position(WorldPosition { x: gx, y: current_height, z: gz });

        light_level *= get_light_multiplier_at_world_position(current_chunk, WorldPosition { x: gx, y: current_height, z: gz }, chunks_read_lock);

        if chunk_position.y == current_chunk.position.y {
            current_chunk.cubes.write().unwrap()[ChunkShape::linearize([x, ((current_height).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1, z]) as usize].natural_light_level = max(light_level as u8, MIN_LIGHT_LEVEL);
        } else if f32::floor((current_height - 1) as f32 / REAL_CHUNK_SIZE as f32) as i32 == current_chunk.position.y {
            if light_level < MIN_LIGHT_LEVEL as f32 {
                return no_light_column(current_chunk, x, z);
            }
            current_chunk.cubes.write().unwrap()[ChunkShape::linearize([x, CHUNK_SIZE - 1, z]) as usize].natural_light_level = light_level as u8;
        } else if light_level <= MIN_LIGHT_LEVEL as f32 {
            if filling_chunk {
                return;
            }
            return no_light_column(current_chunk, x, z);
        }
    }
}
