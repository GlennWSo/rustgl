// Vertex shader

struct VertexInput {
    /// model coord
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    // screen coord
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
};


@vertex
fn vs_rainbow(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4f(model.position, 1.0);
    return out;
}

@vertex
fn vs_brown(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3f(0.6, 0.5, 0.4);
    out.clip_position = vec4f(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4f(in.color, 1.0);
}
