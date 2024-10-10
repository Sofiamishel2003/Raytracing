use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        Color {
            r: Color::clamp(r),
            g: Color::clamp(g),
            b: Color::clamp(b),
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color::new(r as i32, g as i32, b as i32) 
    }

    fn clamp(value: i32) -> u8 {
        if value < 0 {
            0
        } else if value > 255 {
            255
        } else {
            value as u8
        }
    }

    pub fn add(&self, other: &Color) -> Color {
        Color {
            r: Color::clamp(self.r as i32 + other.r as i32),
            g: Color::clamp(self.g as i32 + other.g as i32),
            b: Color::clamp(self.b as i32 + other.b as i32),
        }
    }

    // Multiplicar un color por un número
    pub fn multiply(&self, scalar: f32) -> Color {
        Color {
            r: Color::clamp((self.r as f32 * scalar) as i32),
            g: Color::clamp((self.g as f32 * scalar) as i32),
            b: Color::clamp((self.b as f32 * scalar) as i32),
        }
    }

    pub const fn black() -> Self{
        Color {r: 0, g: 0, b: 0}
    }

    pub fn to_u32(&self) -> u32 {
        let r = (self.r as u32) << 16;  
        let g = (self.g as u32) << 8;   
        let b = self.b as u32;         

        r | g | b
    }
    
}

use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}


impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: Color::clamp((self.r as f32 * scalar) as i32),
            g: Color::clamp((self.g as f32 * scalar) as i32),
            b: Color::clamp((self.b as f32 * scalar) as i32),
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: Color::clamp(self.r as i32 + other.r as i32),
            g: Color::clamp(self.g as i32 + other.g as i32),
            b: Color::clamp(self.b as i32 + other.b as i32),
        }
    }
}