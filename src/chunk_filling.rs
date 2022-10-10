use noise::{NoiseFn, Perlin};

use crate::{
    chunk::{Chunk, Cube},
    items::Items,
    structures::generate_tree,
    lighting::MIN_LIGHT_LEVEL, positions::WorldPosition,
};

const SEED: u32 = 0;

pub struct ChunkFilling {
    pub noise: Perlin,
}

impl ChunkFilling {
    pub fn new() -> Self {
        Self { noise: Perlin::new(SEED) }
    }

    fn fill_surface(&self, world_position: WorldPosition, current_chunk: &Chunk, generate_structure: bool) -> u16 {
        let [gx, gy, gz] = [world_position.x as f64, world_position.y as f64, world_position.z as f64];

        let noise = self.noise.get([gx / 32.0, gz / 32.0]) * 16.0 + 16.0;
        if gy < noise {
            return Items::Sand as u16;
        } else if gy < noise + 1.0 && generate_structure {
            if self.noise.get([gx / 1.12 + 128.25, gz / 1.12 + 128.25]) > 0.25 {
                if self.noise.get([gx / 1.1 + 256.0, gz / 1.1 + 256.0]) > 0.5 {
                    generate_tree(world_position, current_chunk)
                }
            }
        }
        return Items::Air as u16;
    }

    #[allow(unused_variables)]
    fn fill_caverns(&self, world_position: WorldPosition, generate_structure: bool) -> u16 {
        let [gx, gy, gz] = [world_position.x as f64, world_position.y as f64, world_position.z as f64];

        let noise = self.noise.get([gx / 32.0, gy / 32.0, gz / 32.0]);
        if noise > 0.5 {
            return Items::Air as u16;
        }
        return Items::Stone as u16;
    }

    pub fn fill_block(&self, world_position: WorldPosition, current_chunk: &Chunk, generate_structure: bool) -> Cube {
        if world_position.y <= 0 {
            return Cube {
                id: self.fill_caverns(world_position, generate_structure),
                light_level: MIN_LIGHT_LEVEL,
            };
        }
        return Cube {
            id: self.fill_surface(world_position, current_chunk, generate_structure),
            light_level: MIN_LIGHT_LEVEL,
        };
    }
}
