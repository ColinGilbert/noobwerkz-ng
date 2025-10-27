use crate::graphics_context::*;
use crate::instance::Instance;
use crate::scene::*;
use crate::skeletal_context::*;
use crate::skinned_model::*;
use crate::skinned_model_node::*;
use crate::user_context::*;
use ozz_animation_rs::*;
use std::sync::{Arc, RwLock};

pub fn make_skinned_model_nodes(
    gfx_ctx: &mut GraphicsContext,
    user_ctx: &mut UserContext,
    skeletal_context_idx: usize,
    skinned_model_idx: usize,
    scene_idx: usize,
    instances: Vec<Instance>,
) {
    let s = &mut user_ctx.scenes[scene_idx];
    let model = &user_ctx.skinned_models[skinned_model_idx];
    let skeletal = &user_ctx.skeletals[skeletal_context_idx];
    let num_bones = skeletal.skeleton.num_joints();
    let node = SkinnedModelNode::new(&mut gfx_ctx.device, &gfx_ctx.bone_matrices_bind_group_layout, skinned_model_idx, instances, num_bones);
    s.skinned_model_nodes.push(node);
}

// pub async fn load_skeletal_model(
//     filepath: &str,
//     model_filename: &str,
//     skeleton_filename: &str,
//     animation_filenames: &Vec<String>,
//     default_material: Material,
//     device: &mut wgpu::Device,
//     queue: &mut wgpu::Queue,
//     texture_layout: &wgpu::BindGroupLayout,
//     bone_matrices_bind_group_layout: &wgpu::BindGroupLayout,

// ) {
//     let model = load_model_from_serialized(
//         filepath.to_string(),
//         model_filename.to_string(),
//         default_material,
//         device,
//         queue,
//         texture_layout,
//     )
//     .await
//     .unwrap();
//     match model {
//         GenericModel::Textured(_value) => (),
//         GenericModel::SkinnedTextured(value) => {

//         }
//     }
// }
