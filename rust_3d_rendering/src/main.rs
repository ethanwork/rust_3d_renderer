mod camera;
mod display;
mod mesh;
mod matrix;
mod triangle;
mod vector;
mod light;
mod texture;
mod swap;

extern crate image;

use std::{
    thread,
    time::{Duration, Instant}, f32::consts::PI,
};

use display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH, RenderMode};

use light::{Light, light_apply_intensity};
use matrix::{mat4_make_scale, mat4_mul_vec4, mat4_make_translation, mat4_make_rotation_x, mat4_make_rotation_y, mat4_make_rotation_z, mat4_identity, mat4_mul_mat4, Mat4, mat4_make_perspective, mat4_mul_vec4_project};
use mesh::{load_obj_file_data, Mesh, get_cube_mesh};
use sdl2::{
    event::Event,
    keyboard::Scancode,
    pixels::{Color, PixelFormatEnum},
    Sdl,
};
use texture::{Tex2, get_redbrick_texture, read_png_to_colors};
use triangle::{Triangle, draw_filled_triangle, draw_textured_triangle};
use vector::{
    vec3_cross, vec3_dot, vec3_sub, Vec3, vec3_normalize, vec4_from_vec3, Vec4, vec3_from_vec4,
};

const FPS: f32 = 30.0;
const FRAME_TARGET_TIME: f32 = 1000.0 / FPS;

pub struct MainLoop {
    is_running: bool,
    sdl_context: Sdl,
    timer: Instant,
    previous_frame_time: u128,
    display: Display,
    mesh: Mesh,
    triangles_to_render: Vec<Triangle>,
    camera_position: Vec3,
    render_mode: RenderMode,
    backface_culling_enabled: bool,
    proj_matrix: Mat4,
    light: Light,
    texture_width: usize,
    texture_height: usize,
    mesh_texture: Vec<Color>
}

fn main() {
    let mut main_loop = MainLoop::new();
    main_loop.setup();
    main_loop.run_loop();
}

impl MainLoop {
    pub fn new() -> Self {
        let mut sdl_context = sdl2::init().unwrap();
        let timer = Instant::now();
        let display = Display::new(&mut sdl_context);
        //let mesh = load_obj_file_data("./assets/f22.obj");
        let mesh = get_cube_mesh();
        let triangles_to_render = vec![
            Triangle {
                points: [Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }; 3],
                color: Color::RGBA(0, 0, 0, 255),
                avg_depth: 0.0,
                texcoords: [Tex2 { u: 0.0, v: 0.0 }; 3]
            };
            mesh.faces.len() * 2
        ];

        let fov = PI / 3.0;
        let aspect = SCREEN_HEIGHT as f32 / SCREEN_WIDTH as f32;
        let znear = 0.1;
        let zfar = 100.0;

        MainLoop {
            is_running: true,
            sdl_context,
            timer,
            previous_frame_time: 0,
            display,
            mesh,
            triangles_to_render,
            camera_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            backface_culling_enabled: true,
            render_mode: RenderMode::FilledTriangles,
            proj_matrix: mat4_make_perspective(fov, aspect, znear, zfar),
            light: Light {
                direction: Vec3 {
                    x: 0.0, y: 0.0, z: 1.0
                }
            },
            texture_height: 64,
            texture_width: 64,
            mesh_texture: read_png_to_colors("./assets/cube_texture.png").unwrap() //get_redbrick_texture()
        }
    }

    pub fn setup(&mut self) {}

    pub fn run_loop(&mut self) {
        while self.is_running {
            self.process_input();
            self.update();
            self.render();
        }
    }

    fn process_input(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.is_running = false;
                }
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(Scancode::Escape) {
            self.is_running = false;
        }
        if keyboard_state.is_scancode_pressed(Scancode::C) {
            self.backface_culling_enabled = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            self.backface_culling_enabled = false;            
        }
        if keyboard_state.is_scancode_pressed(Scancode::Num1) {
            self.render_mode = RenderMode::WireframeWithDot;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Num2) {
            self.render_mode = RenderMode::Wireframe;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Num3) {
            self.render_mode = RenderMode::FilledTriangles;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Num5) {
            self.render_mode = RenderMode::FilledTrianglesAndWireframe;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Num6) {
            self.render_mode = RenderMode::Textured;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Num7) {
            self.render_mode = RenderMode::TexturedAndWireframe;
        }
    }

    fn update(&mut self) {
        // get the amount of time to sleep until the next frame should start
        let time_to_wait: i128 = (FRAME_TARGET_TIME as i128)
            - (self.timer.elapsed().as_millis() as i128 - self.previous_frame_time as i128);
        if time_to_wait > 0 && time_to_wait <= (FRAME_TARGET_TIME as i128) {
            thread::sleep(Duration::from_millis(time_to_wait as u64));
        }
        self.previous_frame_time = self.timer.elapsed().as_millis();

        self.mesh.rotation.x += 0.01;
        self.mesh.rotation.y += 0.01;
        self.mesh.rotation.z += 0.01;

        // self.mesh.scale.x += 0.002;
        // self.mesh.scale.y += 0.001;
        
        // self.mesh.translation.x += 0.01;
        self.mesh.translation.z = 5.0;
        
        let scale_matrix = mat4_make_scale(self.mesh.scale.x, self.mesh.scale.y, self.mesh.scale.z);
        let translation_matrix = mat4_make_translation(self.mesh.translation.x, self.mesh.translation.y, self.mesh.translation.z);
        let rotation_matrix_x = mat4_make_rotation_x(self.mesh.rotation.x);
        let rotation_matrix_y = mat4_make_rotation_y(self.mesh.rotation.y);
        let rotation_matrix_z = mat4_make_rotation_z(self.mesh.rotation.z);

        // scale, then rotate, then translate in that order. otherwise issues occur
        // with the matrix multiplications
        let mut world_matrix = mat4_identity();
        world_matrix = mat4_mul_mat4(&scale_matrix, &world_matrix);
        world_matrix = mat4_mul_mat4(&rotation_matrix_x, &world_matrix);
        world_matrix = mat4_mul_mat4(&rotation_matrix_y, &world_matrix);
        world_matrix = mat4_mul_mat4(&rotation_matrix_z, &world_matrix);
        world_matrix = mat4_mul_mat4(&translation_matrix, &world_matrix);

        self.triangles_to_render.clear();

        // loop all triangle faces of our mesh
        for i in 0..self.mesh.faces.len() {
            let mesh_face = &self.mesh.faces[i];
            let face_vertices: [Vec3; 3] = [
                self.mesh.vertices[mesh_face.a - 1],
                self.mesh.vertices[mesh_face.b - 1],
                self.mesh.vertices[mesh_face.c - 1],
            ];

            let mut projected_triangle = Triangle {
                points: [Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }; 3],
                texcoords: [
                    mesh_face.a_uv.clone(),
                    mesh_face.b_uv.clone(),
                    mesh_face.c_uv.clone()
                ],
                color: mesh_face.color,
                avg_depth: 0.0
            };

            let mut transformed_vertices = [Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0
            }; 3];
            // transform the vertices
            for j in 0..3 {
                let mut transformed_vertex = vec4_from_vec3(&face_vertices[j]);

                transformed_vertex = mat4_mul_vec4(&world_matrix, &transformed_vertex);
                transformed_vertices[j] = transformed_vertex;
            }

            // check backface culling
            let vector_a = vec3_from_vec4(&transformed_vertices[0]);
            let vector_b = vec3_from_vec4(&transformed_vertices[1]);
            let vector_c = vec3_from_vec4(&transformed_vertices[2]);

            let mut vector_ab = vec3_sub(&vector_b, &vector_a);
            let mut vector_ac = vec3_sub(&vector_c, &vector_a);
            vec3_normalize(&mut vector_ab);
            vec3_normalize(&mut vector_ac);

            let mut normal = vec3_cross(&vector_ab, &vector_ac);
            vec3_normalize(&mut normal);

            let camera_ray = vec3_sub(&self.camera_position, &vector_a);

            let dot_normal_camera = vec3_dot(&normal, &camera_ray);

            // bypass the triangles facing away from the camera
            if dot_normal_camera < 0.0 && self.backface_culling_enabled {
                continue;
            }

            // project the vertices
            for j in 0..3 {
                projected_triangle.points[j] = mat4_mul_vec4_project(&self.proj_matrix, &transformed_vertices[j]);
                
                // scale into the view
                projected_triangle.points[j].x *= SCREEN_WIDTH as f32 / 2.0;
                projected_triangle.points[j].y *= SCREEN_HEIGHT as f32 / 2.0;

                // invert y values to account for flipped y screen coordinates
                projected_triangle.points[j].y *= -1.0;

                // translate to the middle of the screen
                projected_triangle.points[j].x += SCREEN_WIDTH as f32 / 2.0;
                projected_triangle.points[j].y += SCREEN_HEIGHT as f32 / 2.0;
            }
            let light_intensity_factor = -vec3_dot(&normal, &self.light.direction);
            projected_triangle.color = light_apply_intensity(&projected_triangle.color, light_intensity_factor);

            projected_triangle.avg_depth = 
                (transformed_vertices[0].z + 
                 transformed_vertices[1].z + 
                 transformed_vertices[2].z) / 3.0;
            self.triangles_to_render.push(projected_triangle);
        }

        // sort the triangles based upon avg_depth to implement a painters algorithm
        self.triangles_to_render.sort_unstable_by(
            |a, b| b.avg_depth.partial_cmp(&a.avg_depth).unwrap_or(std::cmp::Ordering::Equal));
    }

    fn render(&mut self) {
        for i in 0..self.triangles_to_render.len() {
            let triangle = &self.triangles_to_render[i];
            match self.render_mode {
                RenderMode::FilledTriangles | 
                RenderMode::FilledTrianglesAndWireframe => {
                    draw_filled_triangle(
                        &mut self.display,
                        triangle.points[0].x as usize,
                        triangle.points[0].y as usize,
                        triangle.points[1].x as usize,
                        triangle.points[1].y as usize,
                        triangle.points[2].x as usize,
                        triangle.points[2].y as usize,
                        triangle.color);    
                },
                _ => {}
            };

            match self.render_mode {
                RenderMode::Textured |
                RenderMode::TexturedAndWireframe => {
                    draw_textured_triangle(
                        &mut self.display,
                        triangle.points[0].x as usize,
                        triangle.points[0].y as usize,
                        triangle.texcoords[0].u,
                        triangle.texcoords[0].v,
                        triangle.points[1].x as usize,
                        triangle.points[1].y as usize,
                        triangle.texcoords[1].u,
                        triangle.texcoords[1].v,
                        triangle.points[2].x as usize,
                        triangle.points[2].y as usize,
                        triangle.texcoords[2].u,
                        triangle.texcoords[2].v,
                        &self.mesh_texture,
                        self.texture_width,
                        self.texture_height);
                },
                _ => {}
            };

            match self.render_mode {
                RenderMode::Wireframe |
                RenderMode::WireframeWithDot |
                RenderMode::FilledTrianglesAndWireframe |
                RenderMode::TexturedAndWireframe => {
                    self.display.draw_triangle(
                        triangle.points[0].x as usize,
                        triangle.points[0].y as usize,
                        triangle.points[1].x as usize,
                        triangle.points[1].y as usize,
                        triangle.points[2].x as usize,
                        triangle.points[2].y as usize,
                        Color::RGBA(0, 0, 255, 255));
                },
                _ => {}
            };

            match self.render_mode {
                RenderMode::WireframeWithDot => {
                    self.display.draw_rect((triangle.points[0].x - 3.0) as usize, (triangle.points[0].y - 3.0) as usize, 6, 6, Color::RGBA(255, 0, 0, 0));
                    self.display.draw_rect((triangle.points[1].x - 3.0) as usize, (triangle.points[1].y - 3.0) as usize, 6, 6, Color::RGBA(255, 0, 0, 0));
                    self.display.draw_rect((triangle.points[2].x - 3.0) as usize, (triangle.points[2].y - 3.0) as usize, 6, 6, Color::RGBA(255, 0, 0, 0));
                },
                _ => {}
            };
        }

        let texture_creator = self.display.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();

        self.display.render_color_buffer(&mut texture);
        self.display.clear_color_buffer(Color::RGBA(0, 0, 0, 255));

        self.display.canvas.copy(&texture, None, None).unwrap();
        // swap front buffer and back buffer
        self.display.canvas.present();
    }
}