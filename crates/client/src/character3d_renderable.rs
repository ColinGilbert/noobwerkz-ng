// use ozz_animation_rs::*;
// use simple_animgraph::{animgraph::AnimGraph, animgraph_definition::AnimGraphDefinition};
use std::{collections::HashMap, rc::Rc};
use noobwerkz_shared::{character3d_common::*};


pub struct Character3DRenderable {
    //pub instance_idx: usize,
    pub common: Character3DCommon,
    // pub anim_graph: AnimGraph,
}

impl Character3DRenderable {
    pub fn new(
        // skeleton: Rc<Skeleton>,
        position: glam::Vec3,
        orientation: glam::Quat,
        animgraph_definition: &AnimGraphDefinition,
        animations_by_name: &HashMap<String, Rc<Animation>>,
    ) -> Option<Self> {
        let common = Character3DCommon::new(position, orientation, 1.0);
     let anim_graph = AnimGraph::new(skeleton, animgraph_definition, animations_by_name);
        match anim_graph {
            Ok(val) => {
                Some(Self {
                    //instance_idx,
                    common,
                    anim_graph: val,
                })
            }
            Err(err) => {
                println!(
                    "Failed to create character. Anim graph couldn't be created: {}",
                    err
                );
                return None;
            }
        }
    }
}
