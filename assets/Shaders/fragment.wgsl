@group(1) @binding(0)
var my_array_texture: texture_2d_array<f32>;
@group(1) @binding(1)
var my_array_texture_sampler: sampler;

@fragment
fn fragment(
@location(0) world_position: vec4<f32>,
@location(1) world_normal: vec3<f32>,
@location(2) uv: vec2<f32>,
@location(3) layer: i32
) -> @location(0) vec4<f32> {
    var color = textureSample(my_array_texture, my_array_texture_sampler, uv, layer);
    if (color.a < 0.5) {
        discard;
    }
    return color;
}