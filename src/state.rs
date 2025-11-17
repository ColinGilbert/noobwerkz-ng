use crate::callbacks::*;
use crate::camera::*;
use crate::camera_context::*;
use crate::graphics_context::*;
use crate::light::*;
use crate::passes::{Pass, forward_renderer::*};
use crate::texture::*;
use crate::user_context::*;
use yakui::{button, row, widgets::List, CrossAxisAlignment};

use std::f64;
use std::sync::*;
use winit::{
    event::{MouseButton, MouseScrollDelta},
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};

pub struct State {
    pub window: Arc<Window>,
    pub surface: wgpu::Surface<'static>,
    pub gfx_ctx: GraphicsContext,
    pub light_ctx: LightContext,
    pub cam_ctx: CameraContext,
    pub user_ctx: UserContext,
    pub forward_renderer: ForwardRenderer,
    #[allow(dead_code)]
    pub is_surface_configured: bool,
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
        let mut lights = Vec::<LightUniform>::new();
        let mut user_ctx = UserContext::new(); // { models: :, skinned_models: (), scenes: (), active_scene: () }

        if let Some(cb) = *USER_SETUP_CALLBACK.lock().unwrap() {
            cb(&mut gfx_ctx, &mut user_ctx, &mut lights);
        }

        let u = &mut user_ctx;
        let s = &u.scenes[u.active_scene];
        let c = &s.cameras[s.active_camera];

        let cam_ctx = CameraContext::new(&gfx_ctx.device, &c);
        let light_ctx = LightContext::new(&gfx_ctx.device, lights);

        let forward_renderer = ForwardRenderer::new(
            &gfx_ctx.device,
            &light_ctx.light_buffer,
            &cam_ctx.buffer,
            &gfx_ctx.texture_bind_group_layout_3d,
            &cam_ctx.bind_group_layout,
            &light_ctx.light_bind_group_layout,
            &gfx_ctx.bone_matrices_bind_group_layout,
            &gfx_ctx.config,
        );

        u.ui.yak_renderer = Some(yakui_wgpu::YakuiWgpu::new(&gfx_ctx.device, &gfx_ctx.queue));
        u.ui.yak_window = Some(yakui_winit::YakuiWinit::new(&window.clone()));

        Ok(Self {
            window,
            surface,
            gfx_ctx,
            light_ctx,
            user_ctx,
            cam_ctx,
            forward_renderer,
            // ui,
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
        let u = &mut self.user_ctx;
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
        let u = &mut self.user_ctx;
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
            } // true,
            _ => {} //false
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, pressed: bool) {
        match button {
            MouseButton::Left => self.mouse_pressed = pressed,
            _ => {}
        }
    }

    pub fn handle_mouse_motion(&mut self, dx: f64, dy: f64) {
        if self.mouse_pressed {
            let u = &mut self.user_ctx;
            let scene_idx = u.active_scene;
            let s = &mut u.scenes[scene_idx];
            let cam_idx = s.active_camera;
            let c = &mut s.cameras[cam_idx];

            c.change_yaw(degrees_to_radians(dx as f32));
            c.change_pitch(degrees_to_radians(dy as f32));
        }
    }

    pub fn handle_mouse_scroll(&mut self, delta: &MouseScrollDelta) {
        let u = &mut self.user_ctx;
        let scene_idx = u.active_scene;
        let s = &mut u.scenes[scene_idx];
        let cam_idx = s.active_camera;
        let c = &mut s.cameras[cam_idx];

        match delta {
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
        // Here, we call our user update callback
        if let Some(cb) = *USER_UPDATE_CALLBACK.lock().unwrap() {
            cb(
                &mut self.gfx_ctx,
                &mut self.cam_ctx,
                &mut self.light_ctx,
                &mut self.user_ctx,
                dt,
            );
        }
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

        let u = &mut self.user_ctx;
        let s = &u.scenes[u.active_scene];

        // self.forward_renderer.draw(
        //     &self.gfx_ctx.device,
        //     &self.gfx_ctx.queue,
        //     &u.asset_mgr.models,
        //     &u.asset_mgr.skinned_models,
        //     &s.model_nodes,
        //     &s.skinned_model_nodes,
        //     &self.gfx_ctx.depth_texture.view,
        //     &view,
        // );
        let size = yakui::UVec2::new(self.gfx_ctx.config.width, self.gfx_ctx.config.height);
        
        let yak_surface = u.ui.surface.surface_info(
            &self.gfx_ctx.device,
            &view,
            size,
            self.gfx_ctx.surface_format,
            1,
        );

        let mut encoder = 
            self.gfx_ctx.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: yak_surface.color_attachment,
                    depth_slice: None,
                    resolve_target: yak_surface.resolve_target,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }

        let clear = encoder.finish();

        u.ui.yak.start();
        // if let Some(cb) = *USER_GUI_CALLBACK.lock().unwrap() {
        //     cb();
        // }
                row(|| {
        button("Not stretched");
        let mut col = List::column();
        col.cross_axis_alignment = CrossAxisAlignment::Stretch;
        col.show(|| {
            button("Button 1");
            button("Button 2");
            button("Button 3");
        });
    });
        u.ui.yak.finish();

        let paint_yak = u.ui.yak_renderer.as_mut().unwrap().paint(
            &mut u.ui.yak,
            &self.gfx_ctx.device,
            &self.gfx_ctx.queue,
            yak_surface,
        );

        self.gfx_ctx.queue.submit([clear, paint_yak]);

        output.present();

        Ok(())
    }
}
