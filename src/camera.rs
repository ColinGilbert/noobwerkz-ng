use std::f32::consts::PI;
use std::ops::*;

const PI_2: f32 = PI * 2.0;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub struct Camera {
    pub up: glam::Vec3,
    pub eye: glam::Vec3,
    pub eye_delta: glam::Vec3,
    pub look_at: glam::Vec3,
    pub direction: glam::Vec3,
    pub world_up: glam::Vec3, // This is used to reset the camera.
    pub yaw: f32,
    pub pitch: f32,
    pub max_yaw_rate: f32,
    pub max_pitch_rate: f32,
    pub moving: bool,
    pub speed: f32,
    pub heading_speed: f32,
    pub projection: Projection,
}

impl Camera {
    pub fn new(
        starting_eye_pos: &glam::Vec3,
        look_at: &glam::Vec3,
        world_up: &glam::Vec3,
        speed: f32,
        heading_speed: f32,
        projection: Projection,
    ) -> Self {
        Self {
            up: *world_up,
            eye: *starting_eye_pos,
            eye_delta: glam::Vec3::ZERO,
            look_at: *look_at,
            direction: (*look_at - *starting_eye_pos).normalize(),
            world_up: *world_up, // This is used to reset te camera.
            //scale: scale,
            yaw: 0.0,
            pitch: 0.0,
            max_yaw_rate: degrees_to_radians(5.0),
            max_pitch_rate: degrees_to_radians(5.0),
            moving: true,
            speed,
            // Should be between 0 and 1
            heading_speed,
            projection,
        }
    }

    pub fn update(&mut self) {
        // Adapted from: https://github.com/hmazhar/moderngl_camera
        self.direction = (self.look_at - self.eye).normalize();
        let axis = glam::Vec3::cross(self.direction, self.up);
        //compute quaternion for pitch based on the camera pitch angle
        let pitch_quat = glam::Quat::from_axis_angle(axis, self.pitch);
        //determine yaw quaternion from the camera up vector and the yaw angle
        let yaw_quat = glam::Quat::from_axis_angle(self.up, self.yaw);
        //add the two quaternions
        let mut temp = pitch_quat * yaw_quat;
        temp = temp.normalize();
        //update the direction from the quaternion
        self.direction = temp * self.direction;
        //add the camera delta
        self.eye += self.eye_delta;
        //set the look at to be infront of the camera
        self.look_at = self.eye + self.direction;
        //damping for smooth camera
        self.yaw *= degrees_to_radians(0.5);
        self.pitch *= degrees_to_radians(0.5);
        self.eye_delta = self.eye_delta * 0.8;
    }

    pub fn view_matrix(&self) -> glam::Mat4 {
        let view_matrix = glam::Mat4::look_at_rh(self.eye, self.look_at, self.up);
        //println!("Getting view matrix");
        view_matrix
    }

    pub fn reset(&mut self) {
        self.up = self.world_up;
    }

    pub fn change_pitch(&mut self, rads: f32) {
        let mut temp = rads;
        //Check bounds with the max pitch rate so that we aren't moving too fast
        if temp < -self.max_pitch_rate {
            temp = -self.max_pitch_rate;
        } else if temp > self.max_pitch_rate {
            temp = self.max_pitch_rate;
        }
        self.pitch = self.pitch + (temp * self.heading_speed);

        //Check bounds for the camera pitch
        if self.pitch > PI_2 {
            self.pitch -= PI_2;
        } else if self.pitch < -PI_2 {
            self.pitch += PI_2;
        }
    }

    pub fn change_yaw(&mut self, rads: f32) {
        let mut temp = rads;
        //Check bounds with the max pitch rate so that we aren't moving too fast
        if temp < -self.max_yaw_rate {
            temp = -self.max_yaw_rate;
        } else if temp > self.max_yaw_rate {
            temp = self.max_yaw_rate;
        }
        self.yaw = self.yaw + (temp * self.heading_speed);

        //Check bounds for the camera pitch
        if self.yaw > PI_2 {
            self.yaw -= PI_2;
        } else if self.yaw < -PI_2 {
            self.yaw += PI_2;
        }
    }

    pub fn move_up(&mut self) {
        self.eye_delta += self.up * self.speed;
    }

    pub fn move_down(&mut self) {
        self.eye_delta -= self.up * self.speed;
    }

    pub fn move_forward(&mut self) {
        self.eye_delta += self.direction * self.speed;
    }

    pub fn move_backward(&mut self) {
        self.eye_delta = self.eye_delta.sub(self.direction.mul(self.speed));
    }

    pub fn move_right(&mut self) {
        self.eye_delta = self
            .eye_delta
            .add(self.direction.cross(self.up).mul(self.speed));
    }

    pub fn move_left(&mut self) {
        self.eye_delta = self
            .eye_delta
            .sub(self.direction.cross(self.up).mul(self.speed));
    }
}

pub struct Projection {
    pub aspect_ratio: f32,
    pub fovy_rad: f32,
    pub znear: f32,
    pub zfar: f32,
}
pub const OPENGL_TO_WGPU_MATRIX : glam::Mat4 = glam::Mat4::from_cols(
    glam::Vec4::new(1.0, 0.0, 0.0, 0.0),
    glam::Vec4::new(0.0, 1.0, 0.0, 0.0),
    glam::Vec4::new(0.0, 0.0, 0.5, 0.0),
    glam::Vec4::new(0.0, 0.0, 0.5, 1.0),
);

 

 
impl Projection {
    pub fn new(height: u32, width: u32, fovy_rad: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect_ratio: width as f32 / height as f32,
            fovy_rad,
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, height: u32, width: u32) -> () {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        //OPENGL_TO_WGPU_MATRIX * glam::Mat4::perspective_rh(self.fovy_rad, self.aspect_ratio, self.znear, self.zfar);
        let results =
            OPENGL_TO_WGPU_MATRIX * glam::Mat4::perspective_rh(self.fovy_rad, self.aspect_ratio, self.znear, self.zfar);
        results
    }
}

// This represents the camera uniform that lives on the GPU
#[repr(C)]
// Derive the required traits for safe casting.
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_position: [f32; 4],
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0, 0.0, 0.0, 1.0],
            view_projection: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = [camera.eye.x, camera.eye.y, camera.eye.z, 1.0];
        self.view_projection = (projection.calc_matrix() * camera.view_matrix()).to_cols_array_2d();
    }
}
