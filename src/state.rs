//use glam::{Mat4, Quat, Vec3A};

use crate::camera::*;
use crate::graphics::*;
use crate::instance::*;
use crate::model::*;
use crate::resource::*;
use crate::texture::*;
use std::f32::consts::PI;
use std::iter;
use std::sync::Arc;
use wgpu::util::DeviceExt;
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
    pub ctx: GraphicsContext,
    pub render_pipeline: wgpu::RenderPipeline,
    pub obj_model: Model,
    pub camera: Camera,                      // UPDATED!
    pub projection: Projection,              // NEW!
    pub camera_controller: CameraController, // UPDATED!
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub instances: Vec<Instance>,
    #[allow(dead_code)]
    pub instance_buffer: wgpu::Buffer,
    pub depth_texture: Texture,
    pub is_surface_configured: bool,
    #[allow(unused)]
    pub light_uniform: LightUniform,
    pub light_buffer: wgpu::Buffer,
    pub light_bind_group: wgpu::BindGroup,
    pub light_render_pipeline: wgpu::RenderPipeline,
    #[allow(dead_code)]
    pub debug_material: Material,
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

        let mut ctx = GraphicsContext::new(&window, &surface, &instance).await;

        let texture_bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        // diffuse map
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                        // normal map
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        // UPDATED!
        let camera = Camera::new(
            glam::Vec3 {
                x: 0.0,
                y: 10.0,
                z: 10.0,
            },
            degrees_to_radians(45.0),
            degrees_to_radians(20.0),
        );
        let projection = Projection::new(
            ctx.config.width,
            ctx.config.height,
            degrees_to_radians(45.0),
            0.1,
            1000.0,
        );
        let camera_controller = CameraController::new(4.0, 0.4);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        const SPACE_BETWEEN: f32 = 1.0;
        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 10.0);

                    let position: glam::Vec3A = glam::Vec3 { x, y: 0.0, z }.into();

                    let rotation = if position == glam::Vec3A::ZERO {
                        glam::Quat::from_axis_angle(glam::Vec3::Z, 0.0)
                    } else {
                        let pos : glam::Vec3 = position.into();
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
            .collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let camera_bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let camera_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let obj_model = load_model_from_serialized(
            "res".to_owned(),
            "model.bin".to_owned(),
            &mut ctx.device,
            &mut ctx.queue,
            &texture_bind_group_layout,
        )
        .await
        .unwrap();

        let light_uniform = LightUniform {
            position: [2.0, 2.0, 2.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        };

        let light_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let light_bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    label: None,
                });

        let light_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let depth_texture =
            Texture::create_depth_texture(&ctx.device, &ctx.config, "depth_texture");

        let render_pipeline_layout =
            ctx.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &texture_bind_group_layout,
                        &camera_bind_group_layout,
                        &light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            };
            create_render_pipeline(
                &ctx.device,
                &render_pipeline_layout,
                ctx.config.format,
                Some(Texture::DEPTH_FORMAT),
                &[ModelVertex::desc(), InstanceRaw::desc()],
                shader,
            )
        };

        let light_render_pipeline = {
            let layout = ctx
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Light Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                    push_constant_ranges: &[],
                });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
            };

            create_render_pipeline(
                &ctx.device,
                &layout,
                ctx.config.format,
                Some(Texture::DEPTH_FORMAT),
                &[ModelVertex::desc()],
                shader,
            )
        };

        let debug_material = {
            let diffuse_bytes = include_bytes!("../res/cobble-diffuse.png");
            let normal_bytes = include_bytes!("../res/cobble-normal.png");

            let diffuse_texture = Texture::from_bytes(
                &ctx.device,
                &ctx.queue,
                diffuse_bytes,
                "res/alt-diffuse.png",
                false,
            )
            .unwrap();
            let normal_texture = Texture::from_bytes(
                &ctx.device,
                &ctx.queue,
                normal_bytes,
                "res/alt-normal.png",
                true,
            )
            .unwrap();

            Material::new(
                &ctx.device,
                "alt-material",
                diffuse_texture,
                normal_texture,
                &texture_bind_group_layout,
            )
        };

        Ok(Self {
            window,
            surface,
            ctx,
            render_pipeline,
            obj_model,
            camera,
            projection,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            instances,
            instance_buffer,
            depth_texture,
            is_surface_configured: false,
            light_uniform,
            light_buffer,
            light_bind_group,
            light_render_pipeline,
            #[allow(dead_code)]
            debug_material,
            // NEW!
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
            self.projection.resize(width, height);
            self.is_surface_configured = true;
            self.ctx.config.width = width;
            self.ctx.config.height = height;
            self.surface.configure(&self.ctx.device, &self.ctx.config);
            self.depth_texture =
                Texture::create_depth_texture(&self.ctx.device, &self.ctx.config, "depth_texture");
        }
    }

    // UPDATED!
    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        if !self.camera_controller.handle_key(key, pressed) {
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
        self.camera_controller.handle_scroll(delta);
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        // UPDATED!
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform
            .update_view_proj(&self.camera, &self.projection);
        self.ctx.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // Update the light
        let old_position: glam::Vec3 = self.light_uniform.position.into();
        self.light_uniform.position =
            (glam::Quat::from_axis_angle(glam::Vec3::Y, PI * dt.as_secs_f32()) * old_position)
                .into();
        self.ctx.queue.write_buffer(
            &self.light_buffer,
            0,
            bytemuck::cast_slice(&[self.light_uniform]),
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

        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_pipeline(&self.light_render_pipeline);
            render_pass.draw_light_model(
                &self.obj_model,
                &self.camera_bind_group,
                &self.light_bind_group,
            );

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw_model_instanced(
                &self.obj_model,
                0..self.instances.len() as u32,
                &self.camera_bind_group,
                &self.light_bind_group,
            );
        }
        self.ctx.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
