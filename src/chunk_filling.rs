use noise::{NoiseFn, Perlin};

use crate::{
    chunk::{Chunk, Cube},
    items::Items,
    structures::generate_tree,
};

const SEED: u32 = 0;

pub struct ChunkFilling {
    pub noise: Perlin,
}

impl ChunkFilling {
    pub fn new() -> Self {
        Self { noise: Perlin::new(SEED) }
    }

    fn fill_surface(&self, gx: f64, gy: f64, gz: f64, current_chunk: &Chunk, generate_structure: bool) -> u16 {
        let noise = self.noise.get([gx / 32.0, gz / 32.0]) * 16.0 + 16.0;
        if gy < noise {
            return Items::Sand as u16;
        } else if gy < noise + 1.0 && generate_structure {
            if self.noise.get([gx / 1.12 + 128.25, gz / 1.12 + 128.25]) > 0.25 {
                if self.noise.get([gx / 1.1 + 256.0, gz / 1.1 + 256.0]) > 0.5 {
                    generate_tree([gx as i32, gy as i32, gz as i32], current_chunk)
                }
            }
        }
        return Items::Air as u16;
    }

    #[allow(unused_variables)]
    fn fill_caverns(&self, gx: f64, gy: f64, gz: f64, generate_structure: bool) -> u16 {
        let noise = self.noise.get([gx / 32.0, gy / 32.0, gz / 32.0]);
        if noise > 0.5 {
            return Items::Air as u16;
        }
        return Items::Stone as u16;
    }

    pub fn fill_block(&self, gx: f64, gy: f64, gz: f64, current_chunk: &Chunk, generate_structure: bool) -> Cube {
        if gy <= 0.0 {
            return Cube {
                id: self.fill_caverns(gx, gy, gz, generate_structure),
                light_level: 10,
            };
        }
        return Cube {
            id: self.fill_surface(gx, gy, gz, current_chunk, generate_structure),
            light_level: 10,
        };
    }
}
