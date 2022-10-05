use crate::{chunk_filling, game_material::GameMaterial, items::ITEMS};
use bevy::{
    prelude::*,
    render::{mesh::{Indices, MeshVertexAttribute}, render_resource::{PrimitiveTopology, VertexFormat}},
};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
const CHUNK_SIZE: u32 = 34;
const REAL_CHUNK_SIZE: u32 = CHUNK_SIZE - 2;
pub const ATTRIBUTE_LAYER: MeshVertexAttribute = MeshVertexAttribute::new("Layer", 988540917, VertexFormat::Sint32);

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
    pub layers: Vec<i32>,
    pub filled: bool,
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        let cubes = [Cube { id: 2 }; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize];
        let indices = Vec::new();
        let vertices = Vec::new();
        let normals = Vec::new();
        let uvs = Vec::new();
        let layers = Vec::new();
        let filled = false;

        Self {
            cubes,
            position,
            indices,
            vertices,
            normals,
            uvs,
            layers,
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
            mesh.insert_attribute(ATTRIBUTE_LAYER, std::mem::replace(&mut self.layers, Vec::new()));
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

    fn greedy_meshing(&mut self) {
        let mut buffer = GreedyQuadsBuffer::new((REAL_CHUNK_SIZE * REAL_CHUNK_SIZE * REAL_CHUNK_SIZE) as usize);
        greedy_quads(&self.cubes, &ChunkShape {}, [0; 3], [CHUNK_SIZE as u32 - 1; 3], &RIGHT_HANDED_Y_UP_CONFIG.faces, &mut buffer);
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
        let mut i = 0;

        self.vertices = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.indices = Vec::with_capacity(buffer.quads.num_quads() * 6);
        self.normals = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.uvs = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.layers = Vec::with_capacity(buffer.quads.num_quads() * 4);

        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                self.indices.extend_from_slice(&face.quad_mesh_indices(self.vertices.len() as u32));
                self.vertices.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                self.normals.extend_from_slice(&face.quad_mesh_normals());
                self.uvs.extend_from_slice(&face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad));
                let cube_id = ITEMS[self.cubes[ChunkShape::linearize([quad.minimum[0], quad.minimum[1], quad.minimum[2]]) as usize].id as usize].textures[i] as i32;
                self.layers.extend_from_slice(&[cube_id, cube_id, cube_id, cube_id]);
            }
            i += 1;
        }
    }

    pub fn generate_mesh(&mut self) {
        self.greedy_meshing();
    }
}
