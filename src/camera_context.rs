use crate::camera::*;
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
        let camera = Camera::new();
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
        match key {
            KeyCode::ArrowUp => {
                self.camera.move_forward();
                true
            }
            KeyCode::ArrowDown => {
                self.camera.move_backward();
                true
            }
            KeyCode::ArrowLeft => {
                self.camera.move_left();
                true
            }
            KeyCode::ArrowRight => {
                self.camera.move_right();
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
                self.movement.swing_left = true;
                true
            }
            KeyCode::KeyD => {
                self.movement.swing_right = true;
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
        self.camera.update(); //dt as f32, &self.movement);
        self.movement = CameraMovement::new();
    }
}
