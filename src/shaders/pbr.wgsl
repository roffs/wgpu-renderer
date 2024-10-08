struct Vertex {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f,
    @location(3) tangent: vec4f,
}

struct VSOut {
    @builtin(position) position: vec4f, 
    @location(0) uv: vec2f,
    @location(1) normal: vec3f,
    @location(2) world_position: vec4f,
    @location(3) tangent: vec3f,
    @location(4) bitangent: vec3f,
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
    model: mat4x4f,
    normal: mat4x4f
}

@group(1) @binding(0) var<uniform> transform: Transform;

const PI: f32 = 3.14159265359;

@vertex 
fn vs_main(
    vertex: Vertex,    
) -> VSOut {

    var vsout: VSOut;

    var vertex_world_position = transform.model * vec4f(vertex.position, 1.0);

    vsout.position = camera.proj * camera.view * vertex_world_position;
    vsout.uv = vertex.uv;
    vsout.world_position = vertex_world_position;
    vsout.normal = normalize((transform.normal * vec4f(vertex.normal, 1.0)).xyz);
    vsout.tangent = normalize((transform.normal * vertex.tangent).xyz);
    vsout.bitangent = cross(vsout.tangent, vsout.normal) * vertex.tangent.w; // Correct right-handeness
    
    return vsout;
}

struct MaterialProperties {
    base_color: vec4f,
    metallic_factor: f32,
    roughness_factor: f32,
}

@group(2) @binding(0) var<uniform> material: MaterialProperties;
@group(2) @binding(1) var baseColorSampler: sampler;
@group(2) @binding(2) var baseColorTexture: texture_2d<f32>;
@group(2) @binding(3) var normalSampler: sampler;
@group(2) @binding(4) var normalTexture: texture_2d<f32>;
@group(2) @binding(5) var metallicRoughnessSampler: sampler;
@group(2) @binding(6) var metallicRoughnessTexture: texture_2d<f32>;
@group(2) @binding(7) var ambientOcclussionSampler: sampler;
@group(2) @binding(8) var ambientOcclussionTexture: texture_2d<f32>;

struct PointLight {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
}

@group(3) @binding(0) var<storage, read> lights: array<PointLight>;
@group(3) @binding(1) var shadow_maps: binding_array<texture_cube<f32>, 3>;
@group(3) @binding(2) var shadow_maps_samplers: binding_array<sampler, 3>;

@group(4) @binding(0) var irrSampler: sampler;
@group(4) @binding(1) var irrMap: texture_cube<f32>;;

@fragment 
fn fs_main(vsout: VSOut) -> @location(0) vec4f {
    
    var albedo = get_albedo(vsout.uv);
    var normal = get_normal(vsout);
    var metallic = get_metalness(vsout.uv);
    var roughness = get_roughness(vsout.uv);
    var ao = get_ambient_occlussion(vsout.uv);
    
    var world_position = vsout.world_position.xyz;

    var V = normalize(camera.position - world_position);
    var F0 = mix(vec3(0.04), albedo, metallic);

    // Over all lights:
    var Lo = vec3(0.0);

    for (var i: u32 = 0; i < arrayLength(&lights); i = i + 1 ) {
        var light = lights[i];

        var L = normalize(light.position - world_position);
        var H = normalize(V + L);

        var light_distance = length(light.position - world_position);
        var attenuation = 1.0 / (light_distance * light_distance);
        var light_radiance = light.color * attenuation;

        // Calculate Cook-Torrance specular BRDF: DFG / 4(ωo⋅n)(ωi⋅n)
        var F = fresnel_schlick(max( dot(H, V), 0.0 ), F0);
        var D = distribution_ggx(normal, H, roughness);
        var G = geometry_smith(normal, V, L, roughness);

        var numerator = D*F*G;
        var denominator = 4.0 * max(dot(normal, V), 0.0) * max(dot(normal, L), 0.0) + 0.001;
        var specular = numerator / denominator;

        // Calculate ratio of reflected-refracted light.
        var kS = F;
        var kD = vec3f(1.0) - kS;

        kD *= 1.0 - metallic;	

        // Calculate output radiance.
        var NdotL = max(dot(normal, L), 0.0);

        var Loi = (kD * albedo / PI + specular) * light_radiance * NdotL;

        // add to outgoing radiance Lo
        var shadow = shadow(vsout, i);
        Lo += Loi * (1.0 - shadow);
    }

    // ambient lighting.
    var F = fresnel_schlick_roughness(max(dot(normal, V), 0.0), F0, roughness);
    var kS = F;
    var kD = vec3f(1.0) - kS;
    kD *= 1.0 - metallic;
    
    var irradiance = textureSample(irrMap, irrSampler, normal).rgb;
    var diffuse = irradiance * albedo;
    var ambient = (kD * diffuse) * ao;
    
    var color = ambient + Lo;
    return vec4f(color, 1.0);

}

fn calc_attenuation(vsout: VSOut, light: PointLight) -> f32 {
    var d: f32 = length(light.position - vsout.world_position.xyz);

    var constant = 1.0;
    var linear = 0.027;
    var quadratic = 0.0028;

    var attenuation: f32 = 1.0 / (constant + linear * d + quadratic * (d * d));

    return attenuation;
}

fn shadow(vsout: VSOut, i: u32) -> f32 {
    let light = lights[i];
    let tex = shadow_maps[i];
    let samp = shadow_maps_samplers[i];

    var zFar = 25.0;
    var zNear = 0.5;
    var bias = 0.01;

    var fragToLight: vec3f = vsout.world_position.xyz - light.position;

    var currentDepth = length(fragToLight) / zFar;

    var closestDepth = textureSample(tex, samp, fragToLight).r;

    var shadow = select(0.0, 1.0, currentDepth - closestDepth > bias);
    return shadow;
}

fn distribution_ggx(N: vec3f, H: vec3f, roughness: f32) -> f32 {
    var a = roughness * roughness;
    var a2 = a*a;
    var NdotH  = max(dot(N, H), 0.0);
    var NdotH2 = NdotH*NdotH;
	
    var nom = a2;
    var denom  = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
	
    return nom / denom;    
}

fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    var r = (roughness + 1.0);
    var k = (r * r) / 8.0;

    var nom   = NdotV;
    var denom = NdotV * (1.0 - k) + k;
	
    return nom / denom;
}
  
fn geometry_smith(N: vec3f, V: vec3f, L: vec3f, roughness: f32) -> f32 {
    var NdotV = max(dot(N, V), 0.0);
    var NdotL = max(dot(N, L), 0.0);
    var ggx1 = geometry_schlick_ggx(NdotV, roughness);
    var ggx2 = geometry_schlick_ggx(NdotL, roughness);
	
    return ggx1 * ggx2;
}

fn fresnel_schlick(cosTheta: f32, F0: vec3f) -> vec3f {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

fn fresnel_schlick_roughness(cosTheta: f32, F0: vec3f, roughness: f32) -> vec3f {
    return F0 + (max(vec3(1.0 - roughness), F0) - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}


fn get_albedo(uv: vec2f) -> vec3f {
     var textureColor: vec4f;
    if (textureDimensions(baseColorTexture).x > 1) {
        textureColor = textureSample(baseColorTexture, baseColorSampler, uv);
    } else {
        textureColor = vec4f(1.0, 1.0, 1.0, 1.0);
    }
    return (material.base_color * textureColor).rgb;
}

fn get_normal(vsout: VSOut) -> vec3f {
    var normal: vec3f;
    if (textureDimensions(normalTexture).x > 1) {
        var tbn_matrix = mat3x3f(vsout.tangent, vsout.bitangent, vsout.normal);
        normal = textureSample(normalTexture, normalSampler, vsout.uv).xyz;
        normal = normal * 2.0 - 1.0;
        normal = normalize(tbn_matrix * normal);
    } else {
        normal = vsout.normal;
    }
    return normal;
}

fn get_metalness(uv: vec2f) -> f32 {
    var metalness: f32;
    if (textureDimensions(metallicRoughnessTexture).x > 1) {
        metalness = textureSample(metallicRoughnessTexture, metallicRoughnessSampler, uv).b;
    } else {
        metalness = material.metallic_factor;
    }
    return metalness;
}

fn get_roughness(uv: vec2f) -> f32 {
    var roughness: f32;
    if (textureDimensions(metallicRoughnessTexture).x > 1) {
        roughness = textureSample(metallicRoughnessTexture, metallicRoughnessSampler, uv).g;
    } else {
        roughness = material.roughness_factor;
    }
    return roughness;
}

fn get_ambient_occlussion(uv: vec2f) -> f32 {
    var ao: f32;
    if (textureDimensions(ambientOcclussionTexture).x > 1) {
        ao = textureSample(ambientOcclussionTexture, ambientOcclussionSampler, uv).g;
    } else {
        ao = 1.0;
    }
    return ao;
}