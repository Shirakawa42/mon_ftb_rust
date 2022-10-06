use std::sync::{Arc, RwLock};

use crate::{chunk_filling, game_material::GameMaterial, items::ITEMS, structures::Modification};
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::{PrimitiveTopology, VertexFormat},
    },
    utils::{HashMap, HashSet},
};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use linked_hash_set::LinkedHashSet;

pub const CHUNK_SIZE: u32 = 34;
pub const REAL_CHUNK_SIZE: u32 = CHUNK_SIZE - 2;
pub const ATTRIBUTE_LAYER: MeshVertexAttribute = MeshVertexAttribute::new("Layer", 988540917, VertexFormat::Sint32);

pub type ChunkShape = ConstShape3u32<CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Cube {
    pub id: u16,
}

#[derive(Component)]
pub struct Chunk {
    cubes: [Cube; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
    pub position: [i32; 3],
    indices: Vec<u32>,
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    layers: Vec<i32>,
    pub modifications: RwLock<Vec<Modification>>,
    pub other_chunks_modifications: RwLock<Vec<([i32; 3], Modification)>>,
    pub filled: bool,
    pub drawn: bool,
    gameobject: Option<Entity>,
}

impl Chunk {
    pub fn new(position: [i32; 3]) -> Self {
        let cubes = [Cube { id: 2 }; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize];
        let indices = Vec::new();
        let vertices = Vec::new();
        let normals = Vec::new();
        let uvs = Vec::new();
        let layers = Vec::new();
        let modifications = RwLock::new(Vec::new());
        let other_chunks_modifications = RwLock::new(Vec::new());
        let gameobject = None;

        Self {
            cubes,
            position,
            indices,
            vertices,
            normals,
            uvs,
            layers,
            modifications,
            other_chunks_modifications,
            filled: false,
            drawn: false,
            gameobject,
        }
    }

    pub fn modify_other_chunks(&self, chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>, chunks_to_update: Arc<RwLock<LinkedHashSet<[i32; 3]>>>) {
        let mut modified_chunks: HashSet<[i32; 3]> = HashSet::new();

        while self.other_chunks_modifications.read().unwrap().len() > 0 {
            let (chunk_position, modification) = self.other_chunks_modifications.write().unwrap().pop().unwrap();
            if !chunks.read().unwrap().contains_key(&chunk_position) {
                chunks.write().unwrap().insert(chunk_position, Arc::new(RwLock::new(Chunk::new(chunk_position))));
            }
            let chunk = chunks.read().unwrap().get(&chunk_position).unwrap().clone();
            chunk.read().unwrap().modifications.write().unwrap().push(modification);
            modified_chunks.insert(chunk_position);
        }
        for chunk_position in modified_chunks {
            if chunks.read().unwrap().get(&chunk_position).unwrap().read().unwrap().drawn {
                chunks_to_update.write().unwrap().insert_if_absent(chunk_position);
            }
        }
    }

    pub fn fill_chunk(&mut self, chunk_filling: &chunk_filling::ChunkFilling) {
        for i in 0..CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE {
            let [x, y, z] = ChunkShape::delinearize(i);
            self.cubes[i as usize] = chunk_filling.fill_block(
                (x as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[0]) as f64,
                (y as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[1]) as f64,
                (z as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[2]) as f64,
                self,
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
            if self.gameobject != None {
                commands.entity(self.gameobject.unwrap()).despawn();
            }
            let spawned = commands.spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(mesh),
                material: material.clone(),
                transform: Transform::from_xyz(
                    self.position[0] as f32 * REAL_CHUNK_SIZE as f32,
                    self.position[1] as f32 * REAL_CHUNK_SIZE as f32,
                    self.position[2] as f32 * REAL_CHUNK_SIZE as f32,
                ),
                ..default()
            });
            self.gameobject = Some(spawned.id());
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

    pub fn update_mesh(&mut self) {
        while self.modifications.read().unwrap().len() > 0 {
            if self.modifications.read().unwrap()[0].force || self.cubes[self.modifications.read().unwrap()[0].position].id == 0 {
                self.cubes[self.modifications.read().unwrap()[0].position].id = self.modifications.read().unwrap()[0].id;
            }
            self.modifications.write().unwrap().remove(0);
        }

        self.greedy_meshing();
        self.drawn = true;
    }
}
