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
    @location(2) world_position: vec4f,
    @location(3) tangent: vec3f,
    @location(4) bitangent: vec3f,
}

struct Camera {
    view: mat4x4f,
    rotation: mat4x4f,
    projection: mat4x4f,
    position: vec3f,
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

    vsout.position = camera.projection * camera.view * vertex_world_position;
    vsout.uv = vertex.uv;
    vsout.normal = (transform.normal * vec4f(vertex.normal, 1.0)).xyz;
    vsout.world_position = vertex_world_position;
    vsout.tangent = vertex.tangent;
    vsout.bitangent = vertex.bitangent;
    
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
@group(2) @binding(5) var metallicRoughnessTextureSampler: sampler;
@group(2) @binding(6) var metallicRoughnessTextureTexture: texture_2d<f32>;

struct PointLight {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
}

@group(3) @binding(0) var<storage, read> lights: array<PointLight>;
@group(3) @binding(1) var shadow_maps: binding_array<texture_cube<f32>, 3>;
@group(3) @binding(2) var shadow_maps_samplers: binding_array<sampler, 3>;



@fragment 
fn fs_main(vsout: VSOut) -> @location(0) vec4f {
    
    var albedo = get_albedo(vsout.uv);
    var normal = get_normal(vsout);
    var metalness = get_metalness(vsout.uv);
    var roughness = get_roughness(vsout.uv);

    /// CALCULATE PBR
    var world_position = vsout.world_position.xyz;

    var N = normal;
    var V = normalize(camera.position - world_position);

    var F0 = vec3f(0.04); 
    F0 = mix(F0, albedo, metalness);

    // reflectance equation
    var Lo = vec3f(0.0);
    for (var i: u32 = 0; i < arrayLength(&lights); i = i + 1 ) {
        var light = lights[i];
        
        // calculate per-light radiance
        var L = normalize(light.position - world_position);
        var H = normalize(V + L);

        var attenuation = calc_attenuation(vsout, light);

        var radiance = light.color * attenuation;

         // Cook-Torrance BRDF
        var NDF = calc_distribution_GGX(N, H, roughness);
        var G = calc_geometry_Smith(N, V, L, roughness);      
        var F = calc_fresnel_Schlick(clamp(dot(H, V), 0.0, 1.0), F0);
           
        var numerator = NDF * G * F; 
        var denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001; // + 0.0001 to prevent divide by zero
        var specular = numerator / denominator;

        var kS = F;
        var kD = vec3f(1.0) - kS;
        kD *= 1.0 - metalness;	  

        // scale light by NdotL
        var NdotL = max(dot(N, L), 0.0);        
        var Loi = (kD * albedo / PI + specular) * radiance * NdotL;

        // TODO figure out why some components are negative and remove this :-D
        Loi.x = max(Loi.x, 0.0);
        Loi.y = max(Loi.y, 0.0);
        Loi.z = max(Loi.z, 0.0);
        
        // add to outgoing radiance Lo
        var shadow = calc_shadow(vsout, i);
        Lo += Loi * (1.0 - shadow);  // note that we already multiplied the BRDF by the Fresnel (kS) so we won't multiply by kS again
    }


    // ambient lighting.
    var ambient = vec3f(0.03) * albedo; //* ao;
    
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

fn calc_shadow(vsout: VSOut, i: u32) -> f32 {
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

fn calc_distribution_GGX(N: vec3f, H: vec3f, a: f32) -> f32 {
    var a2 = a*a;
    var NdotH  = max(dot(N, H), 0.0);
    var NdotH2 = NdotH*NdotH;
	
    var nom = a2;
    var denom  = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
	
    return nom / denom;
}


fn calc_geometry_Schlick_GGX(NdotV: f32, k: f32) -> f32 {
    var nom   = NdotV;
    var denom = NdotV * (1.0 - k) + k;
	
    return nom / denom;
}
  
fn calc_geometry_Smith(N: vec3f, V: vec3f, L: vec3f, k: f32) -> f32 {
    var NdotV = max(dot(N, V), 0.0);
    var NdotL = max(dot(N, L), 0.0);
    var ggx1 = calc_geometry_Schlick_GGX(NdotV, k);
    var ggx2 = calc_geometry_Schlick_GGX(NdotL, k);
	
    return ggx1 * ggx2;
}

fn calc_fresnel_Schlick(cosTheta: f32, F0: vec3f) -> vec3f {
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
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
        normal = tbn_matrix * normal;
    } else {
        normal = normalize(vsout.normal);
    }
    return normal;
}

fn get_metalness(uv: vec2f) -> f32 {
    var metalness: f32;
    if (textureDimensions(metallicRoughnessTextureTexture).x > 1) {
        metalness = textureSample(metallicRoughnessTextureTexture, metallicRoughnessTextureSampler, uv).b;
    } else {
        metalness = material.metallic_factor;
    }
    return metalness;
}

fn get_roughness(uv: vec2f) -> f32 {
    var roughness: f32;
    if (textureDimensions(metallicRoughnessTextureTexture).x > 1) {
        roughness = textureSample(metallicRoughnessTextureTexture, metallicRoughnessTextureSampler, uv).g;
    } else {
        roughness = material.roughness_factor;
    }
    return roughness;
}