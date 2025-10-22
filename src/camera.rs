use std::ops::{Add, Sub};

pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub speed: f32,
    pub heading_speed: f32,
    pub view_mat: glam::Mat4,
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

impl Camera {
    pub fn new(
        speed: f32,
        heading_speed: f32,
        eye: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
    ) -> Self {
        let view_mat = glam::Mat4::look_at_rh(eye, target, glam::Vec3::Y);
        Camera {
            eye,
            target,
            //previous_seconds: 0.0,
            up,
            speed,
            heading_speed,
            view_mat,
        }
    }

    pub fn update(&mut self, delta: f32, movement: &CameraMovement) {
        let mut cam_moved = false;
        let mut cam_yaw = 0.0;
        let mut cam_pitch = 0.0;
        let mut cam_roll = 0.0;
        let forward = self.target.sub(self.eye);
        let forward_normalized = forward.normalize();
        let forward_magnitude = forward.length();
        let right = forward_normalized.cross(self.up);

        if movement.move_in && forward_magnitude >= self.speed {
            self.eye = self.eye.add(forward_normalized * self.speed);
        }
        if movement.move_out {
            self.eye = self.eye.sub(forward_normalized * self.speed);
        }

        if movement.move_left {
            self.eye = self.target.sub(forward.sub(right * self.speed));
            self.target = self.target.add(right * self.speed);
            cam_moved = true;
        }

        if movement.move_right {
            self.eye = self.target.sub(forward.add(right * self.speed));
            self.target = self.target.sub(right * self.speed);
            cam_moved = true;
        }

        if movement.move_up {
            self.eye = self.target.sub(forward.sub(self.up * self.speed));
            self.target = self.target.add(self.up * self.speed);
            cam_moved = true;
        }

        if movement.move_down {
            self.eye = self.target.sub(forward.add(self.up * self.speed));
            self.target = self.target.sub(self.up * self.speed);
            cam_moved = true;
        }

        if movement.move_in {
            self.eye = self.eye.add(forward_normalized * self.speed);
            self.target = self.target.add(forward_normalized * self.speed);
            cam_moved = true;
        }

        if movement.move_out {
            self.eye = self.eye.sub(forward_normalized * self.speed);
            self.target = self.target.sub(forward_normalized * self.speed);
            cam_moved = true;
        }

        if movement.swing_left {
            cam_yaw += 1.2;
            let (sin_yaw, cos_yaw) = libm::sincosf(cam_yaw);
            self.target = self.target.add(glam::Vec3::from_slice(&[
                sin_yaw * self.heading_speed,
                cos_yaw * self.heading_speed,
                0.0,
            ]));
        }

        if movement.swing_right {
            cam_yaw -= 1.2;
            let (sin_yaw, cos_yaw) = libm::sincosf(cam_yaw);
            self.target = self.target.add(glam::Vec3::from_slice(&[
                cos_yaw * self.heading_speed,
                sin_yaw * self.heading_speed,
                0.0,
            ]));
        }

        if movement.swing_over {
            cam_pitch += self.heading_speed;
        }

        if movement.swing_under {
            cam_pitch -= self.heading_speed;
        }

        if movement.roll_clockwise {
            cam_roll -= self.heading_speed;
        }

        if movement.roll_counterclockwise {
            cam_roll = self.heading_speed;
        }

        // // Move forward/backward and left/right
        // let (yaw_sin, yaw_cos) = libm::sincosf(camera.yaw_rad);
        // let forward = glam::Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        // let right = glam::Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        // camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        // camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        if cam_moved {
            self.view_mat = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        }
    }
}
