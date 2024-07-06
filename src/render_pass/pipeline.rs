use wgpu::{
    ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device,
    Face, FragmentState, FrontFace, MultisampleState, PipelineLayout, PolygonMode, PrimitiveState,
    PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    StencilState, TextureFormat, VertexBufferLayout, VertexState,
};

pub fn create_pipeline(
    device: &Device,
    layout: &PipelineLayout,
    vertex_layout: &[VertexBufferLayout],
    color_format: TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    shader: ShaderModuleDescriptor,
) -> RenderPipeline {
    let shader = device.create_shader_module(shader);

    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("HDR pipeline"),
        layout: Some(layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: vertex_layout,
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(ColorTargetState {
                format: color_format,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            unclipped_depth: false,
            polygon_mode: PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        }),
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
