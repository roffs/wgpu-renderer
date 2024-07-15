const PI: f32 = 3.1415926535897932384626433832795;

struct VSOut {
    @builtin(position) frag_position: vec4<f32>,
    @location(0) clip_position: vec4<f32>,
};


// TODO: Remove duplicated vs_main (from skybox.wgsl)?
@vertex
fn vs_main(
    @builtin(vertex_index) id: u32
) -> VSOut {

      // Reference: https://sotrh.github.io/learn-wgpu/intermediate/tutorial13-hdr/#skybox
    let uv = vec2<f32>(vec2<u32>(
        id & 1u,
        (id >> 1u) & 1u,
    )) * 4.0 - 1.0;

    var vsout: VSOut;

    vsout.clip_position = vec4(uv, 1.0, 1.0);
    vsout.frag_position = vec4(uv, 1.0, 1.0);
    return vsout;
}

struct Camera {
    position: vec3f,
    view: mat4x4f,
    inv_view: mat4x4f,
    proj: mat4x4f,
    inv_proj: mat4x4f,
}

@group(0) @binding(0) var<uniform> camera: Camera;

@group(1) @binding(0) 
var env_sampler: sampler;
@group(1) @binding(1) 
var env_map: texture_cube<f32>;

// Couldnt we transform in the vertex shader? 

@fragment 
fn fs_main(in: VSOut) -> @location(0) vec4f {
    let view_pos_homogeneous = camera.inv_proj * in.clip_position;
    let view_ray_direction = view_pos_homogeneous.xyz / view_pos_homogeneous.w;
    var normal = normalize((camera.inv_view * vec4f(view_ray_direction, 0.0)).xyz);

    return textureSample(env_map, env_sampler, normal);
    // var irradiance = vec3f(0.0, 0.0, 0.0);

    // var up = vec3f(0.0, 1.0, 0.0);
    // var right = normalize(cross(normal, up));
    // up = normalize(cross(right, normal));

    // var sample_delta = 0.1;
    // var nr_samples: u32 = 0; 
    // for(var phi = 0.0; phi < 2.0 * PI; phi += sample_delta)
    // {
    //     for(var theta = 0.0; theta < 0.5 * PI; theta += sample_delta)
    //     {
    //         // spherical to cartesian (in tangent space)
    //         var tangent_sample  = vec3f(sin(theta) * cos(phi),  sin(theta) * sin(phi), cos(theta));
    //         // tangent space to world
    //         var sample_vec = tangent_sample .x * right + tangent_sample .y * up + tangent_sample .z * normal; 

    //         irradiance += textureSample(env_map, env_sampler, sample_vec).rgb * cos(theta) * sin(theta);
    //         nr_samples++;
    //     }
    // }
    
    // irradiance = PI * irradiance / f32(nr_samples);

    // return vec4f(irradiance, 1.0);
}
