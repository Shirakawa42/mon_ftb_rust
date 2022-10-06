use noise::{NoiseFn, Perlin};

use crate::{chunk::{Cube, Chunk}, items::Items, structures::{generate_tree}};

const SEED: u32 = 0;

#[derive(Clone)]
pub struct ChunkFilling {
    pub noise: Perlin,
}

impl ChunkFilling {
    pub fn new() -> Self {
        Self { noise: Perlin::new(SEED) }
    }

    fn fill_surface(&self, gx: f64, gy: f64, gz: f64, current_chunk: &mut Chunk) -> u16 {
        let noise = self.noise.get([gx / 32.0, gz / 32.0]) * 8.0 + 16.0;
        if gy < noise - 4.0 {
            return Items::Stone as u16;
        }
        if gy < noise - 2.0 {
            return Items::Dirt as u16;
        }
        if gy < noise - 1.0 {
            return Items::Grass as u16;
        }
        if gy < noise {
            if self.noise.get([gx / 4.0 + 256.0, gz / 4.0 + 256.0]) > 0.8 {
                if self.noise.get([gx / 4.0 + 512.0, gz / 4.0 + 512.0]) > 0.8 {
                    generate_tree([gx as i32, gy as i32, gz as i32], current_chunk);
                }
            }
        }
        return Items::Air as u16;
    }

    fn fill_caverns(&self, gx: f64, gy: f64, gz: f64) -> u16 {
        let noise = self.noise.get([gx / 32.0, gy / 32.0, gz / 32.0]);
        if noise > 0.5 {
            return Items::Air as u16;
        }
        return Items::Stone as u16;
    }

    pub fn fill_block(&self, gx: f64, gy: f64, gz: f64, current_chunk: &mut Chunk) -> Cube {
        if gy <= 0.0 {
            return Cube { id: self.fill_caverns(gx, gy, gz) };
        }
        return Cube { id: self.fill_surface(gx, gy, gz, current_chunk) };
    }
}
