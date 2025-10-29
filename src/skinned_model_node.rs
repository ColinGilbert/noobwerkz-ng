use crate::{instance::Instance, skeletal_context::*, skeletal_animate::*};
use crate::skinned_model::*;
use futures::executor::block_on;
use wgpu::{BindGroupLayout, util::*};

pub struct SkinnedModelNode {
    pub skinned_model_idx: usize,
    pub instances: Vec<Instance>,
    pub visible: Vec<bool>,
    pub playbacks: Vec<OzzPlayback>,
    pub bone_matrices: Vec<AnimationMatrix>,
    pub num_bones: usize,
    pub storage_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

impl SkinnedModelNode {
    pub fn new(
        device: &mut wgpu::Device,
        bone_matrices_bind_group_layout: &BindGroupLayout,
        skinned_model_idx: usize,
        instances: Vec<Instance>,
        skeletal_context: &SkeletalContext,
    ) -> Self {
        
        let mut playbacks = Vec::new();
        let skeleton = skeletal_context.skeleton.clone();
        let num_bones = skeleton.num_joints();
        let animation = skeletal_context.animations[0].clone();
        let len = instances.len();
        let mut bone_matrices = Vec::new();

        for _i in 0..len {
            playbacks.push(block_on(OzzPlayback::new(&skeleton, &animation)));
        }

        for p in &mut playbacks {
            p.update(web_time::Duration::from_secs(0));
            let bone_transforms = p.bone_trans();
            for b in bone_transforms {
                bone_matrices.push(AnimationMatrix { data: (glam::Mat4::IDENTITY * glam::Mat4::from_scale(glam::Vec3 {x: b.scale, y: b.scale, z: b.scale}) * glam::Mat4::from_quat(b.rotation) * glam::Mat4::from_translation(b.position)).to_cols_array_2d()});
            }
        }

        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Animation matrices storage buffer"),
            contents: bytemuck::cast_slice(&bone_matrices),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bone_matrices_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
            label: Some("Animation matrices bind Group"),
        });
        Self {
            skinned_model_idx,
            instances,
            visible: vec![true; len],
            playbacks,
            bone_matrices,
            num_bones,
            storage_buffer,
            bind_group,
        }
    }

    pub fn update(
        &mut self,
        device: &mut wgpu::Device,
        bone_matrices_bind_group_layout: &BindGroupLayout,
        dt: web_time::Duration,
    ) {
        self.bone_matrices.clear();


        for p in &mut self.playbacks {
            p.update(dt);
            let bone_transforms = p.bone_trans();
            for b in bone_transforms {
                self.bone_matrices.push(AnimationMatrix { data: (glam::Mat4::IDENTITY * glam::Mat4::from_scale(glam::Vec3 {x: b.scale, y: b.scale, z: b.scale}) * glam::Mat4::from_quat(b.rotation) * glam::Mat4::from_translation(b.position)).to_cols_array_2d()});
            }
        }
        self.storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Animation matrices storage buffer"),
            contents: bytemuck::cast_slice(&self.bone_matrices),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bone_matrices_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.storage_buffer.as_entire_binding(),
            }],
            label: Some("Animation matrices bind Group"),
        });
    }
}
