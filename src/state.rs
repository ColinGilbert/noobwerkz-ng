use crate::callbacks::*;
use crate::camera::*;
use crate::egui_renderer::EguiRenderer;
use crate::graphics::*;
use crate::light::*;
use crate::passes::{Pass, forward_renderer::*};
use crate::texture::*;
use crate::user_context::*;
use egui_wgpu::ScreenDescriptor;

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
    pub egui_renderer: EguiRenderer,
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

        let egui_renderer =
            EguiRenderer::new(&gfx_ctx.device, gfx_ctx.surface_format, &window.clone());

        egui_renderer.context().style_mut(|style| {
            // Disable shadows for popups, context menus, and combo boxes
            style.visuals.popup_shadow = egui::Shadow::NONE;

            // Disable shadows for windows
            style.visuals.window_shadow = egui::Shadow::NONE;

            // Optional: Also set the a faint, "hover" and "open" shadows to none if needed
            style.visuals.faint_bg_color = egui::Color32::TRANSPARENT;
            style.visuals.extreme_bg_color = egui::Color32::TRANSPARENT;
        });

        Ok(Self {
            window,
            surface,
            gfx_ctx,
            light_ctx,
            user_ctx,
            cam_ctx,
            forward_renderer,
            egui_renderer,
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
        if !self.egui_renderer.state.egui_ctx().wants_pointer_input() {
            match button {
                MouseButton::Left => self.mouse_pressed = pressed,
                _ => {}
            }
        }
    }

    pub fn handle_mouse_motion(&mut self, dx: f64, dy: f64) {
        if self.mouse_pressed {
            let u = &mut self.user_ctx;
            let scene_idx = u.active_scene;
            let s = &mut u.scenes[scene_idx];
            let cam_idx = s.active_camera;
            let c = &mut s.cameras[cam_idx];

            c.change_yaw(degrees_to_radians(dx as f32)); //rotate_horizontal = mouse_dx ;
            c.change_pitch(degrees_to_radians(dy as f32)); //)rotate_vertical = mouse_dy ;
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

        self.forward_renderer.draw(
            &self.gfx_ctx.device,
            &self.gfx_ctx.queue,
            &u.asset_mgr.models,
            &u.asset_mgr.skinned_models,
            &s.model_nodes,
            &s.skinned_model_nodes,
            &self.gfx_ctx.depth_texture.view,
            &view,
        );

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.gfx_ctx.config.width, self.gfx_ctx.config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };

        let mut encoder = self
            .gfx_ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.egui_renderer.begin_frame(&self.window);

        if let Some(cb) = *USER_GUI_CALLBACK.lock().unwrap() {
            cb(&mut self.egui_renderer, &mut self.user_ctx);
        }

        self.egui_renderer.end_frame_and_draw(
            &self.gfx_ctx.device,
            &self.gfx_ctx.queue,
            &mut encoder,
            &self.window,
            &view,
            screen_descriptor,
        );

        self.gfx_ctx.queue.submit(Some(encoder.finish()));

        output.present();

        Ok(())
    }
}
