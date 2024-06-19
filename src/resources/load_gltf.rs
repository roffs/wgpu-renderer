use std::path::Path;

use gltf::{Gltf, Mesh as GltfMesh, Node as GltfNode, Scene as GltfScene};
use wgpu::{BindGroupLayout, Color, Device, Queue};

use crate::{
    entity::{Entity, Geometry, Mesh, Node, Vertex},
    material::Material,
};

use super::Resources;

impl Resources {
    pub fn load_gltf<'a>(
        device: &'a Device,
        queue: &'a Queue,
        layout: &'a BindGroupLayout,
        path: &'a Path,
    ) -> Entity {
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
        let mut materials =
            Resources::load_materials(device, queue, layout, &gltf, current_directory);

        let default_material = Material::new(
            device,
            layout,
            Color {
                r: 0.4,
                g: 0.4,
                b: 0.2,
                a: 1.0,
            },
            None,
            None,
        );

        materials.push(default_material); // Put default material at the end of the array

        // Load default scene
        let default_scene = gltf.default_scene().expect("Default scene not provided!");

        Resources::load_scene(device, default_scene, materials, buffers)
    }

    fn load_scene(
        device: &Device,
        scene: GltfScene,
        materials: Vec<Material>,
        buffers: Vec<Vec<u8>>,
    ) -> Entity {
        let mut nodes = vec![];

        for node in scene.nodes() {
            let node = Resources::load_node(device, node, &materials, &buffers);
            nodes.push(node);
        }

        Entity::new(nodes, materials)
    }

    fn load_node(
        device: &Device,
        node: GltfNode,
        materials: &Vec<Material>,
        buffers: &Vec<Vec<u8>>,
    ) -> Node {
        let mut children = vec![];

        for node in node.children() {
            children.push(Resources::load_node(device, node, materials, buffers));
        }

        if let Some(mesh) = node.mesh() {
            let mesh = Resources::load_mesh(device, &mesh, materials, buffers);

            return Node::Mesh {
                mesh,
                transform: None,
                children,
            };
        };

        if let Some(_camera) = node.camera() {
            todo!("Implement camera node")
        }

        Node::Empty {
            transform: None,
            children,
        }
    }

    fn load_mesh(
        device: &Device,
        mesh: &GltfMesh,
        materials: &[Material],
        buffers: &[Vec<u8>],
    ) -> Mesh {
        let mut primitives = vec![];

        for primitive in mesh.primitives() {
            let material_index = match primitive.material().index() {
                Some(i) => i,
                None => materials.len() - 1, // Default materials is in the last place of the array
            };

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            // Read vertex attributes
            let positions = reader.read_positions().unwrap().collect::<Vec<_>>();
            let uvs = reader
                .read_tex_coords(0)
                .map(|v| v.into_f32())
                .unwrap()
                .collect::<Vec<_>>();
            let normals = reader.read_normals().unwrap().collect::<Vec<_>>();
            let tangents = reader
                .read_tangents()
                .map(|iter| iter.map(|t| [t[0], t[1], t[2]]).collect::<Vec<_>>());

            let vertices = (0..positions.len())
                .map(|index| {
                    let tangent = match &tangents {
                        Some(t) => t[index],
                        None => [0.0, 0.0, 0.0],
                    };

                    Vertex::new(positions[index], uvs[index], normals[index], tangent)
                })
                .collect::<Vec<Vertex>>();

            let indices = reader
                .read_indices()
                .unwrap()
                .into_u32()
                .map(|i| i as u16)
                .collect();
            let geometry = Geometry::new(device, vertices, indices);

            primitives.push((geometry, material_index));
        }

        Mesh { primitives }
    }

    fn load_materials(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        gltf: &Gltf,
        current_directory: &Path,
    ) -> Vec<Material> {
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

        let mut materials = Vec::new();

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

        materials
    }
}
