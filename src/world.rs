use bevy::utils::HashMap;
use bevy::prelude::*;

use crate::chunk::Chunk;

#[derive(Component, Default)]
struct World {
    pub chunks: HashMap<[i32; 3], Chunk>,
}
