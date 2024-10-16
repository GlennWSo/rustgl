// Vertex shader

struct VertexOutput {
    // pixel coord
    @builtin(position) clip_position: vec4<f32>,
    // world coord
    @location(0) vert_pos: vec3<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_brown(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    out.color = vec4<f32>(0.3, 0.2, 0.1, 1.0);
    return out;
}

@vertex
fn vs_rainbow(
    @builtin(vertex_index) index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(index)) * 0.5;
    let y = f32(i32(index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    let modr = i32(index + 0 % 3);
    let modg = i32((index + 1) % 3);
    let modb = i32((index + 2) % 3);
    let r = f32(1 - min(modr, 1));
    let g = f32(1 - min(modg, 1));
    let b = f32(1 - min(modb, 1));
    
    out.color = vec4<f32>(r, g, b, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
