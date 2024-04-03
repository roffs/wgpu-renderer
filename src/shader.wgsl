 
struct OurStruct {
    color: vec3f,
    scale: vec2f,
    offset: vec2f,
};

struct Vertex {
    @location(0) position: vec2f,
    @location(1) uv: vec2f,
}

struct VSOut {
    @builtin(position) position: vec4f, 
    @location(0) uv: vec2f,
}
@group(0) @binding(0) var<uniform> model: mat4x4f;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> VSOut {

    var vsout: VSOut;

    vsout.position = model * vec4f(vertex.position, 0.0, 1.0);
    vsout.uv = vertex.uv;
    
    return vsout;
}


@group(1) @binding(0) var ourSampler: sampler;
@group(1) @binding(1) var ourTexture: texture_2d<f32>;

@fragment 
fn fs_main(vsout: VSOut) -> @location(0) vec4f {
    return textureSample(ourTexture, ourSampler, vsout.uv);
}