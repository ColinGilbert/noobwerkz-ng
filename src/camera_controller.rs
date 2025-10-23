use crate::camera::*;
use instant::{Duration, Instant};
use winit::event::*;
use winit::keyboard::KeyCode;

pub struct CameraController {
    pub last_frame: Instant,
    pub camera: Camera,
    pub projection: Projection,
    //pub movement: CameraMovement,
}

impl CameraController {
    pub fn new(last_frame: Instant, width: u32, height: u32) -> Self {
        let camera = Camera::new(
            &glam::Vec3::from_slice(&[10.0, 10.0, 10.0]),
            &glam::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &glam::Vec3::Y,
            0.5,
            degrees_to_radians(15.0),
        );
        let projection = Projection::new(
            height,
            width,
            degrees_to_radians(45.0),
            0.1,
            1000.0,
        );

        Self {
            last_frame,
            camera,
            projection,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::ArrowUp => {
                self.camera.move_up();
                true
            }
            KeyCode::ArrowDown => {
                self.camera.move_down();
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
            KeyCode::KeyW => true,
            KeyCode::KeyS => true,
            KeyCode::KeyA => true,
            KeyCode::KeyD => true,
            KeyCode::KeyQ => true,
            KeyCode::KeyE => true,
            _ => false,
        }
    }

    pub fn handle_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.camera.change_yaw(degrees_to_radians(mouse_dx as f32)); //rotate_horizontal = mouse_dx ;
        self.camera
            .change_pitch(degrees_to_radians(mouse_dy as f32)); //)rotate_vertical = mouse_dy ;
    }

    pub fn handle_scroll(&mut self, delta: &MouseScrollDelta) {
        match delta {
            //     // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, s) => {
                if *s < 0.0 {
                    self.camera.move_backward();
                } else {
                    self.camera.move_forward();
                }
            }
            MouseScrollDelta::PixelDelta(position) => {
                if position.y < 0.0 {
                    self.camera.move_backward();
                } else {
                    self.camera.move_forward();
                }
            }
        }
    }

    pub fn update_camera(&mut self, dt: Duration) {
        let _dt = dt.as_secs_f64();
        self.camera.update(); //dt as f32, &self.movement);
        //self.movement = CameraMovement::new();
    }
}
