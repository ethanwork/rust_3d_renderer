use sdl2::{
    pixels::{Color},
    render::{Canvas, Texture},
    video::Window, Sdl,
};

pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;

pub enum RenderMode {
    WireframeWithDot,
    Wireframe,
    FilledTriangles,
    FilledTrianglesAndWireframe,
    Textured,
    TexturedAndWireframe
}

pub struct Display {
    pub canvas: Canvas<Window>,
    color_buffer: Box<[u8]>,
}

impl Display {
    pub fn new(sdl_context: &mut Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("3D Rendering", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();
        let color_buffer = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4].into_boxed_slice();
        
        Display {
            canvas,
            color_buffer,
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color, ) {
        // if the x and y are not within bounds of the screen, return
        if !(x < SCREEN_WIDTH && y < SCREEN_HEIGHT) {
            return;
        }
        // multiply by 4 because each pixel is split into 4 u8 array indexes and
        // not one u32 array index
        let x = x * 4;
        let y = y * 4;
        // not sure why, but I needed to swap the b and r values
        // to get the correct color
        self.color_buffer[(SCREEN_WIDTH * y) + x] = color.b;
        self.color_buffer[(SCREEN_WIDTH * y) + x + 1] = color.g;
        self.color_buffer[(SCREEN_WIDTH * y) + x + 2] = color.r;
        self.color_buffer[(SCREEN_WIDTH * y) + x + 3] = color.a;
    }    

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let delta_x = x1 as i32 - x0 as i32;
        let delta_y = y1 as i32 - y0 as i32;

        let longest_side_length = 
            if delta_x.abs() >= delta_y.abs() { delta_x.abs() } 
            else { delta_y.abs() };
    
        let x_inc = delta_x as f32 / longest_side_length as f32;
        let y_inc = delta_y as f32 / longest_side_length as f32;

        let mut current_x = x0 as f32;
        let mut current_y = y0 as f32;

        for _i in 0..=longest_side_length {
            self.draw_pixel(current_x.round() as usize, current_y.round() as usize, color);
            current_x += x_inc;
            current_y += y_inc;
        }
    }

    pub fn draw_triangle(
        &mut self, 
        x0: usize, y0: usize,
        x1: usize, y1: usize,
        x2: usize, y2: usize,
        color: Color) {
        self.draw_line(x0, y0, x1, y1, color);
        self.draw_line(x1, y1, x2, y2, color);
        self.draw_line(x2, y2, x0, y0, color);
    }
    
    pub fn clear_color_buffer(&mut self, color: Color) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.draw_pixel(x, y, color);
            }
        }
    }
    
    pub fn draw_grid(&mut self) {
        for y in (0..SCREEN_HEIGHT).step_by(10) {
            for x in (0..SCREEN_WIDTH).step_by(10) {
                self.draw_pixel(x, y, Color::RGBA(255, 255, 255, 255))
            }
        }
    }
    
    pub fn draw_rect(&mut self, x: usize, y: usize, height: usize, width: usize, color: Color) {
        for x_val in x..(x + width) {
            for y_val in y..(y + height) {
                self.draw_pixel(x_val, y_val, color);
            }
        }
    }
    
    pub fn render_color_buffer(&self, texture: &mut Texture) {
        texture
            .update(None, &*self.color_buffer, 4 * SCREEN_WIDTH)
            .unwrap();
    }
}