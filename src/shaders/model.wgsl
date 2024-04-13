struct Vertex {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f,
    @location(3) tangent: vec3f,
    @location(4) bitangent: vec3f,
}

struct VSOut {
    @builtin(position) position: vec4f, 
    @location(0) uv: vec2f,
    @location(1) normal: vec3f,
    @location(2) fragment_position: vec4f,
}

@group(0) @binding(0) var<uniform> view_projection: mat4x4f;

struct Transform {
    model: mat4x4f,
    normal: mat4x4f
}

@group(1) @binding(0) var<uniform> transform: Transform;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> VSOut {

    var vsout: VSOut;

    var vertex_world_position = transform.model * vec4f(vertex.position, 1.0);

    vsout.position = view_projection * vertex_world_position;
    vsout.uv = vertex.uv;
    vsout.normal = (transform.normal * vec4f(vertex.normal, 1.0)).xyz;
    vsout.fragment_position = vertex_world_position;
    
    return vsout;
}

@group(2) @binding(0) var<uniform> baseColor: vec4f;
@group(2) @binding(1) var baseColorSampler: sampler;
@group(2) @binding(2) var baseColorTexture: texture_2d<f32>;
@group(2) @binding(3) var normalSampler: sampler;
@group(2) @binding(4) var normalTexture: texture_2d<f32>;

struct PointLight {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
}

@group(3) @binding(0) var<storage, read> lights: array<PointLight>;

@fragment 
fn fs_main(vsout: VSOut) -> @location(0) vec4f {
    
    var textureColor: vec4f;
    if (textureDimensions(baseColorTexture).x > 1) {
        textureColor = textureSample(baseColorTexture, baseColorSampler, vsout.uv);
    } else {
        textureColor = vec4f(1.0, 1.0, 1.0, 1.0);
    }

    var objectColor: vec4f = baseColor * textureColor;

    // AMBIENT LIGHT
    var ambientStrength = 0.1;
    var ambient = ambientStrength * vec3f(1.0, 1.0, 1.0);

    // DIFFUSE LIGHT
    var normal = normalize(vsout.normal);

    var diffuse = vec3f(0.0, 0.0, 0.0);

    for (var i: u32 = 0; i < arrayLength(&lights); i = i + 1 ) {
        var light = lights[i];
        var lightDir = normalize(lights[i].position - vsout.fragment_position.xyz);
    
        var diff = max(dot(normal, lightDir), 0.0);
        diffuse += diff * light.color;
    }

    var result = vec4f(ambient + diffuse, 1.0) * objectColor;

    return result;
}