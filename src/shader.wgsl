 
struct OurStruct {
    color: vec3f,
    scale: vec2f,
    offset: vec2f,
};

@group(0) @binding(0) var<uniform> ourStruct: OurStruct;

 @vertex 
 fn vs_main(@builtin(vertex_index) vertexIndex : u32) -> @builtin(position) vec4f {

    var pos = array(
        vec2f( 0.0,  0.5),  // top center
        vec2f(-0.5, -0.5),  // bottom left
        vec2f( 0.5, -0.5)   // bottom right
    );

    return vec4f(pos[vertexIndex] * ourStruct.scale + ourStruct.offset, 0.0, 1.0);
}

@group(0) @binding(0) var<uniform> color: vec3f;
 
@fragment 
fn fs_main() -> @location(0) vec4f {
    return vec4f(ourStruct.color, 1.0);
}