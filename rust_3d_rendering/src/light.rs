use sdl2::pixels::Color;

use crate::vector::Vec3;

pub struct Light {
    pub direction: Vec3
}

pub fn light_apply_intensity(original_color: &Color, mut percentage_factor: f32) -> Color {
    percentage_factor = 
        if percentage_factor < 0.0 { 0.0 }
        else if percentage_factor > 1.0 { 1.0 }
        else { percentage_factor };
        
    let r = (original_color.r as f32 * percentage_factor) as u8;
    let g = (original_color.g as f32 * percentage_factor) as u8;
    let b = (original_color.b as f32 * percentage_factor) as u8;

    Color::RGBA(r, g, b, original_color.a)
}
