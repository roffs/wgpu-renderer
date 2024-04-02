 
struct OurStruct {
    color: vec3f,
    scale: vec2f,
    offset: vec2f,
};


struct Vertex {
    @location(0) position: vec2f,
}

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> @builtin(position) vec4f {

    return vec4f(vertex.position, 0.0, 1.0);
}

@fragment 
fn fs_main() -> @location(0) vec4f {
    return vec4f(1.0, 0.0, 0.0, 0.0);
}