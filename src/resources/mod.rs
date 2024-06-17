use std::path::Path;

use cgmath::Vector3;
use image::io::Reader;
use wgpu::{BindGroupLayout, Color, Device, Queue};

use crate::{
    material::Material,
    model::{Mesh, Model, Vertex},
    texture::{CubeMap, Texture},
};

pub struct Resources;

impl Resources {
    pub fn load_model(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        path: &Path,
    ) -> Model {
        let current_directory = path.parent().unwrap();

        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let gltf = gltf::Gltf::from_reader(reader).unwrap();

        // Load buffers
        let mut buffers = Vec::new();
        for buffer in gltf.buffers() {
            let buffer_data: Vec<u8> = match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    std::fs::read(&current_directory.join(uri)).expect("Failed to load binary")
                }
                gltf::buffer::Source::Bin => {
                    gltf.blob.as_deref().expect("Missing binary blob").into()
                }
            };
            buffers.push(buffer_data);
        }

        // Load materials
        let mut materials = Vec::new();

        // TODO remove duplication

        let load_texture = |texture: &gltf::Texture| match texture.source().source() {
            gltf::image::Source::View { .. } => {
                todo!()
            }
            gltf::image::Source::Uri { uri, .. } => {
                let path = current_directory.join(uri);
                Resources::load_texture(device, queue, &path)
            }
        };

        let load_normal_texture = |texture: &gltf::Texture| match texture.source().source() {
            gltf::image::Source::View { .. } => {
                todo!()
            }
            gltf::image::Source::Uri { uri, .. } => {
                let path = current_directory.join(uri);
                Resources::load_normal_texture(device, queue, &path)
            }
        };

        let load_material = |material: gltf::Material| {
            let diffuse_texture = material
                .pbr_metallic_roughness()
                .base_color_texture()
                .map(|diffuse| load_texture(&diffuse.texture()));

            let normal_texture = material
                .normal_texture()
                .map(|normal| load_normal_texture(&normal.texture()));

            Material::new(
                device,
                layout,
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                diffuse_texture,
                normal_texture,
            )
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

            let mut material_index = 0;

            for primitive in mesh.primitives() {
                material_index = primitive.material().index().unwrap(); //TODO can we extract this outside of the for loop? We wanna set the material once per mesh

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                // TODO: better error handling if we can not find some attribute or indices

                // Read vertex attributes
                let positions = reader.read_positions().unwrap();
                let mut uvs = reader.read_tex_coords(0).map(|v| v.into_f32());
                let mut normals = reader.read_normals();
                let mut tangents = reader.read_tangents();

                positions.for_each(|pos| {
                    let uv = uvs
                        .as_mut()
                        .and_then(|uvs| uvs.next().map(|uv| (uv[0], uv[1])))
                        .unwrap();

                    let normal = normals
                        .as_mut()
                        .and_then(|ns| ns.next().map(|n| (n[0], n[1], n[2])))
                        .unwrap();

                    let tangent = tangents
                        .as_mut()
                        .and_then(|ts| ts.next().map(|t| (t[0], t[1], t[2])))
                        .unwrap_or((0.0, 0.0, 0.0));

                    let bitangent = Vector3::from(normal).cross(Vector3::from(tangent));

                    mesh_vertices.push(Vertex::new(
                        pos.into(),
                        uv,
                        normal,
                        tangent,
                        bitangent.into(),
                    ));
                });

                // Read vertex indices
                let indices = reader.read_indices().unwrap();
                indices
                    .into_u32()
                    .for_each(|index| mesh_indices.push(index as u16));
            }

            meshes.push((
                Mesh::new(device, &mesh_vertices, &mesh_indices),
                material_index,
            ));
        }
        Model::new(meshes, materials)
    }

    pub fn load_texture(device: &Device, queue: &Queue, path: &Path) -> Texture {
        let image = Reader::open(path).unwrap().decode().unwrap();

        let width = image.width();
        let height = image.height();

        let data = image.to_rgba8();

        let label = format!("{}", path.display());

        Texture::new_with_data(device, queue, width, height, &data, Some(label.as_str()))
    }

    pub fn load_normal_texture(device: &Device, queue: &Queue, path: &Path) -> Texture {
        let image = Reader::open(path).unwrap().decode().unwrap();

        let width = image.width();
        let height = image.height();

        let data = image.to_rgba8();

        let label = format!("{}", path.display());

        Texture::new_normal_with_data(device, queue, width, height, &data, Some(label.as_str()))
    }

    pub fn load_cube_map(device: &Device, queue: &Queue, paths: [&Path; 6]) -> CubeMap {
        let mut data = Vec::new();

        let mut width = 0_u32;
        let mut height = 0_u32;

        for path in paths {
            let image = Reader::open(path).unwrap().decode().unwrap().to_rgba8();

            width = image.width();
            height = image.height();

            data.append(&mut image.into_raw());
        }

        CubeMap::new_with_data(device, queue, width, height, &data, Some("Cubemap texture"))
    }
}
