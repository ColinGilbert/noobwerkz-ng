// This is where our game world resides.

use std::{collections::HashMap, rc::Rc};

use anim_graph_rs::{animgraph::AnimGraph, animgraph_definitions::AnimGraphDefinition};

use crate::{
    camera::Camera,
    character::*,
    instance::Instance,
    model_node::ModelNode,
    physics_context::PhysicsContext,
    skinned_model_node::{self, SkinnedModelNode},
};

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub model_nodes: Vec<ModelNode>,
    pub skinned_model_nodes: Vec<SkinnedModelNode>,
    pub active_camera: usize,
    pub physics_context: PhysicsContext,
    pub characters: Vec<Character>,
}

impl Scene {
    pub fn new(gravity: &glam::Vec3) -> Self {
        Self {
            cameras: Vec::<Camera>::new(),
            model_nodes: Vec::<ModelNode>::new(),
            skinned_model_nodes: Vec::<SkinnedModelNode>::new(),
            active_camera: 0,
            physics_context: PhysicsContext::new(gravity),
            characters: Vec::new(),
        }
    }

    pub fn load_physics(&mut self, data: &Vec<u8>) {
        self.physics_context = bincode::serde::decode_from_slice(data, bincode::config::standard())
            .unwrap()
            .0;
    }

    pub fn save_physics(&self) -> Vec<u8> {
        let serialized =
            bincode::serde::encode_to_vec(&self.physics_context, bincode::config::standard())
                .unwrap();
        serialized
    }

    pub fn step_physics(&mut self) {
        self.physics_context.step()
    }

    pub fn eval_characters(&mut self, dt: web_time::Duration) {
        let mut output = Vec::<glam::Mat4>::new();
        for c in self.characters.iter_mut() {
            output.clear();
            self.skinned_model_nodes[c.skinned_model_node_idx].instances.clear();
            self.skinned_model_nodes[c.skinned_model_node_idx].bone_matrices.clear();
            c.anim_graph.evaluate(dt);
            let instance = Instance {position: c.position.into(), rotation: c.orientation, scale: glam::Vec3A::splat(1.0)};
            self.skinned_model_nodes[c.skinned_model_node_idx].instances.push(instance);
            c.anim_graph.get_output(&mut output);
            for o in output.clone() {
                self.skinned_model_nodes[c.skinned_model_node_idx]
                    .bone_matrices
                    .push(o);
            }
        }
    }

    pub fn add_characters(
        &mut self,
        skinned_model_node_idx: usize,
        instances: Vec<Instance>,
        animgraph_definition: &AnimGraphDefinition,
        skeleton: Rc<ozz_animation_rs::Skeleton>,
        animations_by_name: &HashMap<String, Rc<ozz_animation_rs::Animation>>,
    ) -> usize {
        for instance in instances {
            // let character_idx = self.characters.len();
            // // let character_instance = CharacterInstance {
            // //     pos_rot_scale: Instance {
            // //         position: instance.position,
            // //         rotation: instance.rotation,
            // //         scale: glam::Vec3A::splat(1.0),
            // //     },
            // //     character_idx,
            // // };

            let character = Character::new(
                skeleton.clone(),
                skinned_model_node_idx,
                instance.position.into(),
                instance.rotation,
                animgraph_definition,
                animations_by_name,
            );
            match character {
                Some(val) => {
                    self.characters.push(val);
                    //self.skinned_model_nodes[skinned_model_node_idx]
                    //    .instances
                    //    .push(character_instance);
                }
                None => {
                    println!("[Scene] add_character: Could not create anim graph from definition")
                }
            }
        }

        self.characters.len() - 1
    }
}
