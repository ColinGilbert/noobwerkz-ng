use std::cell::RefCell;
use std::rc::Rc;

use crate::graphics::*;
use crate::index_types::*;
use crate::instance3d::*;
use crate::model3d::*;
use crate::model3d_node::*;
use crate::particles3d::Particle3DContext;
use crate::particles3d::Particle3DInstance;
use crate::rendering::Pass;
use crate::scene3d::Characters3DContext;
use crate::skinned_model3d::*;
use crate::vertex::*;

// use crate::skinned_model_node::*;
use crate::texture::*;
// use simple_particles::EffectDraw;
// use crate::character::Character;
use wgpu::util::DeviceExt;

pub struct ForwardRenderer3D {
    pub render_pipeline_layout: wgpu::PipelineLayout,
    pub skinned_render_pipeline_layout: wgpu::PipelineLayout,
    pub light_bind_group: wgpu::BindGroup,
    pub camera_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub skinned_render_pipeline: wgpu::RenderPipeline,
    pub light_render_pipeline: wgpu::RenderPipeline,
    pub cpu_particles_render_pipeline_alpha: wgpu::RenderPipeline,
    pub cpu_particles_render_pipeline_additive: wgpu::RenderPipeline,
    //pub bone_matrices_bind_group_layout: wgpu::BindGroupLayout,

}

impl Pass for ForwardRenderer3D {
    fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        models: &Models3D<Model3D>,
        skinned_models: &SkinnedModels3D<SkinnedModel3D>,
        model_nodes: &Model3DNodes<Model3DNode>,
        characters_contexts: &Characters3DContexts<Characters3DContext>,
        particles_ctx: &Rc<RefCell<Particle3DContext>>,
        // particle_effects: &Vec<EffectDraw>,
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
                            a: 0.0,
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
                multiview_mask: None,
            });

            // render_pass.set_pipeline(&self.light_render_pipeline);
            // render_pass.draw_light_model(
            //     &models[0],
            //     &self.camera_bind_group,
            //     &self.light_bind_group,
            // );

            render_pass.set_pipeline(&self.render_pipeline);

            for m in model_nodes.iter() {
                let mut count = 0;
                let mut model_instance_data = Vec::<Instance3DRaw>::new();
                let model = &models[m.model_idx];
                for i in &m.instances {
                    model_instance_data.push(i.to_raw());
                    count += 1;
                }

                for mesh in model.meshes.iter() {
                    let mut mesh_instance_data = Vec::<Instance3DRaw>::new();

                    let mesh_m_mat = glam::Mat4::from_scale_rotation_translation(
                        mesh.scale,
                        mesh.rotation,
                        mesh.translation,
                    );

                    for instance in &model_instance_data {
                        let mesh_n_mat = glam::Mat3::from_quat(mesh.rotation);
                        let model_m_mat = glam::Mat4::from_cols_array_2d(&instance.model);
                        let model_n_mat = glam::Mat3::from_cols_array_2d(&instance.normal);
                        let temp = Instance3DRaw {
                            model: (model_m_mat * mesh_m_mat).to_cols_array_2d(),
                            normal: (model_n_mat * mesh_n_mat).to_cols_array_2d(),
                        };

                        mesh_instance_data.push(temp);
                    }

                    let instance_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Instance Buffer"),
                            contents: bytemuck::cast_slice(&mesh_instance_data),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

                    render_pass.draw_mesh_instanced(
                        &mesh,
                        &model.materials[mesh.material],
                        0..count,
                        &self.camera_bind_group,
                        &self.light_bind_group,
                    );
                }
            }


            render_pass.set_pipeline(&self.skinned_render_pipeline);
            
            for c in characters_contexts.iter() {
                let mut count = 0;
                let mut model_instances = Vec::<SkinnedInstance3DRaw>::new();
                let model = &skinned_models[c.skinned_model_node.skinned_model_idx];

                for i in &c.skinned_model_node.instances {
                    model_instances.push(i.to_skinned_raw());
                    count += 1;
                }
                // for (skel_index, _skel) in model.skeletons.iter().enumerate() {
                for mesh in model.meshes.iter() {
                    let mut mesh_instances = Vec::<SkinnedInstance3DRaw>::new();

                    let mesh_mat = glam::Mat4::from_scale_rotation_translation(
                        mesh.scale,
                        mesh.rotation,
                        mesh.translation,
                    );

                    for instance_raw in &model_instances {
                        let instance_model_mat =
                            glam::Mat4::from_cols_array_2d(&instance_raw.model);

                        let temp = SkinnedInstance3DRaw {
                            model: (instance_model_mat * mesh_mat).to_cols_array_2d(),
                        };

                        mesh_instances.push(temp);
                    }

                    let instance_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Instance Buffer"),
                            contents: bytemuck::cast_slice(&mesh_instances),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                    render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

                    render_pass.draw_skinned_mesh_instanced(
                        mesh,
                        &model.materials[mesh.material],
                        0..count,
                        &self.camera_bind_group,
                        &self.light_bind_group,
                        &c.skinned_model_node.bind_group,
                    );
                }
            // }
            }

            let borrowed = particles_ctx.borrow();
            let Some(fallback) = borrowed.textures.get("") else {
                println!("Could not get fallback texture. Aborting particle effect draw...");
                return;
            };
            // println!(
            //     "ForwardRenderer: Particles batches empty? {}",
            //     borrowed.batches.is_empty()
            // );
            //render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, borrowed.instance_buffer.slice(..));
            for batch in &borrowed.batches {
                let texture = borrowed.textures.get(&batch.texture).unwrap_or(fallback);
                let pipeline = if batch.additive {
                    // println!("Rendering additive");
                    &self.cpu_particles_render_pipeline_additive
                } else {
                    // println!("Rendering alpha");
                    &self.cpu_particles_render_pipeline_alpha
                };
                render_pass.set_pipeline(pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                if texture.bind_group.is_some() {
                    render_pass.set_bind_group(1, &particles_ctx.borrow().depth_bind_group, &[]);
                    render_pass.set_bind_group(2, texture.bind_group.as_ref().unwrap(), &[]);
                } else {
                    println!("Particles texture bind group is None :(")
                }
                render_pass.draw(0..6, batch.start..batch.start + batch.count);
            }
        }

        queue.submit([encoder.finish()]);
    }
}

const ADDITIVE_BLEND: wgpu::BlendState = wgpu::BlendState {
    color: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::SrcAlpha,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    },
    alpha: wgpu::BlendComponent {
        src_factor: wgpu::BlendFactor::One,
        dst_factor: wgpu::BlendFactor::One,
        operation: wgpu::BlendOperation::Add,
    },
};

impl ForwardRenderer3D {
    pub fn new(
        device: &wgpu::Device,
        light_buffer: &wgpu::Buffer,
        camera_buffer: &wgpu::Buffer,
        // particles_instance_buffer: wgpu::Buffer,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        light_bind_group_layout: &wgpu::BindGroupLayout,
        bone_matrices_bind_group_layout: &wgpu::BindGroupLayout,
        depth_bind_group_layout: &wgpu::BindGroupLayout,
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
                    Some(texture_bind_group_layout),
                    Some(camera_bind_group_layout),
                    Some(light_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let skinned_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Skinned Pipeline Layout"),
                bind_group_layouts: &[
                    Some(texture_bind_group_layout),
                    Some(camera_bind_group_layout),
                    Some(light_bind_group_layout),
                    Some(bone_matrices_bind_group_layout),
                ],
                immediate_size: 0,
            });

        let render_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("forward_shader3d.wgsl").into()),
            };

            create_indexed_mesh_render_pipeline(
                &device,
                &render_pipeline_layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[Some(Model3DVertex::desc()), Some(Instance3DRaw::desc())],
                shader,
            )
        };

        let skinned_render_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Skinned Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("forward_skinned3d.wgsl").into()),
            };
            create_indexed_mesh_render_pipeline(
                &device,
                &skinned_render_pipeline_layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[
                    Some(SkinnedModelVertex3D::desc()),
                    Some(SkinnedInstance3DRaw::desc()),
                ],
                shader,
            )
        };

        let light_render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[
                    Some(camera_bind_group_layout),
                    Some(light_bind_group_layout),
                ],
                immediate_size: 0,
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("forward_light3d.wgsl").into()),
            };

            create_indexed_mesh_render_pipeline(
                &device,
                &layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[Some(Model3DVertex::desc())],
                shader,
            )
        };


        
        let cpu_particles_render_pipeline_alpha = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("CPU Particles Pipeline Layout Alpha"),
                bind_group_layouts: &[
                    Some(camera_bind_group_layout),
                    Some(depth_bind_group_layout),
                    Some(texture_bind_group_layout),
                ],
                immediate_size: 0,
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("CPU Particles Shader alpha"),
                source: wgpu::ShaderSource::Wgsl(include_str!("cpu_particles3d.wgsl").into()),
            };

            create_particles_render_pipeline(
                &device,
                &layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[Some(Particle3DInstance::desc())],
                shader,
                wgpu::BlendState::ALPHA_BLENDING,
            )
        };

        let cpu_particles_render_pipeline_additive = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("CPU Particles Pipeline Layout Additive"),
                bind_group_layouts: &[
                    Some(camera_bind_group_layout),
                    Some(depth_bind_group_layout),
                    Some(texture_bind_group_layout),
                ],
                immediate_size: 0,
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("CPU Particles Shader additive"),
                source: wgpu::ShaderSource::Wgsl(include_str!("cpu_particles3d.wgsl").into()),
            };

            create_particles_render_pipeline(
                &device,
                &layout,
                config.format,
                Some(Texture::DEPTH_FORMAT),
                &[Some(Particle3DInstance::desc())],
                shader,
                ADDITIVE_BLEND,
            )
        };



        Self {
            render_pipeline_layout,
            skinned_render_pipeline_layout,
            light_bind_group,
            camera_bind_group,
            render_pipeline,
            skinned_render_pipeline,
            light_render_pipeline,
            cpu_particles_render_pipeline_alpha,
            cpu_particles_render_pipeline_additive,
            // bone_matrices_bind_group_layout: bone_matrices_bind_group_layout.clone(),
        }
    }
}
