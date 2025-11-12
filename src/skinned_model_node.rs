use crate::skinned_model::*;
use crate::{instance::Instance, skeletal_animate::*, skeletal_context::*};
use rayon::prelude::*;
use wgpu::{BindGroupLayout, util::*};

pub struct SkinnedModelNode {
    pub skinned_model_idx: usize,
    pub instances: Vec<Instance>,
    pub playbacks: Vec<OzzPlayback>,
    pub bone_matrices: Vec<BoneMatrix>,
    pub num_bones: u32,
    pub bones_storage_buffer: wgpu::Buffer,
    pub num_bones_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl SkinnedModelNode {
    pub fn new(
        device: &mut wgpu::Device,
        bone_matrices_bind_group_layout: &BindGroupLayout,
        //skinned_model: &SkinnedModel,
        skinned_model_idx: usize,
        instances: Vec<Instance>,
        skeletal_context: &SkeletalContext,
    ) -> Self {
        let mut playbacks = Vec::new();
        let skeleton = skeletal_context.skeleton.clone();
        let num_bones = skeleton.num_joints() as u32;
        let animation = skeletal_context.animations[0].clone();
        let len = instances.len();
        let mut bone_matrices = Vec::new();

        let num_bones_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Num bones uniform buffer"),
            contents: bytemuck::cast_slice(&[num_bones]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        for _i in 0..len {
            playbacks.push(futures::executor::block_on(OzzPlayback::new(
                &skeleton, &animation,
            )));
        }
        for p in &mut playbacks {
            for _m in &*(p.models).read().unwrap() {
                bone_matrices.push(BoneMatrix {
                    data: glam::Mat4::IDENTITY.to_cols_array_2d(),
                });
            }
        }

        println!(
            "Bone matrices length: {}, Bone matrices size: {} MiB, num bones: {}",
            bone_matrices.len(),
            (bone_matrices.len() as f32 * 16.0 * 4.0) / (1024.0 * 1024.0),
            num_bones
        );
        let bones_storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Animation matrices storage buffer"),
            contents: bytemuck::cast_slice(&bone_matrices),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bone_matrices_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: bones_storage_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: num_bones_buffer.as_entire_binding(),
                },
            ],
            label: Some("Animation matrices bind Group"),
        });

        Self {
            skinned_model_idx,
            instances,
            playbacks,
            bone_matrices,
            num_bones,
            bones_storage_buffer,
            num_bones_buffer,
            bind_group,
        }
    }

    pub fn update(
        &mut self,
        _device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
        skinned_model: &SkinnedModel,
        _bone_matrices_bind_group_layout: &BindGroupLayout,
        dt: web_time::Duration,
        speed: f32
    ) {
        // self.bone_matrices.clear();
        // for p in &mut self.playbacks {
        //     p.update(dt);
        //     for (i, mat) in (*p.models).read().unwrap().iter().enumerate() {
        //         let m = mat * skinned_model.inverse_bind_matrices[i];
        //         self.bone_matrices.push(BoneMatrix {
        //             data: (m).to_cols_array_2d(),
        //         });
        //     }
        // }
        self.bone_matrices.clear();
        self.bone_matrices = self.playbacks.par_iter_mut().map(|p| {
            p.update(dt, speed);
            let mut bones = Vec::<BoneMatrix>::new();
            for (i, mat) in (*p.models).read().unwrap().iter().enumerate() {
                let m = mat * skinned_model.inverse_bind_matrices[i];
                bones.push(BoneMatrix {
                    data: (m).to_cols_array_2d(),
                });
            }
            bones
        }).flatten().collect::<Vec<BoneMatrix>>();

        queue.write_buffer(
            &self.bones_storage_buffer,
            0,
            bytemuck::cast_slice(&self.bone_matrices),
        );
    }
}
