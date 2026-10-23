// use ozz_animation_rs::*;
// use simple_animgraph::{animgraph::AnimGraph, animgraph_definition::AnimGraphDefinition};
use std::{collections::HashMap, sync::Arc};
use noobwerkz_shared::{character3d_common::*};
use noobwerkz_skeletal_animation::{animation::Animation, animation_graph::{AnimationGraph, AnimationIndex, Animations}, animation_graph_definition::AnimationGraphDefinition, skeleton::Skeleton};


pub struct Character3DRenderable {
    //pub instance_idx: usize,
    pub common: Character3DCommon,
    pub anim_graph: AnimationGraph,
}

impl Character3DRenderable {
    pub fn new(
        skeleton: Arc<Skeleton>,
        position: glam::Vec3,
        orientation: glam::Quat,
        animgraph_definition: &AnimationGraphDefinition,
        animations: Animations<Animation>,
        animations_by_name: &HashMap<String, AnimationIndex>,
    ) -> Option<Self> {
        let common = Character3DCommon::new(position, orientation, 1.0);
     let anim_graph = AnimationGraph::new(skeleton, animations, animgraph_definition, animations_by_name);
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
