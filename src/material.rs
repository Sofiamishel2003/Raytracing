use std::sync::Arc;
use crate::texture::Texture;
use crate::color::Color;  // Importa Color correctamente

#[derive(Clone)]
pub struct Material {
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub texture: Option<Arc<Texture>>,
    pub emission_color: Option<Color>,  // Usa Color aquí
}

impl Material {
    pub fn new_with_texture(
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture: Option<Arc<Texture>>,
    ) -> Self {
        Material {
            specular,
            albedo,
            refractive_index,
            texture,
            emission_color: None,
        }
    }
}
