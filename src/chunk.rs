use std::sync::{Arc, RwLock};

use crate::{
    chunk_filling::ChunkFilling,
    game_material::GameMaterial,
    items::{Items, FACES, ITEMS},
    lighting::recalculate_natural_light,
    structures::{LightModification, Modification},
    world,
};
use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::{PrimitiveTopology, VertexFormat},
    },
    utils::{HashMap, HashSet},
};
use block_mesh::{
    greedy_quads,
    ndshape::{ConstShape, ConstShape3u32},
};
use block_mesh::{GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use linked_hash_map::LinkedHashMap;

pub const CHUNK_SIZE: u32 = 34;
pub const REAL_CHUNK_SIZE: u32 = CHUNK_SIZE - 2;
pub const ATTRIBUTE_LAYER: MeshVertexAttribute = MeshVertexAttribute::new("Layer", 988540917, VertexFormat::Sint32);
pub const ATTRIBUTE_LIGHT_LEVEL: MeshVertexAttribute = MeshVertexAttribute::new("Light_Level", 988164917, VertexFormat::Float32);

pub type ChunkShape = ConstShape3u32<CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Cube {
    pub id: u16,
    pub light_level: u8,
}

#[derive(Component)]
pub struct Chunk {
    pub cubes: Arc<RwLock<[Cube; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize]>>,
    pub position: [i32; 3],
    indices: Vec<u32>,
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    layers: Vec<i32>,
    light_levels: Vec<f32>,
    pub modifications: RwLock<LinkedHashMap<usize, Modification>>,
    pub light_modifications: RwLock<Vec<LightModification>>,
    pub other_chunks_modifications: RwLock<LinkedHashMap<(usize, [i32; 3]), ([i32; 3], Modification)>>,
    pub other_chunks_light_modifications: RwLock<Vec<([i32; 3], LightModification)>>,
    pub filled: Arc<RwLock<bool>>,
    pub drawn: bool,
    gameobject: Option<Entity>,
    pub world: Arc<RwLock<world::World>>,
    pub update_count: u32,
}

impl Chunk {
    pub fn new(position: [i32; 3], world: Arc<RwLock<world::World>>) -> Self {
        let cubes = Arc::new(RwLock::new([Cube { id: 0, light_level: 0 }; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize]));
        let indices = Vec::new();
        let vertices = Vec::new();
        let normals = Vec::new();
        let uvs = Vec::new();
        let layers = Vec::new();
        let light_levels = Vec::new();
        let modifications = RwLock::new(LinkedHashMap::new());
        let light_modifications = RwLock::new(Vec::new());
        let other_chunks_modifications = RwLock::new(LinkedHashMap::new());
        let other_chunks_light_modifications = RwLock::new(Vec::new());
        let gameobject = None;

        Self {
            cubes,
            position,
            indices,
            vertices,
            normals,
            uvs,
            layers,
            light_levels,
            modifications,
            light_modifications,
            other_chunks_modifications,
            other_chunks_light_modifications,
            filled: Arc::new(RwLock::new(false)),
            drawn: false,
            gameobject,
            world,
            update_count: 0,
        }
    }

    pub fn modify_other_chunks(&self) {
        let mut modified_chunks: HashSet<[i32; 3]> = HashSet::new();
        let chunks = self.world.read().unwrap().chunks.clone();

        while self.other_chunks_modifications.read().unwrap().len() > 0 {
            let (_, (chunk_position, modification)) = self.other_chunks_modifications.write().unwrap().pop_back().unwrap();
            {
                let mut chunks_lock = chunks.write().unwrap();
                if !chunks_lock.contains_key(&chunk_position) {
                    chunks_lock.insert(chunk_position, Arc::new(RwLock::new(Chunk::new(chunk_position, self.world.clone()))));
                }
            }
            let chunk = chunks.read().unwrap().get(&chunk_position).unwrap().clone();
            chunk.read().unwrap().modifications.write().unwrap().insert(modification.position, modification);
            modified_chunks.insert(chunk_position);
        }
        while self.other_chunks_light_modifications.read().unwrap().len() > 0 {
            let (chunk_position, modification) = self.other_chunks_light_modifications.write().unwrap().pop().unwrap();
            {
                let mut chunks_lock = chunks.write().unwrap();
                if !chunks_lock.contains_key(&chunk_position) {
                    chunks_lock.insert(chunk_position, Arc::new(RwLock::new(Chunk::new(chunk_position, self.world.clone()))));
                }
            }
            let chunk = chunks.read().unwrap().get(&chunk_position).unwrap().clone();
            chunk.read().unwrap().light_modifications.write().unwrap().push(modification);
            modified_chunks.insert(chunk_position);
        }
        let chunks_to_update = self.world.read().unwrap().chunks_to_update.clone();
        for chunk_position in modified_chunks {
            let chunk = chunks.read().unwrap().get(&chunk_position).unwrap().clone();
            if chunk.read().unwrap().drawn {
                chunks_to_update.write().unwrap().insert_if_absent(chunk_position);
            }
        }
    }

    fn modify_neighbours(&self, chunk_position: [i32; 3], modification: &Modification) {
        let modification_pos = ChunkShape::delinearize(modification.position as u32);

        if modification_pos[0] == 1 {
            let other_chunk_position = [chunk_position[0] - 1, chunk_position[1], chunk_position[2]];
            let modification = Modification {
                position: ChunkShape::linearize([REAL_CHUNK_SIZE + 1, modification_pos[1], modification_pos[2]]) as usize,
                id: modification.id,
                force: modification.force,
            };
            if other_chunk_position == self.position {
                self.modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_modifications
                    .write()
                    .unwrap()
                    .insert((modification.position, other_chunk_position), (other_chunk_position, modification));
            }
        } else if modification_pos[0] == REAL_CHUNK_SIZE {
            let other_chunk_position = [chunk_position[0] + 1, chunk_position[1], chunk_position[2]];
            let modification = Modification {
                position: ChunkShape::linearize([0, modification_pos[1], modification_pos[2]]) as usize,
                id: modification.id,
                force: modification.force,
            };
            if other_chunk_position == self.position {
                self.modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_modifications
                    .write()
                    .unwrap()
                    .insert((modification.position, other_chunk_position), (other_chunk_position, modification));
            }
        }
        if modification_pos[1] == 1 {
            let other_chunk_position = [chunk_position[0], chunk_position[1] - 1, chunk_position[2]];
            let modification = Modification {
                position: ChunkShape::linearize([modification_pos[0], REAL_CHUNK_SIZE + 1, modification_pos[2]]) as usize,
                id: modification.id,
                force: modification.force,
            };
            if other_chunk_position == self.position {
                self.modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_modifications
                    .write()
                    .unwrap()
                    .insert((modification.position, other_chunk_position), (other_chunk_position, modification));
            }
        } else if modification_pos[1] == REAL_CHUNK_SIZE {
            let other_chunk_position = [chunk_position[0], chunk_position[1] + 1, chunk_position[2]];
            let modification = Modification {
                position: ChunkShape::linearize([modification_pos[0], 0, modification_pos[2]]) as usize,
                id: modification.id,
                force: modification.force,
            };
            if other_chunk_position == self.position {
                self.modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_modifications
                    .write()
                    .unwrap()
                    .insert((modification.position, other_chunk_position), (other_chunk_position, modification));
            }
        }
        if modification_pos[2] == 1 {
            let other_chunk_position = [chunk_position[0], chunk_position[1], chunk_position[2] - 1];
            let modification = Modification {
                position: ChunkShape::linearize([modification_pos[0], modification_pos[1], REAL_CHUNK_SIZE + 1]) as usize,
                id: modification.id,
                force: modification.force,
            };
            if other_chunk_position == self.position {
                self.modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_modifications
                    .write()
                    .unwrap()
                    .insert((modification.position, other_chunk_position), (other_chunk_position, modification));
            }
        } else if modification_pos[2] == REAL_CHUNK_SIZE {
            let other_chunk_position = [chunk_position[0], chunk_position[1], chunk_position[2] + 1];
            let modification = Modification {
                position: ChunkShape::linearize([modification_pos[0], modification_pos[1], 0]) as usize,
                id: modification.id,
                force: modification.force,
            };
            if other_chunk_position == self.position {
                self.modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_modifications
                    .write()
                    .unwrap()
                    .insert((modification.position, other_chunk_position), (other_chunk_position, modification));
            }
        }
    }

    fn modify_neighbours_light(&self, chunk_position: [i32; 3], modification: &LightModification) {
        let modification_pos = ChunkShape::delinearize(modification.position as u32);

        if modification_pos[0] == 1 {
            let other_chunk_position = [chunk_position[0] - 1, chunk_position[1], chunk_position[2]];
            let modification = LightModification {
                position: ChunkShape::linearize([REAL_CHUNK_SIZE + 1, modification_pos[1], modification_pos[2]]) as usize,
                light_level: modification.light_level,
            };
            if other_chunk_position == self.position {
                self.light_modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_light_modifications.write().unwrap().push((other_chunk_position, modification));
            }
        } else if modification_pos[0] == REAL_CHUNK_SIZE {
            let other_chunk_position = [chunk_position[0] + 1, chunk_position[1], chunk_position[2]];
            let modification = LightModification {
                position: ChunkShape::linearize([0, modification_pos[1], modification_pos[2]]) as usize,
                light_level: modification.light_level,
            };
            if other_chunk_position == self.position {
                self.light_modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_light_modifications.write().unwrap().push((other_chunk_position, modification));
            }
        }
        if modification_pos[1] == 1 {
            let other_chunk_position = [chunk_position[0], chunk_position[1] - 1, chunk_position[2]];
            let modification = LightModification {
                position: ChunkShape::linearize([modification_pos[0], REAL_CHUNK_SIZE + 1, modification_pos[2]]) as usize,
                light_level: modification.light_level,
            };
            if other_chunk_position == self.position {
                self.light_modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_light_modifications.write().unwrap().push((other_chunk_position, modification));
            }
        } else if modification_pos[1] == REAL_CHUNK_SIZE {
            let other_chunk_position = [chunk_position[0], chunk_position[1] + 1, chunk_position[2]];
            let modification = LightModification {
                position: ChunkShape::linearize([modification_pos[0], 0, modification_pos[2]]) as usize,
                light_level: modification.light_level,
            };
            if other_chunk_position == self.position {
                self.light_modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_light_modifications.write().unwrap().push((other_chunk_position, modification));
            }
        }
        if modification_pos[2] == 1 {
            let other_chunk_position = [chunk_position[0], chunk_position[1], chunk_position[2] - 1];
            let modification = LightModification {
                position: ChunkShape::linearize([modification_pos[0], modification_pos[1], REAL_CHUNK_SIZE + 1]) as usize,
                light_level: modification.light_level,
            };
            if other_chunk_position == self.position {
                self.light_modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_light_modifications.write().unwrap().push((other_chunk_position, modification));
            }
        } else if modification_pos[2] == REAL_CHUNK_SIZE {
            let other_chunk_position = [chunk_position[0], chunk_position[1], chunk_position[2] + 1];
            let modification = LightModification {
                position: ChunkShape::linearize([modification_pos[0], modification_pos[1], 0]) as usize,
                light_level: modification.light_level,
            };
            if other_chunk_position == self.position {
                self.light_modifications.write().unwrap().insert(modification.position, modification);
            } else {
                self.other_chunks_light_modifications.write().unwrap().push((other_chunk_position, modification));
            }
        }
    }

    pub fn add_modification_no_update(&self, modification: Modification, chunk_position: [i32; 3]) {
        if chunk_position == self.position {
            let exist = self.modifications.read().unwrap().contains_key(&modification.position);
            if exist && modification.force {
                self.modifications.write().unwrap().insert(modification.position, modification);
            } else if !exist {
                self.modifications.write().unwrap().insert(modification.position, modification);
            }
        } else {
            let exist = self.other_chunks_modifications.read().unwrap().contains_key(&(modification.position, chunk_position));
            if exist && modification.force {
                self.modify_neighbours(chunk_position, &modification);
                self.other_chunks_modifications.write().unwrap().insert((modification.position, chunk_position), (chunk_position, modification));
            } else if !exist {
                self.modify_neighbours(chunk_position, &modification);
                self.other_chunks_modifications.write().unwrap().insert((modification.position, chunk_position), (chunk_position, modification));
            }
        }
    }

    pub fn modify_light_at_pos_no_update(&self, modification: LightModification, chunk_position: [i32; 3]) {
        self.modify_neighbours_light(chunk_position, &modification);
        if chunk_position == self.position {
            self.light_modifications.write().unwrap().push(modification);
        } else {
            self.other_chunks_light_modifications.write().unwrap().push((chunk_position, modification));
        }
    }

    pub fn fill_chunk(&self, chunk_filling: Arc<RwLock<ChunkFilling>>, chunks: Arc<RwLock<HashMap<[i32; 3], Arc<RwLock<Chunk>>>>>, sky_heights: &RwLock<[i32; (CHUNK_SIZE * CHUNK_SIZE) as usize]>) {
        let mut column_heights: [[i32; CHUNK_SIZE as usize]; CHUNK_SIZE as usize] = [[0; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

        for i in 0..CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE {
            let [x, y, z] = ChunkShape::delinearize(i);
            let [gx, gy, gz] = [
                (x as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[0]) as f64,
                (y as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[1]) as f64,
                (z as f64 - 1.0) + (REAL_CHUNK_SIZE as i32 * self.position[2]) as f64,
            ];
            self.cubes.write().unwrap()[i as usize] = chunk_filling.read().unwrap().fill_block(gx, gy, gz, self, true);
            if self.cubes.write().unwrap()[i as usize].id != Items::Air as u16 && gy as i32 > column_heights[x as usize][z as usize] {
                column_heights[x as usize][z as usize] = (gy + 1.0) as i32;
            }
        }
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if sky_heights.read().unwrap()[((x + (z * CHUNK_SIZE)) as usize)] < column_heights[x as usize][z as usize] {
                    sky_heights.write().unwrap()[((x + (z * CHUNK_SIZE)) as usize)] = column_heights[x as usize][z as usize];
                }
            }
        }
        self.apply_self_modifications();
        *self.filled.write().unwrap() = true;
        recalculate_natural_light(chunks, self, sky_heights);
    }

    pub fn apply_self_modifications(&self) {
        while self.modifications.read().unwrap().len() > 0 {
            let (_, modification) = self.modifications.write().unwrap().pop_back().unwrap();
            let position = modification.position;
            if modification.force || self.cubes.read().unwrap()[position].id == 0 {
                self.modify_neighbours(self.position, &modification);
                self.cubes.write().unwrap()[position].id = modification.id;
            }
        }
    }

    pub fn apply_self_light_modifications(&self) {
        while self.light_modifications.read().unwrap().len() > 0 {
            let light_modifications = self.light_modifications.write().unwrap().pop().unwrap();
            let position = light_modifications.position;
            self.cubes.write().unwrap()[position].light_level = light_modifications.light_level;
        }
    }

    pub fn draw_mesh(&mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, material: Handle<GameMaterial>) {
        if self.vertices.len() > 0 {
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            mesh.set_indices(Some(Indices::U32(std::mem::replace(&mut self.indices, Vec::new()))));
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, std::mem::replace(&mut self.vertices, Vec::new()));
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, std::mem::replace(&mut self.uvs, Vec::new()));
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, std::mem::replace(&mut self.normals, Vec::new()));
            mesh.insert_attribute(ATTRIBUTE_LAYER, std::mem::replace(&mut self.layers, Vec::new()));
            mesh.insert_attribute(ATTRIBUTE_LIGHT_LEVEL, std::mem::replace(&mut self.light_levels, Vec::new()));
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
        greedy_quads(&*self.cubes.read().unwrap(), &ChunkShape {}, [0; 3], [CHUNK_SIZE as u32 - 1; 3], &RIGHT_HANDED_Y_UP_CONFIG.faces, &mut buffer);
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
        let mut i = 0;

        self.vertices = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.indices = Vec::with_capacity(buffer.quads.num_quads() * 6);
        self.normals = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.uvs = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.layers = Vec::with_capacity(buffer.quads.num_quads() * 4);
        self.light_levels = Vec::with_capacity(buffer.quads.num_quads() * 4);

        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                self.indices.extend_from_slice(&face.quad_mesh_indices(self.vertices.len() as u32));
                self.vertices.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                self.normals.extend_from_slice(&face.quad_mesh_normals());
                self.uvs.extend_from_slice(&face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad));
                let cube_id = ITEMS[self.cubes.read().unwrap()[ChunkShape::linearize([quad.minimum[0], quad.minimum[1], quad.minimum[2]]) as usize].id as usize].textures[i] as i32;
                self.layers.extend_from_slice(&[cube_id, cube_id, cube_id, cube_id]);
                let light_level = self.cubes.read().unwrap()[ChunkShape::linearize([
                    (quad.minimum[0] as i8 + FACES[i][0]) as u32,
                    (quad.minimum[1] as i8 + FACES[i][1]) as u32,
                    (quad.minimum[2] as i8 + FACES[i][2]) as u32,
                ]) as usize]
                    .light_level as f32
                    / 255.0;
                self.light_levels.extend_from_slice(&[light_level, light_level, light_level, light_level]);
            }
            i += 1;
        }
    }

    pub fn update_mesh(&mut self) {
        self.apply_self_modifications();
        self.apply_self_light_modifications();

        self.greedy_meshing();
        self.drawn = true;

        self.update_count += 1;
        //if self.update_count > 1 {
        //    println!("Chunk {} {} {} updated {} times", self.position[0], self.position[1], self.position[2], self.update_count);
        //}
    }
}
