use crate::{animation_graph::AnimGraph};

pub struct Character {
    pub skinned_model_node_idx: usize,
    pub instance_idx: usize,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    // TODO:
    // Colliders, animation blending, IK, pathfinding, AI
    pub anim_graph: AnimGraph, 
    pub bool_params: Vec<bool>,
    pub float_params: Vec<f32>,
    pub uint_params: Vec<usize>,
    pub int_params: Vec<i64>,
    pub vec3_params: Vec<[f32; 3]>,
}

impl Character {
    pub fn new(skinned_model_node_idx: usize, instance_idx: usize, position: glam::Vec3, orientation: glam::Quat) -> Self {
        Self {
            skinned_model_node_idx,
            instance_idx,
            position,
            orientation,
            anim_graph: AnimGraph::new(),
            bool_params: Vec::new(),
            float_params: Vec::new(),
            uint_params: Vec::new(),
            int_params: Vec::new(),
            vec3_params: Vec::new(),
        }
    }
}