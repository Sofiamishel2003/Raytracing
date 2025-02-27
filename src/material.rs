use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::color::Color;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub has_texture: bool,
    pub texture: Option<Arc<Texture>>,  
    pub emission_color: Option<Color>, 
    pub emission_intensity: f32,
}

impl Material {
    pub const fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        emission_color: Option<Color>,
        emission_intensity: f32,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            has_texture: false,
            texture: None,  // Sin textura
            emission_color,
            emission_intensity,
        }
    }

    pub const fn new_with_texture(
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture: Arc<Texture>,  
        emission_color: Option<Color>,
        emission_intensity: f32,
    ) -> Self {
        Material {
            diffuse: Color::new(255, 255, 255), 
            specular,
            albedo,
            refractive_index,
            has_texture: true,
            texture: Some(texture),  
            emission_color,
            emission_intensity,
        }
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if self.has_texture {
            if let Some(texture) = &self.texture {
                let x = (u * (texture.width as f32 - 1.0)) as usize;
                let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
                return texture.get_color(x, y); 
            }
        }
        self.diffuse 
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 0.0,
            has_texture: false,
            texture: None,  
            emission_color: None,  
            emission_intensity: 0.0,
        }
    }
}