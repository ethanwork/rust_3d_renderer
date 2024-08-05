use crate::vector::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub fov_angle: f32,
}