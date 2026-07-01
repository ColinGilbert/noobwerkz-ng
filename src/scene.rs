// This is where our game world resides.

use std::{collections::HashMap, rc::Rc};

use simple_animgraph::{animgraph::AnimGraph, animgraph_definition::AnimGraphDefinition};

use crate::{
    camera::Camera, instance::Instance, model_node::ModelNode, character::Character, 
    physics_context::PhysicsContext, skinned_model_node::SkinnedModelNode, skinned_model::SkinnedModel
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

    // pub fn load_physics(&mut self, data: &Vec<u8>) {
    //     self.physics_context = wincode::decode_from_slice(data)
    //         .unwrap()
    //         .0;
    // }

    // pub fn save_physics(&self) -> Vec<u8> {
    //     let serialized =
    //         wincode::encode_to_vec(&self.physics_context, wincode::config::standard())
    //             .unwrap();
    //     serialized
    // }

    pub fn step_physics(&mut self) {
        self.physics_context.step()
    }

    pub fn update_characters(&mut self, dt: web_time::Duration, skinned_models: &Vec<SkinnedModel>, queue: &wgpu::Queue) {
        for characters_ctx in self.characters_contexts.iter_mut() {
            
            characters_ctx.skinned_model_node.instances.clear();
            characters_ctx.skinned_model_node.bone_matrices.clear();
            
            for c in &mut characters_ctx.characters {
                let r = c.anim_graph.evaluate(dt);
                match r {
                    Ok(_) => {}
                    Err(err) => {println!("Could not evaluate character's anim graph: {}", err)}
                }
                let instance = Instance {
                    position: c.position.into(),
                    orientation: c.orientation,
                    scale: glam::Vec3A::splat(1.0),
                };
                
                characters_ctx.skinned_model_node.instances.push(instance);
                
                let output = c.anim_graph.get_skeletal_matrices();
                
                for (i, o) in output.borrow().iter().enumerate() {
                    characters_ctx.skinned_model_node.bone_matrices.push(o * skinned_models[characters_ctx.skinned_model_node.skinned_model_idx].inverse_bind_matrices[i]);
                }
            }
            
            let bones_storage_buffer = &characters_ctx.skinned_model_node.bones_storage_buffer;
            queue.write_buffer(&bones_storage_buffer, 0, bytemuck::cast_slice(&characters_ctx.skinned_model_node.bone_matrices));
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
                instance.orientation,
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
                AnimGraph::new(skeleton.clone(), definition, animations_by_name);
            match updated_anim_graph {
                Ok(val) => c.anim_graph = val,
                Err(err) => {
                    println!(
                        "[Scene] Trying to change character to use an anim graph with an invalid definition, {}", err
                    );
                    return false;
                }
            }
        }
        true
    }
}
