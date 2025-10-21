//use glam::{Mat4, Quat, Vec3A};

use crate::camera::*;
use crate::graphics_context::*;
use crate::instance::*;
use crate::light::*;
use crate::model_node::*;
use crate::passes::{Pass, phong::*};
use crate::resource::*;
use crate::texture::*;

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
    pub model_nodes: Vec<ModelNode>,
    #[allow(dead_code)]
    pub is_surface_configured: bool,
    // NEW!
    pub mouse_pressed: bool,
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

        let cam_ctx = CameraContext::new(&gfx_ctx.device, &gfx_ctx.config);
        let mut model_nodes = Vec::<ModelNode>::new();

        const SPACE_BETWEEN: f32 = 1.0;
        model_nodes.push(ModelNode::new(
            load_model_from_serialized(
                "res".to_owned(),
                "model.bin".to_owned(),
                &mut gfx_ctx.device,
                &mut gfx_ctx.queue,
                &gfx_ctx.texture_bind_group_layout,
            )
            .await
            .unwrap(),
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
            model_nodes,
            is_surface_configured: false,
            mouse_pressed: false,
        })
    }

    #[allow(unused)]
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // UPDATED!
        if width > 0 && height > 0 {
            self.cam_ctx.projection.resize(width, height);
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

    // UPDATED!
    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        if !self.cam_ctx.controller.handle_key(key) {
            match (key, pressed) {
                (KeyCode::Escape, true) => event_loop.exit(),
                _ => {}
            }
        }
    }

    // NEW!
    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.mouse_pressed = pressed,
            _ => {}
        }
    }

    // NEW!
    pub fn handle_mouse_scroll(&mut self, delta: &MouseScrollDelta) {
        self.cam_ctx.controller.handle_scroll(delta);
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        // UPDATED!
        self.cam_ctx.controller.update_camera(dt);
        self.cam_ctx.uniform
            .update_view_proj(&self.cam_ctx.controller.camera, &self.cam_ctx.projection);
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

        self.phong.draw(
            &self.gfx_ctx.device,
            &self.gfx_ctx.queue,
            &self.model_nodes,
            &self.gfx_ctx.depth_texture.view,
            &view,
        );

        output.present();

        Ok(())
    }
}
