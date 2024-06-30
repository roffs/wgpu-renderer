use crate::texture::Texture;

pub struct Material {
    pub base_color: [f32; 4],
    pub base_texture: Option<Texture>,
    pub normal_texture: Option<Texture>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub metallic_roughness_texture: Option<Texture>,
    pub ambient_occlussion_texture: Option<Texture>,
}

impl Material {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_color: [f32; 4],
        base_texture: Option<Texture>,
        normal_texture: Option<Texture>,
        metallic_factor: f32,
        roughness_factor: f32,
        metallic_roughness_texture: Option<Texture>,
        ambient_occlussion_texture: Option<Texture>,
    ) -> Material {
        Material {
            base_color,
            base_texture,
            normal_texture,
            metallic_factor,
            roughness_factor,
            metallic_roughness_texture,
            ambient_occlussion_texture,
        }
    }
}
