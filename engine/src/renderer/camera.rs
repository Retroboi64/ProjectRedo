use crate::renderer::scene::Transform;

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(transform: Transform, fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            transform,
            fov,
            aspect,
            near,
            far,
        }
    }
}
