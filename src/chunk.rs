use crate::{cube_infos::*, items::ITEMS};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
const CHUNK_SIZE: usize = 32;
const TEXTURE_ATLAS_SIZE: f32 = 16.0;
const NORMALIZED: f32 = 1.0 / TEXTURE_ATLAS_SIZE;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Chunk {
    pub cubes: [[[u16; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub position: [i32; 3],
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        let mut cubes = [[[2; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];

        cubes[0][0][0] = 4;
        Self { cubes, position }
    }

    fn generate_uv(texture_id: u16) -> Vec<[f32; 2]> {
        let mut uv: Vec<[f32; 2]> = Vec::new();
        let x = texture_id as f32 / TEXTURE_ATLAS_SIZE;
        let y = texture_id as f32 - (x * TEXTURE_ATLAS_SIZE);

        uv.push([x + NORMALIZED, y + NORMALIZED]);
        uv.push([x + NORMALIZED, y]);
        uv.push([x, y + NORMALIZED]);
        uv.push([x, y]);
        return uv;
    }

    fn add_usize_3_to_f32_3(a: [usize; 3], b: [f32; 3]) -> Vec<[f32; 3]> {
        vec![[a[0] as f32 + b[0], a[1] as f32 + b[1], a[2] as f32 + b[2]]]
    }

    fn check_side(&self, x: i32, y: i32, z: i32) -> bool {
        if (x < 0 || x >= CHUNK_SIZE as i32)
            || (y < 0 || y >= CHUNK_SIZE as i32)
            || (z < 0 || z >= CHUNK_SIZE as i32)
        {
            return true;
        }
        if ITEMS[self.cubes[x as usize][y as usize][z as usize] as usize].is_opaque == false {
            return true;
        }
        return false;
    }

    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut uv: Vec<[f32; 2]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertex_index = 0;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.cubes[x][y][z] == 0 {
                        continue;
                    }
                    for j in 0..6 {
                        if self.check_side(
                            x as i32 + CUBE_FACE_CHECKS[j][0],
                            y as i32 + CUBE_FACE_CHECKS[j][1],
                            z as i32 + CUBE_FACE_CHECKS[j][2],
                        ) {
                            vertices.append(&mut Chunk::add_usize_3_to_f32_3(
                                [x, y, z],
                                CUBE_VERTICES[CUBE_INDICES[j][0]],
                            ));
                            vertices.append(&mut Chunk::add_usize_3_to_f32_3(
                                [x, y, z],
                                CUBE_VERTICES[CUBE_INDICES[j][1]],
                            ));
                            vertices.append(&mut Chunk::add_usize_3_to_f32_3(
                                [x, y, z],
                                CUBE_VERTICES[CUBE_INDICES[j][2]],
                            ));
                            vertices.append(&mut Chunk::add_usize_3_to_f32_3(
                                [x, y, z],
                                CUBE_VERTICES[CUBE_INDICES[j][3]],
                            ));

                            normals.append(&mut vec![CUBE_NORMALS[j]]);
                            normals.append(&mut vec![CUBE_NORMALS[j]]);
                            normals.append(&mut vec![CUBE_NORMALS[j]]);
                            normals.append(&mut vec![CUBE_NORMALS[j]]);

                            uv.append(
                                &mut Chunk::generate_uv(
                                    ITEMS[self.cubes[x as usize][y as usize][z as usize] as usize]
                                        .textures[j],
                                )
                                .to_vec(),
                            );

                            indices.append(&mut vec![
                                vertex_index as u32,
                                vertex_index as u32 + 1,
                                vertex_index as u32 + 2,
                                vertex_index as u32 + 2,
                                vertex_index as u32 + 1,
                                vertex_index as u32 + 3,
                            ]);

                            vertex_index += 4;
                        }
                    }
                }
            }
        }
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        return mesh;
    }
}
