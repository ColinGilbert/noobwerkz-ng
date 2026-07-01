use simple_animgraph::{animgraph::AnimGraph, animgraph_definition::AnimGraphDefinition};
use ozz_animation_rs::*;
use std::{collections::HashMap, rc::Rc};

pub struct Character {
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
        position: glam::Vec3,
        orientation: glam::Quat,
        animgraph_definition: &AnimGraphDefinition,
        animations_by_name: &HashMap<String, Rc<Animation>>,
    ) -> Option<Self> {
        let anim_graph =
            AnimGraph::new(skeleton, animgraph_definition, animations_by_name);
        match anim_graph {
            Ok(val) => { Some(Self {
                //instance_idx,
                position,
                orientation,
                anim_graph: val,
            })}
            Err(err) => { println!("Failed to create character. Anim graph couldn't be created: {}", err); return None }
        }
    }

}
