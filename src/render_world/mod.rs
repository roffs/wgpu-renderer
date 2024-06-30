mod extracted_camera;
mod extracted_material;
mod extracted_mesh;
mod extracted_skybox;
mod extracted_transform;
mod render_object;

use cgmath::Matrix4;
use extracted_camera::ExtractedCamera;
use extracted_material::{extract_material, ExtractedMaterial};
use extracted_mesh::ExtractedMesh;
use extracted_skybox::{DrawSkybox, ExtractedSkybox};
use extracted_transform::ExtractedTransform;
use render_object::{DrawRenderObject, RenderObject};
use wgpu::{Device, RenderPass};

use crate::{
    camera::Camera,
    entity::{Entity, Mesh, Node},
    layouts::Layouts,
    light::PointLight,
    skybox::Skybox,
};

pub struct RenderWorld<'a> {
    objects: Vec<RenderObject>,
    pub camera: ExtractedCamera,
    materials: Vec<ExtractedMaterial>,
    pub lights: &'a Vec<PointLight>,
    pub skybox: ExtractedSkybox,
}

impl<'a> RenderWorld<'a> {
    pub fn extract(
        device: &Device,
        layouts: &Layouts,
        entities: &[Entity],
        camera: &Camera,
        lights: &'a Vec<PointLight>,
        skybox: &'a Skybox,
    ) -> RenderWorld<'a> {
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
        let skybox = ExtractedSkybox::new(device, &layouts.skybox, skybox);

        RenderWorld {
            objects,
            camera,
            materials,
            lights,
            skybox,
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
    fn draw_sky(&mut self, world: &'a RenderWorld);
}

impl<'a> DrawWorld<'a> for RenderPass<'a> {
    fn draw_world(&mut self, world: &'a RenderWorld) {
        for render_object in &world.objects {
            self.draw_render_object(render_object, &world.materials)
        }
    }

    fn draw_sky(&mut self, world: &'a RenderWorld) {
        self.set_bind_group(0, &world.camera, &[]);
        self.draw_skybox(&world.skybox);
    }
}
