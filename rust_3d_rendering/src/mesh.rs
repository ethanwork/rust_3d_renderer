use rand::Rng;
use sdl2::pixels::Color;

use crate::texture::Tex2;
use crate::{triangle::Face, vector::Vec3};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub translation: Vec3
}

pub fn get_cube_vertices() -> Vec<Vec3> {
        vec![
        Vec3 { x: -1.0, y: -1.0, z: -1.0 }, // 1
        Vec3 { x: -1.0, y:  1.0, z: -1.0 }, // 2
        Vec3 { x:  1.0, y:  1.0, z: -1.0 }, // 3
        Vec3 { x:  1.0, y: -1.0, z: -1.0 }, // 4
        Vec3 { x:  1.0, y:  1.0, z:  1.0 }, // 5
        Vec3 { x:  1.0, y: -1.0, z:  1.0 }, // 6
        Vec3 { x: -1.0, y:  1.0, z:  1.0 }, // 7
        Vec3 { x: -1.0, y: -1.0, z:  1.0 }, // 8
    ]
}

fn get_cube_faces() -> Vec<Face> {
    vec![
        // front
        Face { a: 1, b: 2, c: 3, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 0.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 1.0 }, color: Color::RGBA(255, 255, 255, 255) },
        Face { a: 1, b: 3, c: 4, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 1.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 0.0 }, color: Color::RGBA(255, 255, 255, 255) },
        // right
        Face { a: 4, b: 3, c: 5, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 0.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 1.0 }, color: Color::RGBA(255, 255, 255, 255) },
        Face { a: 4, b: 5, c: 6, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 1.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 0.0 }, color: Color::RGBA(255, 255, 255, 255) },
        // back
        Face { a: 6, b: 5, c: 7, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 0.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 1.0 }, color: Color::RGBA(255, 255, 255, 255) },
        Face { a: 6, b: 7, c: 8, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 1.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 0.0 }, color: Color::RGBA(255, 255, 255, 255) },
        // left
        Face { a: 8, b: 7, c: 2, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 0.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 1.0 }, color: Color::RGBA(255, 255, 255, 255) },
        Face { a: 8, b: 2, c: 1, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 1.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 0.0 }, color: Color::RGBA(255, 255, 255, 255) },
        // top
        Face { a: 2, b: 7, c: 5, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 0.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 1.0 }, color: Color::RGBA(255, 255, 255, 255) },
        Face { a: 2, b: 5, c: 3, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 1.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 0.0 }, color: Color::RGBA(255, 255, 255, 255) },
        // bottom
        Face { a: 6, b: 8, c: 1, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 0.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 1.0 }, color: Color::RGBA(255, 255, 255, 255) },
        Face { a: 6, b: 1, c: 4, a_uv: Tex2 { u: 0.0, v: 0.0 }, b_uv: Tex2 { u: 1.0, v: 1.0 }, c_uv: Tex2 { u: 1.0, v: 0.0 }, color: Color::RGBA(255, 255, 255, 255) }
    ]
}

pub fn get_cube_mesh() -> Mesh {
    Mesh {
        vertices: get_cube_vertices(),
        faces: get_cube_faces(),
        rotation: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        translation: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub fn load_obj_file_data(filename: &str) -> Mesh {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut vertices = Vec::<Vec3>::new();
    let mut faces = Vec::<Face>::new();

    let mut rng = rand::thread_rng();    

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() > 0 {
            if words[0] == "v" {
                let vec3 = Vec3 {
                    x: words[1].parse().unwrap_or(0.0),
                    y: words[2].parse().unwrap_or(0.0),
                    z: words[3].parse().unwrap_or(0.0),
                };
                vertices.push(vec3);
            }
            if words[0] == "f" {
                let a_strings: Vec<&str> = words[1].split('/').collect();
                let b_strings: Vec<&str> = words[2].split('/').collect();
                let c_strings: Vec<&str> = words[3].split('/').collect();
                let a: usize = a_strings[0].parse().unwrap_or(1);
                let b: usize = b_strings[0].parse().unwrap_or(1);
                let c: usize = c_strings[0].parse().unwrap_or(1);

                let color_r: u8 = rng.gen_range(0..=255);
                let color_g: u8 = rng.gen_range(0..=255);
                let color_b: u8 = rng.gen_range(0..=255);
                let face = Face { 
                    a, 
                    b, 
                    c, 
                    color: Color::RGBA(255, 255, 255, 255), 
                    a_uv: Tex2 { u: 0.0, v: 0.0 },
                    b_uv: Tex2 { u: 0.0, v: 0.0 },
                    c_uv: Tex2 { u: 0.0, v: 0.0 },
                };
                // let face = Face { a, b, c, color: Color::RGBA(color_r, color_g, color_b, 255) };
                faces.push(face);
            }
        }
    }

    return Mesh {
        vertices,
        faces,
        rotation: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scale: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        translation: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    };
}
