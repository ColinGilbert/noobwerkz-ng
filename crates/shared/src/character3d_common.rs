use std::collections::VecDeque;

pub struct Character3DCommon {
    pub alive: bool,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    pub top_speed: f32,
    pub current_speed: f32,
    pub path: VecDeque<glam::Vec3>,
}

impl Character3DCommon {
    pub fn new(position: glam::Vec3, orientation: glam::Quat, top_speed: f32) -> Self {
        return Self {
            alive: true,
            position,
            orientation,
            top_speed,
            current_speed: 0.0,
            path: VecDeque::new(),
        }
    }
}