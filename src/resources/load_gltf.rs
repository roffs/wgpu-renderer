use std::path::Path;

use cgmath::{Vector2, Vector3};
use gltf::{Gltf, Mesh as GltfMesh, Node as GltfNode, Scene as GltfScene};
use wgpu::{Device, Queue};

use crate::{
    entity::{Entity, Geometry, Mesh, Node, Vertex},
    material::Material,
    texture::TextureType,
    transform::Transform,
};

use super::Resources;

impl Resources {
    //TODO move texture loading to extract world stage?
    pub fn load_gltf(device: &Device, queue: &Queue, path: &Path) -> Entity {
        let current_directory = path.parent().unwrap();

        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let gltf = gltf::Gltf::from_reader(reader).unwrap();

        // Load buffers
        let buffers = Resources::load_buffers(&gltf, current_directory);

        // Load materials
        let mut materials = Resources::load_materials(device, queue, &gltf, current_directory);

        let default_material =
            Material::new([0.4, 0.4, 0.2, 1.0], None, None, 0.0, 0.0, None, None);

        materials.push(default_material); // Put default material at the end of the array

        // Load default scene
        let default_scene = gltf.default_scene().expect("Default scene not provided!");

        Resources::load_scene(default_scene, materials, buffers)
    }

    fn load_scene(scene: GltfScene, materials: Vec<Material>, buffers: Vec<Vec<u8>>) -> Entity {
        let mut nodes = vec![];

        for node in scene.nodes() {
            let node = Resources::load_node(node, &materials, &buffers);
            nodes.push(node);
        }

        Entity::new(nodes, materials, Transform::zero())
    }

    fn load_node(node: GltfNode, materials: &Vec<Material>, buffers: &Vec<Vec<u8>>) -> Node {
        let transform = match node.transform() {
            gltf::scene::Transform::Matrix { .. } => {
                let t = node.transform().decomposed();
                Transform::new(t.0.into(), t.1.into(), t.2.into())
            }
            gltf::scene::Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => Transform::new(translation.into(), rotation.into(), scale.into()),
        };

        let mesh = node
            .mesh()
            .map(|m| Resources::load_mesh(&m, materials, buffers));

        let children = node
            .children()
            .map(|c| Resources::load_node(c, materials, buffers))
            .collect();

        Node {
            mesh,
            transform,
            children,
        }
    }

    fn load_mesh(mesh: &GltfMesh, materials: &[Material], buffers: &[Vec<u8>]) -> Mesh {
        let mut primitives = vec![];

        for primitive in mesh.primitives() {
            let material_index = match primitive.material().index() {
                Some(i) => i,
                None => materials.len() - 1, // Default materials is in the last place of the array
                                             // TODO: Make material_index an Option<> and move this logic into extract_world()?
            };

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            // Read vertex attributes
            let indices: Vec<u16> = reader
                .read_indices()
                .unwrap()
                .into_u32()
                .map(|i| i as u16)
                .collect();

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

            let tangents = tangents.unwrap_or_else(|| {
                let mut tangents = vec![[0.0; 3]; positions.len()];
                for i in indices.chunks(3) {
                    let i1 = i[0] as usize;
                    let i2 = i[1] as usize;
                    let i3 = i[2] as usize;

                    let pos0: Vector3<f32> = positions[i1].into();
                    let pos1: Vector3<f32> = positions[i2].into();
                    let pos2: Vector3<f32> = positions[i3].into();

                    let uv0: Vector2<f32> = uvs[i1].into();
                    let uv1: Vector2<f32> = uvs[i2].into();
                    let uv2: Vector2<f32> = uvs[i3].into();

                    let delta_pos1 = pos1 - pos0;
                    let delta_pos2 = pos2 - pos0;

                    let delta_uv1 = uv1 - uv0;
                    let delta_uv2 = uv2 - uv0;

                    let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
                    let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                    let tangent: [f32; 3] = tangent.into();
                    // let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

                    tangents[i1] = tangent;
                    tangents[i2] = tangent;
                    tangents[i3] = tangent;
                }
                tangents
            });

            let vertices = (0..positions.len())
                .map(|index| {
                    Vertex::new(
                        positions[index],
                        uvs[index],
                        normals[index],
                        tangents[index],
                    )
                })
                .collect::<Vec<Vertex>>();

            let geometry = Geometry::new(vertices, indices);

            primitives.push((geometry, material_index));
        }

        Mesh { primitives }
    }

    fn load_materials(
        device: &Device,
        queue: &Queue,
        gltf: &Gltf,
        current_directory: &Path,
    ) -> Vec<Material> {
        let load_texture =
            |texture: &gltf::Texture, texture_type: TextureType| match texture.source().source() {
                gltf::image::Source::View { .. } => {
                    todo!()
                }
                gltf::image::Source::Uri { uri, .. } => {
                    let path = current_directory.join(uri);
                    Resources::load_texture(device, queue, &path, texture_type)
                }
            };

        let mut materials = Vec::new();

        let load_material = |material: gltf::Material| {
            let pbr_metallic_roughness = material.pbr_metallic_roughness();

            let base_color = pbr_metallic_roughness.base_color_factor();

            let diffuse_texture = pbr_metallic_roughness
                .base_color_texture()
                .map(|diffuse| load_texture(&diffuse.texture(), TextureType::Diffuse));

            let normal_texture = material
                .normal_texture()
                .map(|normal| load_texture(&normal.texture(), TextureType::Normal));

            let metallic_factor = pbr_metallic_roughness.metallic_factor();
            let roughness_factor = pbr_metallic_roughness.roughness_factor();

            let metallic_roughness_texture = material
                .pbr_metallic_roughness()
                .metallic_roughness_texture()
                .map(|texture| load_texture(&texture.texture(), TextureType::Diffuse));

            let ambient_occlusion_texture = material
                .occlusion_texture()
                .map(|texture| load_texture(&texture.texture(), TextureType::Diffuse));

            Material::new(
                base_color,
                diffuse_texture,
                normal_texture,
                metallic_factor,
                roughness_factor,
                metallic_roughness_texture,
                ambient_occlusion_texture,
            )
        };

        for material in gltf.materials() {
            let material = load_material(material);
            materials.push(material);
        }

        materials
    }

    fn load_buffers(gltf: &gltf::Gltf, current_directory: &Path) -> Vec<Vec<u8>> {
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

        buffers
    }
}
