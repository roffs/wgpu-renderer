struct Vertex {
    @location(0) position: vec3f,
}

struct VSOut {
    @builtin(position) position: vec4f,
    @location(1) distance: f32,
}

struct Camera {
    position: vec3f,
    view: mat4x4f,
    inv_view: mat4x4f,
    proj: mat4x4f,
    inv_proj: mat4x4f
}

@group(0) @binding(0) var<uniform> camera: Camera;

struct Transform {
    model: mat4x4f
}

@group(1) @binding(0) var<uniform> transform: Transform;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> VSOut {

    let invert_x = mat4x4f(
        vec4f(-1.0, 0.0, 0.0, 0.0),
        vec4f( 0.0, 1.0, 0.0, 0.0),
        vec4f( 0.0, 0.0, 1.0, 0.0),
        vec4f( 0.0, 0.0, 0.0, 1.0),
    );

    var vsout: VSOut;
    var camera_space_vertex_position = camera.view * transform.model * vec4f(vertex.position, 1.0);
    
    vsout.distance = min(length(camera_space_vertex_position.xyz) / 25.0, 1.0); //TODO read zFar plane from a uniform?
    vsout.position = invert_x * camera.proj * camera_space_vertex_position;

    return vsout;
}

@fragment
fn fs_main(vsout: VSOut) -> @location(0) vec4f {
    return vec4f(vsout.distance, vsout.distance, vsout.distance, 1.0);
}