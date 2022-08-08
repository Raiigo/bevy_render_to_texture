struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(textureGather(3, texture, texture_sampler, vertex_output.uv).wwww);
}