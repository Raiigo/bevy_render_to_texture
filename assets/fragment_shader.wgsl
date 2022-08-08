struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct CustomMaterial {
    color: vec4<f32>,
};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

@group(1) @binding(0)
var<uniform> custom_material: CustomMaterial;

@fragment
fn fragment(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(lerp(1.0, 0.0, vertex_output.uv.x), 0.0, lerp(0.0, 1.0, vertex_output.uv.x), 1.0);
}