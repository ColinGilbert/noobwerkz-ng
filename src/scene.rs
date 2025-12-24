// This is where our game world resides.

use std::{collections::HashMap, rc::Rc};

use anim_graph_rs::{animgraph::AnimGraph, animgraph_definitions::AnimGraphDefinition};

use crate::{
    camera::Camera, character::Character, instance::Instance, model_node::ModelNode,
    physics_context::PhysicsContext, skinned_model_node::SkinnedModelNode,
};

pub struct CharactersContext {
    pub characters: Vec<Character>,
    pub skinned_model_node: SkinnedModelNode,
}
pub struct Scene {
    pub cameras: Vec<Camera>,
    pub model_nodes: Vec<ModelNode>,
    pub active_camera: usize,
    pub physics_context: PhysicsContext,
    pub characters_contexts: Vec<CharactersContext>,
    pub character_types_by_name: HashMap<String, usize>,
}

impl Scene {
    pub fn new(gravity: &glam::Vec3) -> Self {
        Self {
            cameras: Vec::<Camera>::new(),
            model_nodes: Vec::<ModelNode>::new(),
            active_camera: 0,
            physics_context: PhysicsContext::new(gravity),
            characters_contexts: Vec::new(),
            character_types_by_name: HashMap::new(),
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

    pub fn update_characters(&mut self, dt: web_time::Duration, queue: &wgpu::Queue) {
        let mut output = Vec::<glam::Mat4>::new();
        for characters_ctx in self.characters_contexts.iter_mut() {
            let bones_storage_buffer = &characters_ctx.skinned_model_node.bones_storage_buffer;

            output.clear();

            characters_ctx.skinned_model_node.instances.clear();
            characters_ctx.skinned_model_node.bone_matrices.clear();
            for c in &mut characters_ctx.characters {

                c.anim_graph.evaluate(dt);

                let instance = Instance {
                    position: c.position.into(),
                    rotation: c.orientation,
                    scale: glam::Vec3A::splat(1.0),
                };

                characters_ctx.skinned_model_node.instances.push(instance);

                c.anim_graph.get_output(&mut output);

                for o in output.clone() {
                    characters_ctx.skinned_model_node.bone_matrices.push(o);
                }
            }

            queue.write_buffer(&bones_storage_buffer, 0, bytemuck::cast_slice(&output));
        }
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
    ) -> Option<usize> {
        self.characters_contexts.push(CharactersContext {
            characters: Vec::new(),
            skinned_model_node: SkinnedModelNode::new(
                device,
                bone_matrices_bind_group_layout,
                skinned_model_idx,
                instances,
                skeleton.clone(),
            ),
        });
        println!("ADDING CHARACTERS: Instance count: {}", instances.len());
        let characters_ctx_idx = self.characters_contexts.len() - 1;
        for instance in instances {
            let character = Character::new(
                skeleton.clone(),
                instance.position.into(),
                instance.rotation,
                animgraph_definition,
                animations_by_name,
            );
            match character {
                Some(val) => {
                    self.characters_contexts[characters_ctx_idx].characters.push(val);
                }
                None => {
                    println!("[Scene] add_character: Could not create anim graph from definition");
                    return None;
                }
            }
        }

        self.character_types_by_name.insert(name, characters_ctx_idx);

        Some(characters_ctx_idx)
    }

    pub fn change_anim_graphs(
        &mut self,
        character_type_idx: usize,
        definition: &AnimGraphDefinition,
        skeleton: Rc<ozz_animation_rs::Skeleton>,
        animations_by_name: &HashMap<String, Rc<ozz_animation_rs::Animation>>,
    ) -> bool {
        for c in self.characters_contexts[character_type_idx].characters.iter_mut() {
            let updated_anim_graph =
                AnimGraph::create_from_definition(skeleton.clone(), definition, animations_by_name);
            match updated_anim_graph {
                Some(val) => c.anim_graph = val,
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
