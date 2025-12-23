use anim_graph_rs::{animgraph::AnimGraph, animgraph_definitions::AnimGraphDefinition};
use ozz_animation_rs::*;
use std::{collections::HashMap, rc::Rc};

pub struct Character {
    pub skinned_model_node_idx: usize,
    //pub instance_idx: usize,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    // TODO:
    // Colliders, IK, pathfinding
    pub anim_graph: AnimGraph,
}

impl Character {
    pub fn new(
        skeleton: Rc<Skeleton>,
        skinned_model_node_idx: usize,
        //instance_idx: usize,
        position: glam::Vec3,
        orientation: glam::Quat,
        animgraph_definition: &AnimGraphDefinition,
        animations_by_name: &HashMap<String, Rc<Animation>>,
    ) -> Option<Self> {
        let anim_graph =
            AnimGraph::create_from_definition(skeleton, animgraph_definition, animations_by_name);
        match anim_graph {
            Some(val) => Some(Self {
                skinned_model_node_idx,
                //instance_idx,
                position,
                orientation,
                anim_graph: val,
            }),
            None => None,
        }
    }
}
