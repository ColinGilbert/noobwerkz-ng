use crate::instance::Instance;
use crate::skinned_model::*;
use wgpu::{BindGroupLayout, util::*};

pub struct SkinnedModelNode {
    pub skinned_model_idx: usize,
    pub instances: Vec<Instance>,
    pub visible: Vec<bool>,
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
        num_bones: usize,
    ) -> Self {
        let len = instances.len();
        let bone_matrices = vec![AnimationMatrix { m: [[0.0; 4]; 4] }; len * num_bones];
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
        instances: Vec<Instance>,
        bone_matrices: &Vec<AnimationMatrix>,
    ) {

        self.instances = instances;

        self.storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Animation matrices storage buffer"),
            contents: bytemuck::cast_slice(&bone_matrices),
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
