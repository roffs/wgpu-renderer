use cgmath::Vector3;
use wgpu::{
    include_wgsl, BindGroupDescriptor, BindGroupEntry, Color, ColorTargetState, ColorWrites,
    CommandEncoderDescriptor, Device, Face, FragmentState, FrontFace, LoadOp, MultisampleState,
    Operations, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    StoreOp, TextureFormat, TextureUsages, VertexState,
};

use crate::{camera::Camera, layouts::Layouts, render_world::ExtractedCamera, texture::CubeMap};

pub struct IrrMapGenerator {
    texture_format: TextureFormat,
    pipeline: RenderPipeline,
    face_cameras: [ExtractedCamera; 6],
}

impl IrrMapGenerator {
    pub fn new(device: &Device, layouts: &Layouts) -> IrrMapGenerator {
        let shader = device.create_shader_module(include_wgsl!("convolution.wgsl"));

        let texture_format = CubeMap::RGBA_32_FLOAT;

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Irradiation map pipeline layout"),
            bind_group_layouts: &[&layouts.camera, &layouts.cube_map],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Irradiance pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: texture_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: true,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let camera_directions = [
            (-Vector3::<f32>::unit_x(), Vector3::<f32>::unit_y()),
            (Vector3::<f32>::unit_x(), Vector3::<f32>::unit_y()),
            (Vector3::<f32>::unit_y(), -Vector3::<f32>::unit_z()),
            (-Vector3::<f32>::unit_y(), Vector3::<f32>::unit_z()),
            (Vector3::<f32>::unit_z(), Vector3::<f32>::unit_y()),
            (-Vector3::<f32>::unit_z(), Vector3::<f32>::unit_y()),
        ];

        let face_cameras = camera_directions.map(|(look_dir, up)| {
            let camera = Camera::new_from_look_direction(
                (0.0, 0.0, 0.0),
                look_dir,
                up,
                90.0,
                1.0,
                0.5,
                25.0,
            );
            ExtractedCamera::new(device, &layouts.camera, &camera)
        });

        Self {
            pipeline,
            texture_format,
            face_cameras,
        }
    }

    pub fn generate(
        &self,
        device: &Device,
        queue: &Queue,
        env_map: &CubeMap,
        size: u32,
        layouts: &Layouts,
    ) -> CubeMap {
        let irr_map = CubeMap::new(
            device,
            size,
            size,
            self.texture_format,
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            Some("Irradiance map"),
        );

        let env_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Irradiance env map bind group"),
            layout: &layouts.cube_map,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&env_map.sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&env_map.view),
                },
            ],
        });
        for (camera_index, camera) in self.face_cameras.iter().enumerate() {
            let face_view = irr_map.create_face_view(camera_index);
            let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Irradiance map Encoder"),
            });

            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Irradiance map render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &face_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::WHITE),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, camera, &[]);
            pass.set_bind_group(1, &env_bind_group, &[]);
            pass.draw(0..3, 0..1);

            drop(pass);
            let encoder = encoder.finish();

            queue.submit(std::iter::once(encoder));
        }

        irr_map
    }
}
