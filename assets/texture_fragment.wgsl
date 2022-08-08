struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct TextureMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> texture_material: TextureMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;

@fragment
fn fragment(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(
        textureGather(0, texture, texture_sampler, vertex_output.uv).x,
        textureGather(1, texture, texture_sampler, vertex_output.uv).y,
        textureGather(2, texture, texture_sampler, vertex_output.uv).z,
        textureGather(3, texture, texture_sampler, vertex_output.uv).w
    );
}