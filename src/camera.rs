use instant::{Duration, Instant};
use std::ops::{Add, Sub};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::keyboard::KeyCode;
// pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols(
//     glam::Vec4::from_array([1.0, 0.0, 0.0, 0.0]),
//     glam::Vec4::from_array([0.0, 1.0, 0.0, 0.0]),
//     glam::Vec4::from_array([0.0, 0.0, 0.5, 0.0]),
//     glam::Vec4::from_array([0.0, 0.0, 0.5, 1.0]),
// );

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub struct CameraContext {
    pub projection: Projection,
    pub controller: CameraController,
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl CameraContext {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let camera = Camera::new(
            1.0,
            1.0,
            glam::Vec3::from_slice(&[0.0, 10.0, 10.0]),
            glam::Vec3::ZERO,
            glam::Vec3::Y,
        );
        let projection = Projection::new(
            config.width,
            config.height,
            degrees_to_radians(45.0),
            0.1,
            1000.0,
        );

        let mut uniform = CameraUniform::new();
        uniform.update_view_proj(&camera, &projection);
        let controller = CameraController::new(Instant::now(), camera);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            projection,
            controller,
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
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

pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    pub up: glam::Vec3,
    pub speed: f32,
    pub heading_speed: f32,
    pub view_mat: glam::Mat4,
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
            self.eye = self.target.sub(forward.add(self.up * self.speed));
            self.target = self.target.sub(self.up * self.speed);
            cam_moved = true;
        }

        if movement.move_down {
            self.eye = self.target.sub(forward.sub(self.up * self.speed));
            self.target = self.target.add(self.up * self.speed);
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
        }

        if movement.swing_right {
            cam_yaw -= self.heading_speed * delta;
        }

        if movement.swing_over {
            cam_pitch += self.heading_speed * delta;
        }

        if movement.swing_under {
            cam_pitch -= self.heading_speed * delta;
        }

        if movement.roll_clockwise {
            cam_roll -= self.heading_speed * delta;
        }

        if movement.roll_counterclockwise {
            cam_roll = self.heading_speed * delta;
        }

        if cam_moved {
            self.view_mat = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        }
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
        self.view_projection = (projection.calc_matrix() * camera.view_mat).to_cols_array_2d();
    }
}
pub struct Projection {
    pub aspect_ratio: f32,
    pub fovy_rad: f32,
    pub znear: f32,
    pub zfar: f32,
}

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
            glam::Mat4::perspective_rh(self.fovy_rad, self.aspect_ratio, self.znear, self.zfar);
        results
    }
}

pub struct CameraController {
    pub last_frame: Instant,
    pub camera: Camera,
    pub movement: CameraMovement,
}

impl CameraController {
    pub fn new(last_frame: Instant, camera: Camera) -> Self {
        Self {
            last_frame,
            camera,
            movement: CameraMovement::new(),
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        // delta_time = self.last_frame.elapsed();

        match key {
            KeyCode::ArrowUp => {
                self.movement.move_in = true;
                true
            }
            KeyCode::ArrowDown => {
                self.movement.move_out = true;
                true
            }
            KeyCode::ArrowLeft => {
                self.movement.move_left = true;
                true
            }
            KeyCode::ArrowRight => {
                self.movement.move_right = true;
                true
            }
            KeyCode::KeyW => {
                self.movement.move_up = true;
                true
            }
            KeyCode::KeyS => {
                self.movement.move_down = true;
                true
            }

            KeyCode::KeyA => {
                self.movement.move_left = true;
                true
            }

            KeyCode::KeyD => {
                self.movement.move_right = true;
                true
            }

            KeyCode::KeyQ => {
                self.movement.swing_left = true;
                true
            }
            KeyCode::KeyE => {
                self.movement.swing_right = true;
                true
            }
            _ => false,
        }
    }

    pub fn handle_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        // self.rotate_horizontal = mouse_dx ;
        // self.rotate_vertical = mouse_dy ;
    }

    pub fn handle_scroll(&mut self, delta: &MouseScrollDelta) {
        // self.scroll = match delta {
        //     // I'm assuming a line is about 100 pixels
        //     MouseScrollDelta::LineDelta(_, scroll) => -scroll * 100.0, // * 0.5,
        //     MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -*scroll ,
    }

    pub fn update_camera(&mut self, dt: Duration) {
        let dt = dt.as_secs_f64();
        //self.camera.previous_seconds = selfdelta_time;
        self.camera.update(dt as f32, &self.movement);
        self.movement = CameraMovement::new();
        //self.camera.
        // // Move forward/backward and left/right
        // let (yaw_sin, yaw_cos) = libm::sincosf(camera.yaw_rad);
        // let forward = glam::Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        // let right = glam::Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        // camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        // camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // // Move in/out (aka. "zoom")
        // // Note: this isn't an actual zoom. The camera's position
        // // changes when zooming. I've added this to make it easier
        // // to get closer to an object you want to focus on.
        // let (pitch_sin, pitch_cos) = libm::sincosf(camera.pitch_rad);
        // let scrollward =
        //     glam::Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        // camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        // self.scroll = 0.0;

        // // Move up/down. Since we don't use roll, we can just
        // // modify the y coordinate directly.
        // camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;

        // // Rotate
        // camera.yaw_rad += (self.rotate_horizontal) * self.sensitivity * dt;
        // camera.pitch_rad += (-self.rotate_vertical) * self.sensitivity * dt;

        // // If process_mouse isn't called every frame, these values
        // // will not get set to zero, and the camera will rotate
        // // when moving in a non cardinal direction.
        // self.rotate_horizontal = 0.0;
        // self.rotate_vertical = 0.0;

        // // Keep the camera's angle from going too high/low.
        // if camera.pitch_rad < -SAFE_FRAC_PI_2 {
        //     camera.pitch_rad = -SAFE_FRAC_PI_2;
        // } else if camera.pitch_rad > SAFE_FRAC_PI_2 {
        //     camera.pitch_rad = SAFE_FRAC_PI_2;
        // }
    }
}
