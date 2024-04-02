 
struct OurStruct {
    color: vec3f,
    scale: vec2f,
    offset: vec2f,
};

@group(0) @binding(0) var<storage, read> ourStructs: array<OurStruct>;

struct VSOutput {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
}

@vertex 
fn vs_main(
    @builtin(vertex_index) vertexIndex : u32,
    @builtin(instance_index) instanceIndex: u32
) -> VSOutput {

    var pos = array(
        vec2f( 0.0,  0.5),  
        vec2f(-0.5, -0.5),  
        vec2f( 0.5, -0.5)   
    );

    var ourStruct = ourStructs[instanceIndex];

    var vsOut: VSOutput;

    vsOut.position = vec4f(pos[vertexIndex] * ourStruct.scale + ourStruct.offset, 0.0, 1.0);
    vsOut.color = ourStruct.color;

    return vsOut;
}

@fragment 
fn fs_main(vsOut: VSOutput) -> @location(0) vec4f {
    return vec4f(vsOut.color, 1.0);
}