use crate::chunk::REAL_CHUNK_SIZE;

pub fn world_position_to_chunk_position(world_position: WorldPosition) -> ChunkPosition {
    ChunkPosition {
        x: f32::floor(world_position.x as f32 / REAL_CHUNK_SIZE as f32) as i32,
        y: f32::floor(world_position.y as f32 / REAL_CHUNK_SIZE as f32) as i32,
        z: f32::floor(world_position.z as f32 / REAL_CHUNK_SIZE as f32) as i32,
    }
}

pub fn world_position_to_position_in_chunk(world_position: WorldPosition) -> [u32; 3] {
    [
        ((world_position.x).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1,
        ((world_position.y).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1,
        ((world_position.z).rem_euclid(REAL_CHUNK_SIZE as i32)) as u32 + 1,
    ]
}

pub fn to_world_position(value: u32, chunk_position_axis: i32) -> i32 {
    value as i32 - 1 + chunk_position_axis * REAL_CHUNK_SIZE as i32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}