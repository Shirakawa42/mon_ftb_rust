use crate::{chunk_filling, cube_infos::*, game_material::GameMaterial, items::ITEMS};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
const CHUNK_SIZE: usize = 34;
const REAL_CHUNK_SIZE: usize = CHUNK_SIZE - 2;
const TEXTURE_ATLAS_SIZE: f32 = 16.0;
const NORMALIZED: f32 = 1.0 / TEXTURE_ATLAS_SIZE;

#[derive(Component)]
pub struct Chunk {
    pub cubes: [[[u16; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub position: [i32; 3],
    pub indices: Vec<u32>,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub filled: bool,
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        let cubes = [[[2; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        let indices = Vec::new();
        let vertices = Vec::new();
        let normals = Vec::new();
        let uvs = Vec::new();
        let filled = false;

        Self {
            cubes,
            position,
            indices,
            vertices,
            normals,
            uvs,
            filled,
        }
    }

    pub fn fill_chunk(&mut self, chunk_filling: &chunk_filling::ChunkFilling) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    self.cubes[x][y][z] = chunk_filling.fill_block(
                        (x as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[0]) as f64,
                        (y as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[1]) as f64,
                        (z as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[2]) as f64,
                    );
                }
            }
        }
        self.filled = true;
    }

    pub fn draw_mesh(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, material: Handle<GameMaterial>) {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(std::mem::replace(&mut self.indices, Vec::new()))));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, std::mem::replace(&mut self.vertices, Vec::new()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, std::mem::replace(&mut self.uvs, Vec::new()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, std::mem::replace(&mut self.normals, Vec::new()));
        commands.spawn().insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: material.clone(),
            transform: Transform::from_xyz(
                self.position[0] as f32 * REAL_CHUNK_SIZE as f32,
                self.position[1] as f32 * REAL_CHUNK_SIZE as f32,
                self.position[2] as f32 * REAL_CHUNK_SIZE as f32,
            ),
            ..default()
        });
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
        if ITEMS[self.cubes[x as usize][y as usize][z as usize] as usize].is_opaque == false {
            return true;
        }
        return false;
    }

    pub fn generate_mesh(&mut self) {
        let mut vertex_index = 0;

        for x in 1..CHUNK_SIZE - 1 {
            for y in 1..CHUNK_SIZE - 1 {
                for z in 1..CHUNK_SIZE - 1 {
                    if self.cubes[x][y][z] == 0 {
                        continue;
                    }
                    for j in 0..6 {
                        if self.check_side(x as i32 + CUBE_FACE_CHECKS[j][0], y as i32 + CUBE_FACE_CHECKS[j][1], z as i32 + CUBE_FACE_CHECKS[j][2]) {
                            self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][0]]));
                            self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][1]]));
                            self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][2]]));
                            self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][3]]));

                            self.normals.append(&mut vec![CUBE_NORMALS[j]]);
                            self.normals.append(&mut vec![CUBE_NORMALS[j]]);
                            self.normals.append(&mut vec![CUBE_NORMALS[j]]);
                            self.normals.append(&mut vec![CUBE_NORMALS[j]]);

                            self.uvs.append(&mut Chunk::generate_uv(ITEMS[self.cubes[x as usize][y as usize][z as usize] as usize].textures[j]).to_vec());

                            self.indices.append(&mut vec![
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
    }
}
