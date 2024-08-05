use sdl2::pixels::Color;

use crate::{vector::{Vec4, vec2_sub, Vec2, Vec3}, display::Display, texture::Tex2, swap::swap};

#[derive(Debug, Clone)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub a_uv: Tex2,
    pub b_uv: Tex2,
    pub c_uv: Tex2,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: [Vec4; 3],
    pub texcoords: [Tex2; 3],
    pub color: Color,
    pub avg_depth: f32
}

pub fn draw_filled_triangle(
    display: &mut Display, 
    mut x0: usize, mut y0: usize,
    mut x1: usize, mut y1: usize,
    mut x2: usize, mut y2: usize,
    color: Color) {
    if y0 > y1 {
        swap(&mut y0, &mut y1);
        swap(&mut x0, &mut x1);
    }
    if y1 > y2 {
        swap(&mut y1, &mut y2);
        swap(&mut x1, &mut x2);
    }
    if y0 > y1 {
        swap(&mut y0, &mut y1);
        swap(&mut x0, &mut x1);
    }
    
    if y1 == y2 {
        // no need to draw botton triangle in this case
        fill_flat_bottom_triangle(display, x0, y0, x1, y1, x2, y2, color);
    } else if y0 == y1 {
        // no need to draw the top triangle in this case
        fill_flat_top_triangle(display, x0, y0, x1, y1, x2, y2, color);
    } else {
        // need to draw both halves of the triangle
        let my = y1;    
        let mx = ((((x2 as isize - x0 as isize) * (y1 as isize - y0 as isize)) as f32 / (y2 as isize - y0 as isize) as f32) as f32 + x0 as f32) as usize;

        fill_flat_bottom_triangle(display, x0, y0, x1, y1, mx as usize, my, color);
        fill_flat_top_triangle(display, x1, y1, mx as usize, my, x2, y2, color);
    }    
}

fn fill_flat_bottom_triangle(
    display: &mut Display, 
    x0: usize, y0: usize,
    x1: usize, y1: usize,
    x2: usize, y2: usize,
    color: Color) {
    let inv_slope_1 = if y1 != y0 {
        (x1 as isize - x0 as isize) as f32 / (y1 as isize - y0 as isize) as f32
    } else {
        0.0
    };

    let inv_slope_2 = if y2 != y0 {
        (x2 as isize - x0 as isize) as f32 / (y2 as isize - y0 as isize) as f32
    } else {
        0.0
    };

    let mut x_start = x0 as f32;
    let mut x_end = x0 as f32;

    for y in y0..=y2 {
        display.draw_line(x_start as usize, y, x_end as usize, y, color);
        x_start += inv_slope_1;
        x_end += inv_slope_2;            
    }
}

fn fill_flat_top_triangle(
    display: &mut Display, 
    x0: usize, y0: usize,
    x1: usize, y1: usize,
    x2: usize, y2: usize,
    color: Color) {
    let inv_slope_1 = if y2 != y0 {
        (x2 as isize - x0 as isize) as f32 / (y2 as isize - y0 as isize) as f32
    } else {
        0.0
    };

    let inv_slope_2 = if y2 != y1 {
        (x2 as isize - x1 as isize) as f32 / (y2 as isize - y1 as isize) as f32
    } else {
        0.0
    };

    let mut x_start = x2 as f32;
    let mut x_end = x2 as f32;

    for y in (y0..=y2).rev() {
        display.draw_line(x_start as usize, y, x_end as usize, y, color);
        x_start -= inv_slope_1;
        x_end -= inv_slope_2;            
    }
}

fn draw_texel(display: &mut Display, x: usize, y: usize, texture: &Vec<Color>, texture_width: usize, texture_height: usize,
    point_a: &Vec2, point_b: &Vec2, point_c: &Vec2,
    u0: f32, v0: f32, u1: f32, v1: f32, u2: f32, v2: f32) {
    let point_p = Vec2 { x: x as f32, y: y as f32 };
    let weights = barycentric_weights(point_a, point_b, point_c, &point_p);

    let alpha = weights.x;
    let beta = weights.y;
    let gamma = weights.z;

    let interpolated_u = u0 * alpha + u1 * beta + u2 * gamma;
    let interpolated_v = v0 * alpha + v1 * beta + v2 * gamma;

    let tex_x = (interpolated_u * texture_width as f32).abs() as usize;
    let tex_y = (interpolated_v * texture_height as f32).abs() as usize;

    let texture_index = (texture_width * tex_y) + tex_x;
    if texture_index < texture.len() {
        display.draw_pixel(x, y, texture[texture_index]);
    }
}

pub fn draw_textured_triangle(
    display: &mut Display, 
    mut x0: usize, mut y0: usize,
    mut u0: f32, mut v0: f32,
    mut x1: usize, mut y1: usize,
    mut u1: f32, mut v1: f32,
    mut x2: usize, mut y2: usize,
    mut u2: f32, mut v2: f32,
    texture: &Vec<Color>, texture_width: usize, texture_height: usize) {
    if y0 > y1 {
        swap(&mut y0, &mut y1);
        swap(&mut x0, &mut x1);
        swap(&mut u0, &mut u1);
        swap(&mut v0, &mut v1);
    }
    if y1 > y2 {
        swap(&mut y1, &mut y2);
        swap(&mut x1, &mut x2);
        swap(&mut u1, &mut u2);
        swap(&mut v1, &mut v2);
    }
    if y0 > y1 {
        swap(&mut y0, &mut y1);
        swap(&mut x0, &mut x1);
        swap(&mut u0, &mut u1);
        swap(&mut v0, &mut v1);
    }

    let point_a = Vec2 { x: x0 as f32, y: y0 as f32};
    let point_b = Vec2 { x: x1 as f32, y: y1 as f32};
    let point_c = Vec2 { x: x2 as f32, y: y2 as f32};
    
    // render top part of the triangle
    let mut inv_slope_1: f32 = 0.0;
    let mut inv_slope_2: f32 = 0.0; 

    if y1 as isize - y0 as isize != 0 {
        inv_slope_1 = (x1 as isize - x0 as isize) as f32 / ((y1 as isize - y0 as isize) as f32).abs();
    }    
    if y2 as isize - y0 as isize != 0 {
        inv_slope_2 = (x2 as isize - x0 as isize) as f32 / ((y2 as isize - y0 as isize) as f32).abs();
    }
    
    if y1 as isize - y0 as isize != 0 {
        for y in y0..=y1 {
            let mut x_start = (x1 as f32 + (y as isize - y1 as isize) as f32 * inv_slope_1) as usize;
            let mut x_end = (x0 as f32 + (y as isize - y0 as isize) as f32 * inv_slope_2) as usize;
    
            if x_end < x_start {
                swap(&mut x_start, &mut x_end);
            }
    
            for x in x_start..x_end {
                draw_texel(display, x, y, texture, texture_width, texture_height, &point_a, &point_b, &point_c, u0, v0, u1, v1, u2, v2);
            }
        }
    }

    // render the bottom part of the triangle
    inv_slope_1 = 0.0;
    inv_slope_2 = 0.0;

    if y2 as isize - y1 as isize != 0 {
        inv_slope_1 = (x2 as isize - x1 as isize) as f32 / ((y2 as isize - y1 as isize) as f32).abs();
    }    
    if y2 as isize - y0 as isize != 0 {
        inv_slope_2 = (x2 as isize - x0 as isize) as f32 / ((y2 as isize - y0 as isize) as f32).abs();
    }
    
    if y2 as isize - y1 as isize != 0 {
        for y in y1..=y2 {
            let mut x_start = (x1 as f32 + (y as isize - y1 as isize) as f32 * inv_slope_1) as usize;
            let mut x_end = (x0 as f32 + (y as isize - y0 as isize) as f32 * inv_slope_2) as usize;
    
            if x_end < x_start {
                swap(&mut x_start, &mut x_end);
            }
    
            for x in x_start..x_end {
                draw_texel(display, x, y, texture, texture_width, texture_height, &point_a, &point_b, &point_c, u0, v0, u1, v1, u2, v2);
            }
        }
    }
}

pub fn barycentric_weights(a: &Vec2, b: &Vec2, c: &Vec2, p: &Vec2) -> Vec3 {
    let ac = vec2_sub(c, a);
    let ab = vec2_sub(b, a);
    let pc = vec2_sub(c, p);
    let pb = vec2_sub(b, p);
    let ap = vec2_sub(p, a);

    let area_parallelogram_abc = ac.x * ab.y - ac.y * ab.x;
    let alpha = (pc.x * pb.y - pc.y * pb.x) / area_parallelogram_abc;
    let beta = (ac.x * ap.y - ac.y * ap.x) / area_parallelogram_abc;
    let gamma = 1.0 - alpha - beta;

    Vec3 {
        x: alpha,
        y: beta,
        z: gamma
    }
}