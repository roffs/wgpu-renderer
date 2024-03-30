 
  
struct VertexShaderOutput {
    @builtin(position) position: vec4f,
}

 @vertex 
 fn vs_main(@builtin(vertex_index) vertexIndex : u32) -> VertexShaderOutput {

    var pos = array(
        vec2f( 0.0,  0.5),  // top center
        vec2f(-0.5, -0.5),  // bottom left
        vec2f( 0.5, -0.5)   // bottom right
    );

    var out: VertexShaderOutput;
    out.position = vec4f(pos[vertexIndex], 0.0, 1.0);
    return out;
}

@group(0) @binding(0) var<uniform> color: vec3f;
 
@fragment 
fn fs_main() -> @location(0) vec4f {
    return vec4f(color, 1.0);
}