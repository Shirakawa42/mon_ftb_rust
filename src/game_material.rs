use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError},
    },
};

use crate::chunk::{ATTRIBUTE_LAYER, ATTRIBUTE_LIGHT_LEVEL, ATTRIBUTE_AO};

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "9c5a0ddf-1eaf-41b4-9832-ed736fd26af3"]
pub struct GameMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    pub array_texture: Handle<Image>,
}

impl Material for GameMaterial {
    fn fragment_shader() -> ShaderRef {
        "Shaders/fragment.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "Shaders/vertex.wgsl".into()
    }

    fn specialize(_pipeline: &MaterialPipeline<Self>, descriptor: &mut RenderPipelineDescriptor, layout: &MeshVertexBufferLayout, _key: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_LAYER.at_shader_location(3),
            ATTRIBUTE_LIGHT_LEVEL.at_shader_location(4),
            ATTRIBUTE_AO.at_shader_location(5),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
