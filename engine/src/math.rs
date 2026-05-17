#![allow(dead_code)]

use numix::types::Vec3;

pub struct Transform {
    pub postion: Vec3<f32>,
    pub roation: Vec3<f32>,
    pub scale: Vec3<f32>,
}

impl Transform {
    pub fn new(postion: Vec3<f32>, roation: Vec3<f32>, scale: Vec3<f32>) -> Self {
        Self {
            postion,
            roation,
            scale,
        }
    }

    pub fn get_postion(&mut self) -> &mut Vec3<f32> {
        &mut self.postion
    }

    pub fn get_roation(&mut self) -> &mut Vec3<f32> {
        &mut self.roation
    }

    pub fn get_scale(&mut self) -> &mut Vec3<f32> {
        &mut self.scale
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            postion: Vec3::new(0.0, 0.0, 0.0),
            roation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}
