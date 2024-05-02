struct Vertex {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
}

struct VSOut {
    @builtin(position) position: vec4f, 
    @location(0) uv: vec3f,
}
@group(0) @binding(0) 
var<uniform> view_projection: mat4x4f;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> VSOut {

    var vsout: VSOut;

    vsout.position = view_projection * vec4f(vertex.position * 2.0, 1.0);
    vsout.uv = vertex.position; // TODO remove .uv and use .position instead
    
    return vsout;
}

@group(1) @binding(0) 
var sky_sampler: sampler;
@group(1) @binding(1) 
var sky_texture: texture_cube<f32>;

@fragment 
fn fs_main(vsout: VSOut) -> @location(0) vec4f {
    return textureSample(sky_texture, sky_sampler, vsout.uv);
}