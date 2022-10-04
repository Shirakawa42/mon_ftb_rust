use crate::{chunk_filling, game_material::GameMaterial};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
const CHUNK_SIZE: u32 = 34;
const REAL_CHUNK_SIZE: u32 = CHUNK_SIZE - 2;
const TEXTURE_ATLAS_SIZE: f32 = 16.0;
const NORMALIZED: f32 = 1.0 / TEXTURE_ATLAS_SIZE;

type ChunkShape = ConstShape3u32<CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Cube {
    pub id: u16,
}

#[derive(Component)]
pub struct Chunk {
    pub cubes: [Cube; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
    pub position: [i32; 3],
    pub indices: Vec<u32>,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub filled: bool,
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        let cubes = [Cube { id: 2 }; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize];
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
        for i in 0..CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE {
            let [x, y, z] = ChunkShape::delinearize(i);
            self.cubes[i as usize] = chunk_filling.fill_block(
                (x as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[0]) as f64,
                (y as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[1]) as f64,
                (z as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[2]) as f64,
            );
        }
        self.filled = true;
    }

    pub fn draw_mesh(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, material: Handle<GameMaterial>) {
        if self.vertices.len() > 0 {
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

    //    fn add_usize_3_to_f32_3(a: [usize; 3], b: [f32; 3]) -> Vec<[f32; 3]> {
    //        vec![[a[0] as f32 + b[0], a[1] as f32 + b[1], a[2] as f32 + b[2]]]
    //    }
    //
    //    fn check_side(&self, x: i32, y: i32, z: i32) -> bool {
    //        if ITEMS[self.cubes[ChunkShape::linearize([x as u32, y as u32, z as u32]) as usize].id as usize].is_opaque == false {
    //            return true;
    //        }
    //        return false;
    //    }

    fn greedy_meshing(&mut self) {
        let mut buffer = GreedyQuadsBuffer::new((REAL_CHUNK_SIZE * REAL_CHUNK_SIZE * REAL_CHUNK_SIZE) as usize);
        greedy_quads(&self.cubes, &ChunkShape {}, [0; 3], [CHUNK_SIZE as u32 - 1; 3], &RIGHT_HANDED_Y_UP_CONFIG.faces, &mut buffer);
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
        self.vertices = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.indices = Vec::with_capacity(buffer.quads.num_quads() * 6);
        self.normals = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.uvs = Vec::with_capacity(buffer.quads.num_quads() * 4);

        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                self.indices.extend_from_slice(&face.quad_mesh_indices(self.vertices.len() as u32));
                self.vertices.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                self.normals.extend_from_slice(&face.quad_mesh_normals());
                self.uvs.extend_from_slice(&Self::generate_uv(1));
            }
        }
    }

    pub fn generate_mesh(&mut self) {
        //let mut vertex_index = 0;

        self.greedy_meshing();
        //for x in 1..CHUNK_SIZE as usize - 1 {
        //    for y in 1..CHUNK_SIZE as usize - 1 {
        //        for z in 1..CHUNK_SIZE as usize - 1 {
        //            if self.cubes[ChunkShape::linearize([x as u32, y as u32, z as u32]) as usize].id == 0 {
        //                continue;
        //            }
        //            for j in 0..6 {
        //                if self.check_side(x as i32 + CUBE_FACE_CHECKS[j][0], y as i32 + CUBE_FACE_CHECKS[j][1], z as i32 + CUBE_FACE_CHECKS[j][2]) {
        //                    self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][0]]));
        //                    self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][1]]));
        //                    self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][2]]));
        //                    self.vertices.append(&mut Chunk::add_usize_3_to_f32_3([x, y, z], CUBE_VERTICES[CUBE_INDICES[j][3]]));
        //
        //                    self.normals.append(&mut vec![CUBE_NORMALS[j]]);
        //                    self.normals.append(&mut vec![CUBE_NORMALS[j]]);
        //                    self.normals.append(&mut vec![CUBE_NORMALS[j]]);
        //                    self.normals.append(&mut vec![CUBE_NORMALS[j]]);
        //
        //                    self.uvs
        //                        .append(&mut Chunk::generate_uv(ITEMS[self.cubes[ChunkShape::linearize([x as u32, y as u32, z as u32]) as usize].id as usize].textures[j]).to_vec());
        //
        //                    self.indices.append(&mut vec![
        //                        vertex_index as u32,
        //                        vertex_index as u32 + 1,
        //                        vertex_index as u32 + 2,
        //                        vertex_index as u32 + 2,
        //                        vertex_index as u32 + 1,
        //                        vertex_index as u32 + 3,
        //                    ]);
        //
        //                    vertex_index += 4;
        //                }
        //            }
        //        }
        //    }
        //}
    }
}
