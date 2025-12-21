use anim_graph_rs::animgraph::AnimGraph;
use ozz_animation_rs::*;
use std::{rc::Rc};

pub struct Character {
    pub skinned_model_node_idx: usize,
    pub instance_idx: usize,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    // TODO:
    // Colliders, animation blending, IK, pathfinding
    pub anim_graph: AnimGraph,

}

impl Character {
    pub fn new(skeleton: Rc<Skeleton>, skinned_model_node_idx: usize, instance_idx: usize, position: glam::Vec3, orientation: glam::Quat) -> Self {
        Self {
            skinned_model_node_idx,
            instance_idx,
            position,
            orientation,
            anim_graph: AnimGraph::new(skeleton),
        }
    }
}