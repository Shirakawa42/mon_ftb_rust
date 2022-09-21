use bevy::{render::render_resource::{AsBindGroup, ShaderRef}, reflect::TypeUuid, prelude::*};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct GameMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}

impl Material for GameMaterial {
    fn fragment_shader() -> ShaderRef {
        "Shaders/shader.wgsl".into()
    }
}