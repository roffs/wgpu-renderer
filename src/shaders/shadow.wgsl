struct Vertex {
    @location(0) position: vec3f,
}

@group(0) @binding(0) var<uniform> view: mat4x4f;
@group(0) @binding(1) var<uniform> projection: mat4x4f;

struct Transform {
    model: mat4x4f
}

@group(1) @binding(0) var<uniform> transform: Transform;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> @builtin(position) vec4f {

    return projection * view * transform.model * vec4f(vertex.position, 1.0);
}