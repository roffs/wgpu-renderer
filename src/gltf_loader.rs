use wgpu::{BindGroupLayout, Device, Queue};

use crate::{
    material::Material,
    model::{Mesh, Model, Vertex},
    texture::Texture,
};

pub fn load_gltf(
    device: &Device,
    queue: &Queue,
    texture_bind_group_layout: &BindGroupLayout,
    path: &str,
) -> Model {
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

    let load_texture =
        |device: &Device, queue: &Queue, layout: &BindGroupLayout, texture: &gltf::Texture| {
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
                    Texture::new(
                        device,
                        queue,
                        layout,
                        &path,
                        Some(format!("{}", path.display()).as_str()),
                    )
                }
            }
        };

    let load_material = |material: gltf::Material| {
        let diffuse = material
            .pbr_metallic_roughness()
            .base_color_texture()
            .unwrap()
            .texture();
        let diffuse = load_texture(device, queue, texture_bind_group_layout, &diffuse);

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

        meshes.push((Mesh::new(device, queue, &mesh_vertices, &mesh_indices), 0));
    }
    Model::new(meshes, materials)
}
