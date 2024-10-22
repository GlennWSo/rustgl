// Vertex shader
struct TransformU {
    transform: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: TransformU;

@group(2) @binding(0) 
var<uniform> model_iso: TransformU;

struct VertexInput {
    /// model coord
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = vec2f(model.tex_coords.x, 1.0 - model.tex_coords.y);
    out.clip_position = model_iso.transform * camera.transform *  vec4f(model.position, 1.0);
    out.clip_position.z = 0.5;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
