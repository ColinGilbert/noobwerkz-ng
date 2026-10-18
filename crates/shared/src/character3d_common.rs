use std::collections::VecDeque;

pub struct Character3DCommon {
    pub alive: bool,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    pub base_speed: f32,
    pub speed_modifier: f32,
    pub path: VecDeque<glam::Vec3>,
}

impl Character3DCommon {
    pub fn new(position: glam::Vec3, orientation: glam::Quat, base_speed: f32) -> Self {
        return Self {
            alive: true,
            position,
            orientation,
            base_speed,
            speed_modifier: 1.0,
            path: VecDeque::new(),
        }
    }
}