use std::ops::*;

#[derive(Copy, Clone)]
pub struct Camera {
    pub eye: glam::Vec3,
    pub front: glam::Vec3,
    pub up: glam::Vec3,
    pub right: glam::Vec3,
    pub world_up: glam::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    //pub roll: f32,
    pub movement_speed: f32,
    pub movement_sensitivity: f32,
    // pub min_pitch: f32,
    // pub max_pitch: f32,
    // pub min_yaw: f32,
    // pub max_yaw: f32,
    // pub min_roll: f32,
    // pub max_roll: f32,
}

impl Camera {
    pub fn new() -> Self {
        let up = glam::Vec3::Y;

        Camera {
            eye: glam::Vec3::from_slice(&[0.0, 0.0, 0.0]),
            front: glam::Vec3::from_slice(&[0.0, 0.0, -1.0]),
            up,
            right: glam::Vec3::from_slice(&[1.0, 0.0, 0.0]),
            world_up: up,
            yaw: 0.0,
            pitch: 0.0,
            //roll: 0.0,
            movement_speed: 1.0,
            movement_sensitivity: 1.0,
            // min_pitch: -PI / 2.0,
            // max_pitch: PI / 2.0,
            // min_yaw: -PI / 2.0,
            // max_yaw: PI / 2.0,
            // min_roll: -PI / 2.0,
            // max_roll: PI / 2.0,
        }
    }

    pub fn view_matrix(self) -> glam::Mat4 {
        let result = glam::Mat4::look_at_rh(self.eye, self.eye + self.front, self.up);
        result
    }

    pub fn update(&mut self) {
        let mut front = glam::Vec3::from_slice(&[0.0, 0.0, 0.0]);
        let (sin_yaw, cos_yaw) = libm::sincosf(self.yaw);
        let (sin_pitch, cos_pitch) = libm::sincosf(self.pitch);

        front.x = cos_yaw * cos_pitch;
        front.y = sin_pitch;
        front.z = sin_yaw * cos_pitch;

        self.front = front.normalize();

        //self.front = front;
        self.right = (front.cross(self.world_up)).normalize();
        self.up = (self.right.cross(front)).normalize();
    }

    pub fn move_forward(&mut self) {
        self.eye = self.eye.add(self.front.mul(self.movement_speed ));
    }

    pub fn move_backward(&mut self) {
        self.eye = self.eye.add(self.front.mul(-self.movement_speed));
    }

    pub fn move_left(&mut self) {
        self.eye = self.eye.add(self.right.mul(-self.movement_speed));
    }

    pub fn move_right(&mut self) {
        self.eye = self.eye.add(self.right.mul(self.movement_speed));
    }
}


pub struct CameraMovement {
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
    pub move_in: bool,
    pub move_out: bool,
    pub swing_left: bool,
    pub swing_right: bool,
    pub swing_over: bool,
    pub swing_under: bool,
    pub roll_clockwise: bool,
    pub roll_counterclockwise: bool,
}

impl CameraMovement {
    pub fn new() -> Self {
        Self {
            move_left: false,
            move_right: false,
            move_up: false,
            move_down: false,
            move_in: false,
            move_out: false,
            swing_left: false,
            swing_right: false,
            swing_over: false,
            swing_under: false,
            roll_clockwise: false,
            roll_counterclockwise: false,
        }
    }
}
