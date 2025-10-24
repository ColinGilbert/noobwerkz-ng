//use glam::{Mat4, Quat, Vec3A};
use crate::callbacks::*;
use crate::camera::*;
use crate::camera_context::*;
use crate::graphics_context::*;
use crate::instance::*;
use crate::light::*;
use crate::model_node::*;
use crate::passes::{Pass, phong::*};
use crate::resource::*;
use crate::scene::*;
use crate::texture::*;
use crate::user_context::*;

use std::f32::consts::PI;

use std::sync::Arc;

use winit::{
    event::{MouseButton, MouseScrollDelta},
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};

const NUM_INSTANCES_PER_ROW: u32 = 10;

pub struct State {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub gfx_ctx: GraphicsContext,
    pub light_ctx: LightContext,
    pub cam_ctx: CameraContext,
    pub phong: Phong,
    #[allow(dead_code)]
    pub is_surface_configured: bool,
    // NEW!
    pub mouse_pressed: bool,
}


pub type UserSetupCallback = fn(&str);


pub fn initialize_callbacks(callback: UserSetupCallback) {
   callback("Hello from the library");
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let mut gfx_ctx = GraphicsContext::new(&window, &surface, &instance).await;

        let mut u = USER_CONTEXT.lock().unwrap();
        u.models.push(
            load_model_from_serialized(
                "res".to_owned(),
                "model.bin".to_owned(),
                &mut gfx_ctx.device,
                &mut gfx_ctx.queue,
                &gfx_ctx.texture_bind_group_layout,
            )
            .await
            .unwrap(),
        );

        let projection = Projection::new(
            gfx_ctx.config.height,
            gfx_ctx.config.width,
            degrees_to_radians(45.0),
            0.0001,
            1000.0,
        );

        let mut s = Scene::new();
        let c = Camera::new(
            &glam::Vec3 {
                x: 10.0,
                y: 10.0,
                z: 10.0,
            },
            &glam::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &glam::Vec3::Y,
            0.1,
            0.1,
            projection,
        );
        const SPACE_BETWEEN: f32 = 1.0;
        s.model_nodes.push(ModelNode::new(
            u.models.len() - 1,
            (0..NUM_INSTANCES_PER_ROW)
                .flat_map(|z| {
                    (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                        let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);
                        let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);

                        let position: glam::Vec3A = glam::Vec3 { x, y: 0.0, z }.into();

                        let rotation = if position == glam::Vec3A::ZERO {
                            glam::Quat::from_axis_angle(glam::Vec3::Z, 0.0)
                        } else {
                            let pos: glam::Vec3 = position.into();
                            glam::Quat::from_axis_angle(pos.normalize(), 45.0)
                        };
                        let scale: glam::Vec3A = glam::Vec3 {
                            x: 10.0,
                            y: 10.0,
                            z: 10.0,
                        }
                        .into();
                        Instance {
                            position,
                            rotation,
                            scale,
                        }
                    })
                })
                .collect::<Vec<_>>(),
        ));

        let cam_ctx = CameraContext::new(&gfx_ctx.device, &c);
        s.cameras.push(c);
        u.scenes.push(s);

        let mut lights = Vec::<LightUniform>::new();

        lights.push(LightUniform {
            position: [2.0, 2.0, 2.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        });

        let light_ctx = LightContext::new(&gfx_ctx.device, lights);

        let phong = Phong::new(
            &gfx_ctx.device,
            &light_ctx.light_buffer,
            &cam_ctx.buffer,
            &gfx_ctx.texture_bind_group_layout,
            &cam_ctx.bind_group_layout,
            &light_ctx.light_bind_group_layout,
            &gfx_ctx.config,
        );
        

        Ok(Self {
            window,
            surface,
            gfx_ctx,
            light_ctx,
            cam_ctx,
            phong,
            is_surface_configured: false,
            mouse_pressed: false,
        })
    }

    #[allow(unused)]
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        println!("resizing");
        let mut u = USER_CONTEXT.lock().unwrap();
        let scene_idx = u.active_scene;
        let s = &mut u.scenes[scene_idx];
        let cam_idx = s.active_camera;
        let c = &mut s.cameras[cam_idx];
        if width > 0 && height > 0 {
            c.projection.resize(height, width);
            self.is_surface_configured = true;
            self.gfx_ctx.config.width = width;
            self.gfx_ctx.config.height = height;
            self.surface
                .configure(&self.gfx_ctx.device, &self.gfx_ctx.config);
            self.gfx_ctx.depth_texture = Texture::create_depth_texture(
                &self.gfx_ctx.device,
                &self.gfx_ctx.config,
                "depth_texture",
            );
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        // if !self.cam_ctx.controller.handle_key(key) {
        let mut u = USER_CONTEXT.lock().unwrap();
        let scene_idx = u.active_scene;
        let s = &mut u.scenes[scene_idx];
        let cam_idx = s.active_camera;
        let c = &mut s.cameras[cam_idx];
        match (key, pressed) {
            (KeyCode::ArrowUp, true) => {
                c.move_up();
                //true
            }
            (KeyCode::ArrowDown, true) => {
                c.move_down();
                // true
            }
            (KeyCode::ArrowLeft, true) => {
                c.move_left();
                // true
            }
            (KeyCode::ArrowRight, true) => {
                c.move_right();
                // true
            }
            (KeyCode::Escape, true) => {
                event_loop.exit();
            } // true},
            _ => {} //false }
        }
        // }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.mouse_pressed = pressed,
            _ => {}
        }
    }

    pub fn handle_mouse_scroll(&mut self, delta: &MouseScrollDelta) {
        let mut u = USER_CONTEXT.lock().unwrap();
        let scene_idx = u.active_scene;
        let s = &mut u.scenes[scene_idx];
        let cam_idx = s.active_camera;
        let c = &mut s.cameras[cam_idx];
        match delta {
            //     // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, s) => {
                if *s < 0.0 {
                    c.move_backward();
                } else {
                    c.move_forward();
                }
            }
            MouseScrollDelta::PixelDelta(position) => {
                if position.y < 0.0 {
                    c.move_backward();
                } else {
                    c.move_forward();
                }
            }
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        let mut u = USER_CONTEXT.lock().unwrap();
        let scene_idx = u.active_scene;
        let s = &mut u.scenes[scene_idx];
        let cam_idx = s.active_camera;
        s.cameras[s.active_camera].update();
        self.cam_ctx
            .uniform
            .update_view_proj(&s.cameras[cam_idx], &s.cameras[cam_idx].projection);
        self.gfx_ctx.queue.write_buffer(
            &self.cam_ctx.buffer,
            0,
            bytemuck::cast_slice(&[self.cam_ctx.uniform]),
        );

        // Update the light
        let old_position: glam::Vec3 = self.light_ctx.light_uniforms[0].position.into();
        self.light_ctx.light_uniforms[0].position =
            (glam::Quat::from_axis_angle(glam::Vec3::Y, PI * dt.as_secs_f32()) * old_position)
                .into();
        self.gfx_ctx.queue.write_buffer(
            &self.light_ctx.light_buffer,
            0,
            bytemuck::cast_slice(&[self.light_ctx.light_uniforms[0]]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let u = USER_CONTEXT.lock().unwrap();
        let s = &u.scenes[u.active_scene];

        self.phong.draw(
            &self.gfx_ctx.device,
            &self.gfx_ctx.queue,
            &u.models,
            &s.model_nodes,
            &self.gfx_ctx.depth_texture.view,
            &view,
        );

        output.present();

        Ok(())
    }
}
