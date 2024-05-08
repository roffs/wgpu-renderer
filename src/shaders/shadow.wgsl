struct Vertex {
    @location(0) position: vec3f,
}
struct VSOut {
    @builtin(position) position: vec4f, 

}

@group(0) @binding(0) var<uniform> view_projection: mat4x4f;

struct Transform {
    model: mat4x4f
}

@group(1) @binding(0) var<uniform> transform: Transform;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> VSOut {

    var vsout: VSOut;

    var vertex_world_position = transform.model * vec4f(vertex.position, 1.0);
    vsout.position = view_projection * vertex_world_position;
    vsout.position.x *= -1.0;
    // vsout.position.x += 1.0;

    return vsout;
}

