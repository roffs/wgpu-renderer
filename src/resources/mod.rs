use std::path::Path;

use image::io::Reader;
use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferDescriptor, BufferUsages, Device,
    Extent3d, ImageCopyTextureBase, ImageDataLayout, Origin3d, Queue, TextureDescriptor,
};

use crate::{
    material::Material,
    model::{Mesh, Model, Vertex},
    texture::Texture,
};

pub struct Resources<'a> {
    device: &'a Device,
    queue: &'a Queue,
}

impl<'a> Resources<'a> {
    pub fn new(device: &'a Device, queue: &'a Queue) -> Resources<'a> {
        Resources { device, queue }
    }

    pub fn load_model(&self, layout: &BindGroupLayout, path: &str) -> Model {
        let relative_path = std::path::Path::new(path);
        let current_directory = relative_path.parent().unwrap();

        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let gltf = gltf::Gltf::from_reader(reader).unwrap();

        // Load buffers
        let mut buffer_data = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    let binary_data =
                        std::fs::read(&current_directory.join(uri)).expect("Failed to load binary");
                    buffer_data.push(binary_data);
                }
                gltf::buffer::Source::Bin => {
                    if let Some(blob) = gltf.blob.as_deref() {
                        buffer_data.push(blob.into())
                    };
                }
            }
        }

        // Load materials
        let mut materials = Vec::new();

        let load_texture = |layout: &BindGroupLayout, texture: &gltf::Texture| {
            match texture.source().source() {
                gltf::image::Source::View {
                    view: _,
                    mime_type: _,
                } => {
                    // let start = view.offset();
                    // let end = view.offset() + view.length();
                    // let data = &buffer_data[view.buffer().index()][start..end];
                    todo!()
                }
                gltf::image::Source::Uri { uri, mime_type: _ } => {
                    let path = current_directory.join(uri);
                    self.load_texture(layout, &path, Some(format!("{}", path.display()).as_str()))
                }
            }
        };

        let load_material = |material: gltf::Material| {
            let diffuse = material
                .pbr_metallic_roughness()
                .base_color_texture()
                .unwrap()
                .texture();
            let diffuse = load_texture(layout, &diffuse);

            // let normal = material.normal_texture().unwrap().texture();
            // let normal = load_texture(&normal);

            Material::new(diffuse)
        };

        for material in gltf.materials() {
            let material = load_material(material);
            materials.push(material);
        }

        // Load meshes
        let mut meshes = Vec::new();

        for mesh in gltf.meshes() {
            let mut mesh_vertices = Vec::new();
            let mut mesh_indices = Vec::new();

            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

                // TODO: better error handling if we can not find some attribute or indices

                // Read vertex attributes
                let positions = reader.read_positions().unwrap();
                let uvs = reader.read_tex_coords(0).map(|v| v.into_f32()).unwrap();
                // let normals = reader.read_normals().unwrap();
                // let tangents = reader.read_tangents().unwrap();

                // positions.zip(uvs).zip(normals).zip(tangents).for_each(
                //     |(((pos, uv), normal), tangent)| {
                //         let normal: Vector3<f32> = normal.into();
                //         let tangent: Vector3<f32> = [tangent[0], tangent[1], tangent[2]].into();
                //         let bitangent = normal.cross(tangent);

                //         mesh_vertices.push(Vertex::new(pos, uv));
                //     },
                // );

                positions.zip(uvs).for_each(|(pos, uv)| {
                    // let normal: Vector3<f32> = normal.into();
                    // let tangent: Vector3<f32> = [tangent[0], tangent[1], tangent[2]].into();
                    // let bitangent = normal.cross(tangent);

                    mesh_vertices.push(Vertex::new(pos.into(), uv.into()));
                });

                // Read vertex indices
                let indices = reader.read_indices().unwrap();
                indices
                    .into_u32()
                    .for_each(|index| mesh_indices.push(index as u16));
            }

            meshes.push((self.new_mesh(&mesh_vertices, &mesh_indices), 0));
        }
        Model::new(meshes, materials)
    }

    pub fn new_mesh(&self, vertices: &[Vertex], indices: &[u16]) -> Mesh {
        let vertex_buffer_data = as_u8_slice(vertices);
        let index_buffer_data = as_u8_slice(indices);

        let vertex_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Vertex buffer"),
            size: vertex_buffer_data.len() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.queue
            .write_buffer(&vertex_buffer, 0, vertex_buffer_data);

        let index_buffer = self.device.create_buffer(&BufferDescriptor {
            label: Some("Index buffer"),
            size: index_buffer_data.len() as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        self.queue.write_buffer(&index_buffer, 0, index_buffer_data);

        Mesh::new(vertex_buffer, index_buffer, indices.len() as u32)
    }

    // TODO improve
    pub fn _new_mesh_cube(&self) -> Mesh {
        let vertices = &[
            // FRONT
            Vertex::new((-0.5, -0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (1.0, 0.0)),
            Vertex::new((0.5, 0.5, 0.5), (1.0, 1.0)),
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 1.0)),
            // BACK
            Vertex::new((-0.5, -0.5, -0.5), (1.0, 0.0)),
            Vertex::new((0.5, -0.5, -0.5), (0.0, 0.0)),
            Vertex::new((0.5, 0.5, -0.5), (0.0, 1.0)),
            Vertex::new((-0.5, 0.5, -0.5), (1.0, 1.0)),
            // LEFT
            Vertex::new((-0.5, -0.5, -0.5), (0.0, 0.0)),
            Vertex::new((-0.5, -0.5, 0.5), (1.0, 0.0)),
            Vertex::new((-0.5, 0.5, 0.5), (1.0, 1.0)),
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 1.0)),
            // RIGHT
            Vertex::new((0.5, -0.5, -0.5), (1.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, 0.5, 0.5), (0.0, 1.0)),
            Vertex::new((0.5, 0.5, -0.5), (1.0, 1.0)),
            // TOP
            Vertex::new((-0.5, 0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, 0.5, 0.5), (1.0, 0.0)),
            Vertex::new((0.5, 0.5, -0.5), (1.0, 1.0)),
            Vertex::new((-0.5, 0.5, -0.5), (0.0, 1.0)),
            // BOTTOM
            Vertex::new((-0.5, -0.5, 0.5), (1.0, 0.0)),
            Vertex::new((0.5, -0.5, 0.5), (0.0, 0.0)),
            Vertex::new((0.5, -0.5, -0.5), (0.0, 1.0)),
            Vertex::new((-0.5, -0.5, -0.5), (1.0, 1.0)),
        ];

        let indices: &[u16] = &[
            0, 1, 2, 0, 2, 3, // FRONT
            4, 6, 5, 4, 7, 6, // BACK
            8, 9, 10, 8, 10, 11, // LEFT
            12, 14, 13, 12, 15, 14, // RIGHT
            16, 17, 18, 16, 18, 19, // TOP
            20, 22, 21, 20, 23, 22, // BOTTOM
        ];

        self.new_mesh(vertices, indices)
    }

    pub fn load_texture(
        &self,
        layout: &BindGroupLayout,
        path: &Path,
        label: Option<&str>,
    ) -> Texture {
        let image = Reader::open(path).unwrap().decode().unwrap();

        let texture_size = Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&TextureDescriptor {
            label,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image.to_rgba8(),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * image.width()),
                rows_per_image: Some(image.height()),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture bind group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
            ],
        });

        Texture::new(view, sampler, Some(bind_group))
    }

    pub fn new_depth_texture(&self, width: u32, height: u32, label: Option<&str>) -> Texture {
        let texture_size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&TextureDescriptor {
            label,
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Texture::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Texture::new(view, sampler, None)
    }
}

fn as_u8_slice<T: Sized>(data: &[T]) -> &[u8] {
    let size = std::mem::size_of_val(data);
    unsafe { std::slice::from_raw_parts(data as *const [T] as *const u8, size) }
}
