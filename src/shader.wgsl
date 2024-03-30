 
  
struct VertexShaderOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

 @vertex 
 fn vs_main(@builtin(vertex_index) vertexIndex : u32) -> VertexShaderOutput {

    var pos = array(
        vec2f( 0.0,  0.5),  // top center
        vec2f(-0.5, -0.5),  // bottom left
        vec2f( 0.5, -0.5)   // bottom right
    );

    var colors = array(
        vec4f(1.0, 0.0, 0.0, 1.0),
        vec4f(0.0, 1.0, 0.0, 1.0),
        vec4f(0.0, 0.0, 1.0, 1.0),
    );

    var out: VertexShaderOutput;

    out.position = vec4f(pos[vertexIndex], 0.0, 1.0);
    out.color = colors[vertexIndex];

    return out;
}
 
@fragment 
fn fs_main(fs_input: VertexShaderOutput) -> @location(0) vec4f {
    return fs_input.color;
}