// Vertex struct passed in by rust code through buffer
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>
}


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Vertex shader code
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32, model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = vec3<f32>(x, y, 1.0); 
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> { // @location(0) means store into first color target
    return vec4<f32>(in.color, 1.0); // sets color of fragment to brown
}


