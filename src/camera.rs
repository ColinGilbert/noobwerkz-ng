use std::ops::*;

const CAMERA_WORLD_FORWARD: glam::Vec3 = glam::Vec3::Z;
const CAMERA_WORLD_UP: glam::Vec3 = glam::Vec3::Y;
const CAMERA_WORLD_RIGHT: glam::Vec3 = glam::Vec3::X;

const CAMERA_MODE_DISABLE_ROLL: u32 = 1; // Disables the roll axis
const CAMERA_MODE_MOVE_IN_WORLDPLANE: u32 = 1 << 2; // Projects movement onto world plane
const CAMERA_MODE_CLAMP_PITCH_ANGLE: u32 = 1 << 3; // Limits the pitch angle. Typically used in first/third person to prevent overrotation  (i.e. somersaults).
const CAMERA_MODE_CLAMP_YAW_ANGLE: u32 = 1 << 4; // Limits the yaw angle.
const CAMERA_MODE_CLAMP_ROLL_ANGLE: u32 = 1 << 5; // Limits the roll angle.
const CAMERA_MODE_FREE: u32 = 0; // Free float camera mode (no restrictions applied)
const CAMERA_MODE_FIRST_PERSON: u32 = CAMERA_MODE_FREE
    | CAMERA_MODE_DISABLE_ROLL
    | CAMERA_MODE_MOVE_IN_WORLDPLANE
    | CAMERA_MODE_CLAMP_PITCH_ANGLE; // Note: Set camera.minPitch = -pi/2 and camera.maxPitch = pi/2
const CAMERA_MODE_THIRD_PERSON: u32 = CAMERA_MODE_FIRST_PERSON; //  Note: Set camera.minPitch = -pi/2 and camera.maxPitch = pi/2. Note: Use a target_distance > 0
const CAMERA_MODE_ORBITAL: u32 =
    CAMERA_MODE_FREE | CAMERA_MODE_DISABLE_ROLL | CAMERA_MODE_CLAMP_PITCH_ANGLE; // Note: Set camera.minPitch = -pi/2 and camera.maxPitch = pi/2

fn mul(q: &glam::Quat, v: &glam::Vec3) -> glam::Vec3 {
    let tmp0 = q.inverse();
    let qv = glam::Quat::from_slice(&[v.x, v.y, v.z, 0.0]);
    let tmp1 = tmp0.mul(qv);
    let result = tmp1.mul(q);
    glam::Vec3::from_slice(&[result.x, result.y, result.z])
}

pub struct Camera {
    pub target_position: glam::Vec3,
    pub target_distance: f32,
    pub orientation: glam::Quat,
    pub movement_accumulator: glam::Vec3,
    pub rotation_accumulator: glam::Vec3,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub min_yaw: f32,
    pub max_yaw: f32,
    pub min_roll: f32,
    pub max_roll: f32,
}

impl Camera {
    pub fn new() -> Self {
        let view_mat = glam::Mat4::look_at_rh(eye, target, glam::Vec3::Y);
        Camera {
            target_position: glam::Vec3::ZERO,
            target_distance: 0.0,
            //previous_seconds: 0.0,
            orientation: glam::Quat::from_slice(&[0.0, 0.0, 0.0, 0.0]),
            movement_accumulator: glam::Vec3::from_slice(&[0.0, 0.0, 0.0]),
            rotation_accumulator: glam::Vec3::from_slice(&[0.0, 0.0, 0.0]),
            min_pitch: 0.0,
            max_pitch: 0.0,
            min_yaw: 0.0,
            max_yaw: 0.0,
            min_roll: 0.0,
            max_roll: 0.0,
        }
    }

    pub fn forward(self) -> glam::Vec3 {
        let results = mul(&self.orientation.inverse(), &CAMERA_WORLD_FORWARD);
        results
    }

    pub fn up(self) -> glam::Vec3 {
        let results = mul(&self.orientation.inverse(), &CAMERA_WORLD_UP);
        results
    }

    pub fn right(self) -> glam::Vec3 {
        let results = mul(&self.orientation.inverse(), &CAMERA_WORLD_RIGHT);
        results
    }

    pub fn eye(self) -> glam::Vec3 {
        let target_dist = self.target_distance;
        let results = self.target_position.add(self.forward().mul(-target_dist));
        results
    }

    pub fn move_cam(mut self, offset: &glam::Vec3) {
        self.movement_accumulator = self.movement_accumulator.add(offset);
    }

    pub fn rotate_cam(mut self, angles: &glam::Vec3) {
        self.rotation_accumulator= self.rotation_accumulator.add(angles);
    }
    
    pub fn look_at(self, forward: &glam::Vec3, up: &glam::Vec3) {}
    pub fn view_matrix(self) -> glam::Mat4 {
        glam::Mat4::IDENTITY
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
