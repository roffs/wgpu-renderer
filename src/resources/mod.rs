mod hdr_loader;
mod irr_map_generator;
mod load_gltf;
mod load_textures;
mod skybox_loader;

pub struct Resources;

pub use {hdr_loader::HdrLoader, skybox_loader::SkyboxLoader};
