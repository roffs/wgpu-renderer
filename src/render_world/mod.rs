mod extracted_camera;
mod extracted_env_map;
mod extracted_material;
mod extracted_mesh;
mod extracted_point_light;
mod extracted_transform;
mod render_object;

use cgmath::Matrix4;
use extracted_env_map::ExtractedEnvMap;
use extracted_material::{extract_material, ExtractedMaterial};
use extracted_mesh::ExtractedMesh;
use extracted_point_light::{ExtractedPointLight, PointLightUniform};
use extracted_transform::ExtractedTransform;
use render_object::{DrawRenderObject, RenderObject};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BufferDescriptor, BufferUsages, Device, Queue,
    RenderPass, Sampler, TextureView,
};

pub use extracted_camera::ExtractedCamera;

use crate::{
    camera::Camera,
    entity::{Entity, Mesh, Node},
    environment_map::EnvironmentMap,
    layouts::Layouts,
    light::PointLight,
};

pub struct RenderWorld {
    objects: Vec<RenderObject>,
    pub camera: ExtractedCamera,
    materials: Vec<ExtractedMaterial>,
    pub lights: Vec<ExtractedPointLight>,
    pub lights_bind_group: BindGroup,
    pub env_map: ExtractedEnvMap,
}

impl RenderWorld {
    pub fn extract(
        device: &Device,
        queue: &Queue,
        layouts: &Layouts,
        entities: &[Entity],
        camera: &Camera,
        lights: &[PointLight],
        env_map: &EnvironmentMap,
    ) -> RenderWorld {
        let mut materials = vec![];
        let mut objects = vec![];

        for entity in entities {
            let mut entity_render_objects =
                extract_entity_render_objects(device, layouts, entity, materials.len());
            objects.append(&mut entity_render_objects);

            let mut entity_materials = extract_entity_materials(device, layouts, entity);
            materials.append(&mut entity_materials);
        }

        let camera = ExtractedCamera::new(device, &layouts.camera, camera);
        let lights = lights
            .iter()
            .map(|l| ExtractedPointLight::new(device, layouts, l))
            .collect::<Vec<_>>();
        let env_map = ExtractedEnvMap::new(device, &layouts.sky, env_map);

        // Create lights buffer
        let light_buffer_size = lights.len() * std::mem::size_of::<PointLightUniform>();

        let lights_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Model light buffer"),
            size: light_buffer_size as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Put all lights data into lights buffer
        let light_size = std::mem::size_of::<PointLightUniform>();
        for (index, light) in lights.iter().enumerate() {
            let light_data = unsafe {
                std::slice::from_raw_parts(
                    &light.uniform as *const PointLightUniform as *const u8,
                    light_size,
                )
            };

            queue.write_buffer(&lights_buffer, (light_size * index) as u64, light_data);
        }

        let view_array = lights
            .iter()
            .map(|light| &light.shadow_map.view)
            .collect::<Vec<&TextureView>>();

        let sampler_array = lights
            .iter()
            .map(|light| &light.shadow_map.sampler)
            .collect::<Vec<&Sampler>>();

        let lights_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Model light bind group"),
            layout: &layouts.light,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: lights_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureViewArray(&view_array),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::SamplerArray(&sampler_array),
                },
            ],
        });

        RenderWorld {
            objects,
            camera,
            materials,
            lights,
            lights_bind_group,
            env_map,
        }
    }
}

fn extract_entity_materials(
    device: &Device,
    layouts: &Layouts,
    entity: &Entity,
) -> Vec<ExtractedMaterial> {
    let mut entity_materials = vec![];
    for material in entity.get_materials() {
        let extracted_material = extract_material(device, &layouts.material, material);
        entity_materials.push(extracted_material);
    }

    entity_materials
}

fn extract_entity_render_objects(
    device: &Device,
    layouts: &Layouts,
    entity: &Entity,
    current_material_index: usize,
) -> Vec<RenderObject> {
    let mut render_objects = vec![];
    for node in entity.get_nodes() {
        let mut node_render_objects = extract_node(
            device,
            layouts,
            node,
            current_material_index,
            entity.transform.model(),
        );
        render_objects.append(&mut node_render_objects)
    }

    render_objects
}

fn extract_node(
    device: &Device,
    layouts: &Layouts,
    node: &Node,
    current_material_index: usize,
    parent_model_matrix: Matrix4<f32>,
) -> Vec<RenderObject> {
    let mut render_objects = vec![];

    if let Some(mesh) = &node.mesh {
        let mut mesh_render_objects = extract_mesh(device, mesh, current_material_index)
            .into_iter()
            .map(|(extracted_mesh, material_index)| {
                let transform = ExtractedTransform::new(
                    device,
                    &layouts.transform,
                    &node.transform,
                    parent_model_matrix,
                );
                RenderObject::new(extracted_mesh, transform, material_index)
            })
            .collect::<Vec<_>>();

        render_objects.append(&mut mesh_render_objects);
    }

    let local_transform = parent_model_matrix * node.transform.model();

    for child in &node.children {
        let mut child_render_objects = extract_node(
            device,
            layouts,
            child,
            current_material_index,
            local_transform,
        );
        render_objects.append(&mut child_render_objects);
    }

    render_objects
}

fn extract_mesh(
    device: &Device,
    mesh: &Mesh,
    current_material_index: usize,
) -> Vec<(ExtractedMesh, usize)> {
    let mut mesh_with_material_index = vec![];

    for (geometry, material_index) in &mesh.primitives {
        let mesh = ExtractedMesh::new(device, geometry);

        mesh_with_material_index.push((mesh, *material_index + current_material_index));
    }

    mesh_with_material_index
}

pub trait DrawWorld<'a> {
    fn draw_world(&mut self, world: &'a RenderWorld);
    fn draw_skybox(&mut self, world: &'a RenderWorld);
}

impl<'a> DrawWorld<'a> for RenderPass<'a> {
    fn draw_world(&mut self, world: &'a RenderWorld) {
        for render_object in &world.objects {
            self.draw_render_object(render_object, &world.materials)
        }
    }

    fn draw_skybox(&mut self, world: &'a RenderWorld) {
        self.set_bind_group(1, &world.env_map, &[]);
        self.draw(0..3, 0..1)
    }
}
