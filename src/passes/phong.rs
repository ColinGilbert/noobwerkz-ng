use crate::model_node::ModelNode;
use crate::graphics_context::create_render_pipeline;
use crate::instance::*;
use crate::normal_mapped_model::*;
use crate::passes::Pass;
use crate::texture::*;
use std::iter::once;
use wgpu::util::DeviceExt;

pub struct Phong {
    pub render_pipeline_layout: wgpu::PipelineLayout,
    pub light_bind_group: wgpu::BindGroup,
    pub camera_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub light_render_pipeline: wgpu::RenderPipeline,
}

impl Pass for Phong {
    fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        models: &Vec<NormalMappedModel>,
        model_nodes: &Vec<ModelNode>,
        depth_texture_view: &wgpu::TextureView,
        view: &wgpu::TextureView,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                    view: depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for (_i, m) in model_nodes.iter().enumerate() {
                let instance_data = m.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

                let instance_buffer =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&instance_data),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                render_pass.set_pipeline(&self.light_render_pipeline);
                render_pass.draw_light_model(
                    &models[m.model_idx],
                    &self.camera_bind_group,
                    &self.light_bind_group,
                );

                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.draw_model_instanced(
                    &models[m.model_idx],
                    0..m.instances.len() as u32,
                    &self.camera_bind_group,
                    &self.light_bind_group,
                );
            }
        }
        queue.submit(once(encoder.finish()));
    }
}

impl Phong {
    pub fn new(
        device: &wgpu::Device,
        light_buffer: &wgpu::Buffer,
        camera_buffer: &wgpu::Buffer,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        light_bind_group_layout: &wgpu::BindGroupLayout,
        config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
                &device,
                &render_pipeline_layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[NormalMappedModelVertex::desc(), InstanceRaw::desc()],
                shader,
            )
        };

        let light_render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                push_constant_ranges: &[],
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
            };

            create_render_pipeline(
                &device,
                &layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[NormalMappedModelVertex::desc()],
                shader,
            )
        };

        Self {
            render_pipeline_layout,
            light_bind_group,
            camera_bind_group,
            render_pipeline,
            light_render_pipeline,
        }
    }
}
