const PI: f32 = 3.1415926535897932384626433832795;

struct Face {
    forward: vec3<f32>,
    up: vec3<f32>,
    right: vec3<f32>,
}

struct VSOut {
    @builtin(position) frag_position: vec4<f32>,
    @location(0) clip_position: vec4<f32>,
};

@group(0)
@binding(0)
var env_map: texture_cube<f32>;

@group(0)
@binding(1)
var env_map_sampler: sampler;

@group(0)
@binding(2)
var dst: texture_storage_2d_array<rgba32float, write>;

var<push_constant> face_index: u32;

@compute
@workgroup_size(16, 16, 1)
fn compute_irr_map(
    @builtin(global_invocation_id)
    gid: vec3<u32>,
) {

    // If texture size is not divisible by 32 we
    // need to make sure we don't try to write to
    // pixels that don't exist.
    if gid.x >= u32(textureDimensions(dst).x) {
        return;
    }

    var FACES: array<Face, 6> = array(
        // FACES +X
        Face(
            vec3(1.0, 0.0, 0.0),  // forward
            vec3(0.0, 1.0, 0.0),  // up
            vec3(0.0, 0.0, -1.0), // right
        ),
        // FACES -X
        Face (
            vec3(-1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, 1.0),
        ),
        // FACES +Y
        Face (
            vec3(0.0, -1.0, 0.0),
            vec3(0.0, 0.0, 1.0),
            vec3(1.0, 0.0, 0.0),
        ),
        // FACES -Y
        Face (
            vec3(0.0, 1.0, 0.0),
            vec3(0.0, 0.0, -1.0),
            vec3(1.0, 0.0, 0.0),
        ),
        // FACES +Z
        Face (
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
            vec3(1.0, 0.0, 0.0),
        ),
        // FACES -Z
        Face (
            vec3(0.0, 0.0, -1.0),
            vec3(0.0, 1.0, 0.0),
            vec3(-1.0, 0.0, 0.0),
        ),
    );

    // Get texture coords relative to cubemap face
    let dst_dimensions = vec2<f32>(textureDimensions(dst));
    let cube_uv = (vec2<f32>(gid.xy) / dst_dimensions) * 2.0 - 1.0; // Mapping to [-1, 1]

    // Get spherical coordinate from cube_uv
    let face = FACES[face_index];
    var normal = normalize(face.forward + face.right * cube_uv.x + face.up * cube_uv.y);
    normal.y *= -1.0;

    var irradiance = vec3f(0.0, 0.0, 0.0);

    var up = vec3f(0.0, 1.0, 0.0);
    var right = normalize(cross(normal, up));
    up = normalize(cross(right, normal));

    var sample_delta =  0.005;
    var nr_samples: u32 = 0; 
    for(var phi = 0.0; phi < 2.0 * PI; phi += sample_delta)
    {
        for(var theta = 0.0; theta < 0.5 * PI; theta += sample_delta)
        {
            // spherical to cartesian (in tangent space)
            var tangent_sample  = vec3f(sin(theta) * cos(phi),  sin(theta) * sin(phi), cos(theta));
            // tangent space to world
            var sample_vec = tangent_sample .x * right + tangent_sample .y * up + tangent_sample .z * normal; 

            irradiance += textureSampleLevel(env_map, env_map_sampler, sample_vec, 0.0).rgb * cos(theta) * sin(theta);
            nr_samples++;
        }
    }
    
    irradiance = PI * irradiance / f32(nr_samples);
    
    textureStore(dst, gid.xy, face_index, vec4f(irradiance, 1.0));
}