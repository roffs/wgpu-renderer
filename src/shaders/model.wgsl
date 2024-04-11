struct Vertex {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f,
}

struct VSOut {
    @builtin(position) position: vec4f, 
    @location(0) uv: vec2f,
    @location(1) normal: vec3f,
}
@group(0) @binding(0) var<uniform> view_projection: mat4x4f;
@group(1) @binding(0) var<uniform> model: mat4x4f;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> VSOut {

    var vsout: VSOut;

    vsout.position = view_projection * model * vec4f(vertex.position, 1.0);
    vsout.uv = vertex.uv;
    vsout.normal = vertex.normal;
    
    return vsout;
}

@group(2) @binding(0) var<uniform> baseColor: vec4f;
@group(2) @binding(1) var baseColorSampler: sampler;
@group(2) @binding(2) var baseColorTexture: texture_2d<f32>;

@fragment 
fn fs_main(vsout: VSOut) -> @location(0) vec4f {
    
    var textureColor: vec4f;
    if (textureDimensions(baseColorTexture).x > 2) {
        textureColor = textureSample(baseColorTexture, baseColorSampler, vsout.uv);
    } else {
        textureColor = vec4f(1.0, 1.0, 1.0, 1.0);
    }

    return (baseColor * textureColor);
}