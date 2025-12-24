use std::rc::Rc;

use crate::{instance::Instance};
// use rayon::prelude::*;
use wgpu::{BindGroupLayout, util::*};

pub struct SkinnedModelNode {
    pub skinned_model_idx: usize,
    pub instances: Vec<Instance>,
    pub num_bones: u32,
    pub bones_storage_buffer: wgpu::Buffer,
    pub num_bones_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bone_matrices: Vec<glam::Mat4>,
}

impl SkinnedModelNode {
    pub fn new(
        device: &mut wgpu::Device,
        bone_matrices_bind_group_layout: &BindGroupLayout,
        skinned_model_idx: usize,
        instances_arg: &Vec<Instance>,
        skeleton: Rc<ozz_animation_rs::Skeleton>,
    ) -> Self {
        // let mut playbacks = Vec::new();
        let skeleton = skeleton.clone();
        let num_bones = skeleton.num_joints() as u32;
        // let animation = skeletal_context.animations[0].clone();
        // let len = instances.len();
        let mut bone_matrices = Vec::<glam::Mat4>::new();
        let mut instances = Vec::new();
        for instance in instances_arg {
            let i = Instance{ position: instance.position, rotation: instance.rotation, scale: instance.scale};
            instances.push(i);
        }
        let num_bones_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Num bones uniform buffer"),
            contents: bytemuck::cast_slice(&[num_bones]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        for _ in 0..instances.len().saturating_sub(1) {
            // playbacks.push(futures::executor::block_on(OzzPlayback::new(
            //     &skeleton, &animation,
            // )));
            for i in 0..num_bones {
                bone_matrices.push(glam::Mat4::IDENTITY);
                println!("{}", i)
            }
        }

        // println!(
        //     "Bone matrices length: {}, Bone matrices size: {} MiB, num bones: {}",
        //     bone_matrices.len(),
        //     (bone_matrices.len() as f32 * 16.0 * 4.0) / (1024.0 * 1024.0),
        //     num_bones
        // );
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
            label: Some("Bone matrices bind group"),
        });


        Self {
            skinned_model_idx,
            instances,
            num_bones,
            bones_storage_buffer,
            num_bones_buffer,
            bind_group,
            bone_matrices
        }
    }

    // pub fn update(
    //     &mut self,
    //     queue: &mut wgpu::Queue,
    //     skinned_model: &SkinnedModel,
    //     dt: web_time::Duration,
    //     speed: f32,
    // ) {
    //     //self.bone_matrices.clear();
    //     //
    //     // This is for the multithreaded version
    //     //
    //     // self.bone_matrices = self
    //     //     .playbacks
    //     //     .par_iter_mut()
    //     //     .map(|p: &mut OzzPlayback| {
    //     //         p.update(dt, speed);
    //     //         let mut bones = Vec::<BoneMatrix>::new();
    //     //         for (i, mat) in (*p.models).read().unwrap().iter().enumerate() {
    //     //             let m = mat * skinned_model.inverse_bind_matrices[i];
    //     //             bones.push(BoneMatrix {
    //     //                 data: (m).to_cols_array_2d(),
    //     //             });
    //     //         }
    //     //         bones
    //     //     })
    //     //     .flatten()
    //     //     .collect::<Vec<BoneMatrix>>();

    //     // This is the single-threaded version
    //     //
    //     // for p in self.playbacks.iter_mut() {
    //     //     p.update(dt, speed);
    //     //     for (i, mat) in (*p.models).borrow().iter().enumerate() {
    //     //         let m = mat * skinned_model.inverse_bind_matrices[i];
    //     //         self.bone_matrices.push(BoneMatrix {
    //     //             data: m.to_cols_array_2d(),
    //     //         });
    //     //     }
    //     // }

    //     queue.write_buffer(
    //         &self.bones_storage_buffer,
    //         0,
    //         bytemuck::cast_slice(&self.bone_matrices),
    //     );
    // }
}
