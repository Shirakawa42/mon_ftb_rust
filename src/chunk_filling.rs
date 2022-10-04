use noise::{NoiseFn, Perlin};

use crate::{chunk::Cube, items::Items};

const SEED: u32 = 0;

#[derive(Clone)]
pub struct ChunkFilling {
    pub noise: Perlin,
}

impl ChunkFilling {
    pub fn new() -> Self {
        Self { noise: Perlin::new(SEED) }
    }

    fn fill_surface(&self, gx: f64, gy: f64, gz: f64) -> u16 {
        let noise = self.noise.get([gx / 32.0, gz / 32.0]) * 8.0 + 24.0;
        if gy < noise - 4.0 {
            return Items::Stone as u16;
        }
        if gy < noise - 1.0 {
            return Items::Dirt as u16;
        }
        if gy < noise {
            return Items::Grass as u16;
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

    pub fn fill_block(&self, gx: f64, gy: f64, gz: f64) -> Cube {
        if gy <= 0.0 {
            return Cube { id: self.fill_caverns(gx, gy, gz) };
        }
        return Cube { id: self.fill_surface(gx, gy, gz) };
    }
}
