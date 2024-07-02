use wgpu::RenderPass;

use super::extracted::{DrawMesh, ExtractedMaterial, ExtractedMesh, ExtractedTransform};

pub struct RenderObject {
    mesh: ExtractedMesh,
    transform: ExtractedTransform,
    material_index: usize,
}

impl RenderObject {
    pub fn new(
        mesh: ExtractedMesh,
        transform: ExtractedTransform,
        material_index: usize,
    ) -> RenderObject {
        RenderObject {
            mesh,
            transform,
            material_index,
        }
    }
}

pub trait DrawRenderObject<'a> {
    fn draw_render_object(&mut self, object: &'a RenderObject, materials: &'a [ExtractedMaterial]);
}

impl<'a> DrawRenderObject<'a> for RenderPass<'a> {
    fn draw_render_object(&mut self, object: &'a RenderObject, materials: &'a [ExtractedMaterial]) {
        let material = &materials[object.material_index];
        self.set_bind_group(1, &object.transform, &[]);
        self.set_bind_group(2, material, &[]);
        self.draw_mesh(&object.mesh);
    }
}
