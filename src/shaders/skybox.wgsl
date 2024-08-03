struct VSOut {
    @builtin(position) frag_position: vec4<f32>,
    @location(0) clip_position: vec4<f32>,
}

@vertex 
fn vs_main(
    @builtin(vertex_index) id: u32,    
) -> VSOut {

    // Reference: https://sotrh.github.io/learn-wgpu/intermediate/tutorial13-hdr/#skybox
    let uv = vec2<f32>(vec2<u32>(
        id & 1u,
        (id >> 1u) & 1u,
    ));

    var vsout: VSOut;

    vsout.clip_position = vec4(uv * 4.0 - 1.0, 1.0, 1.0);
    vsout.frag_position = vec4(uv * 4.0 - 1.0, 1.0, 1.0);
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

@group(1) @binding(0) var env_sampler: sampler;
@group(1) @binding(1) var env_map: texture_cube<f32>;


// Couldnt we transform in the vertex shader? 

@fragment 
fn fs_main(in: VSOut) -> @location(0) vec4f {
    let view_pos_homogeneous = camera.inv_proj * in.clip_position;
    let view_ray_direction = view_pos_homogeneous.xyz / view_pos_homogeneous.w;
    var ray_direction = normalize((camera.inv_view * vec4(view_ray_direction, 0.0)).xyz);

    return textureSample(env_map, env_sampler, ray_direction);
}