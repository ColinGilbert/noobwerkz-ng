// This is where our game world resides.

use std::{collections::HashMap, rc::Rc};

use anim_graph_rs::{animgraph::AnimGraph, animgraph_definitions::AnimGraphDefinition};

use crate::{
    camera::Camera, character::*, instance::Instance, model_node::ModelNode,
    physics_context::PhysicsContext, skinned_model_node::SkinnedModelNode,
};

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub model_nodes: Vec<ModelNode>,
    pub skinned_model_nodes: Vec<SkinnedModelNode>,
    pub active_camera: usize,
    pub physics_context: PhysicsContext,
    pub characters: Vec<Character>,
    pub characters_by_name: HashMap<String, std::ops::Range<usize>>,
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
            characters_by_name: HashMap::new(),
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

    pub fn update_characters(
        &mut self,
        dt: web_time::Duration,
        queue: &wgpu::Queue,
        bones_storage_buffer: &wgpu::Buffer,
    ) {
        let mut output = Vec::<glam::Mat4>::new();
        for c in self.characters.iter_mut() {
            output.clear();
            self.skinned_model_nodes[c.skinned_model_node_idx]
                .instances
                .clear();
            self.skinned_model_nodes[c.skinned_model_node_idx]
                .bone_matrices
                .clear();
            c.anim_graph.evaluate(dt);
            let instance = Instance {
                position: c.position.into(),
                rotation: c.orientation,
                scale: glam::Vec3A::splat(1.0),
            };
            self.skinned_model_nodes[c.skinned_model_node_idx]
                .instances
                .push(instance);
            c.anim_graph.get_output(&mut output);
            for o in output.clone() {
                self.skinned_model_nodes[c.skinned_model_node_idx]
                    .bone_matrices
                    .push(o);
            }
        }
        
        queue.write_buffer(bones_storage_buffer, 0, bytemuck::cast_slice(&output));
    }

    pub fn add_characters(
        &mut self,
        device: &mut wgpu::Device,
        bone_matrices_bind_group_layout: &wgpu::BindGroupLayout,
        skinned_model_idx: usize,
        instances: &Vec<Instance>,
        animgraph_definition: &AnimGraphDefinition,
        skeleton: Rc<ozz_animation_rs::Skeleton>,
        animations_by_name: &HashMap<String, Rc<ozz_animation_rs::Animation>>,
        name: String,
    ) -> Option<std::ops::Range<usize>> {
        let start_idx = self.characters.len();
        self.skinned_model_nodes.push(SkinnedModelNode::new(
            device,
            bone_matrices_bind_group_layout,
            skinned_model_idx,
            instances,
            skeleton.clone(),
        ));
        let skinned_model_node_idx = self.skinned_model_nodes.len() - 1;
        for instance in instances {
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
                }
                None => {
                    println!("[Scene] add_character: Could not create anim graph from definition");
                    return None;
                }
            }
        }

        let results = std::ops::Range {
            start: start_idx,
            end: self.characters.len() - 1,
        };

        self.characters_by_name.insert(name, results.clone());

        Some(results)
    }

    pub fn change_anim_graphs(
        &mut self,
        range: std::ops::Range<usize>,
        definition: &AnimGraphDefinition,
        skeleton: Rc<ozz_animation_rs::Skeleton>,
        animations_by_name: &HashMap<String, Rc<ozz_animation_rs::Animation>>,
    ) -> bool {
        for i in range {
            let updated_anim_graph =
                AnimGraph::create_from_definition(skeleton.clone(), definition, animations_by_name);
            match updated_anim_graph {
                Some(val) => self.characters[i].anim_graph = val,
                None => {
                    println!(
                        "[Scene] Trying to change anim graph with an invalid animation graph definition"
                    );
                    return false;
                }
            }
        }
        true
    }
}
