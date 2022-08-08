fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

fn inverse_lerp(a: f32, b: f32, t: f32) -> f32 {
    return (lerp(a, b, t) - a) / (b - a);
}

fn remap(out_min: f32, out_max: f32, in_min: f32, in_max: f32, t: f32) -> f32 {
    return lerp(out_min, out_max, inverse_lerp(in_min, in_max, t));
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(4) color: vec4<f32>,
};

// @group(1) @binding(0)
// var<uniform> vertex_out: VertexOutput;

@fragment
fn fragment(vertex_out: VertexOutput) -> @location(0) vec4<f32> {
    return vertex_out.color;
}