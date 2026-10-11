// This is where our game world resides.

use std::{collections::HashMap, rc::Rc};

use simple_animgraph::{animgraph::AnimGraph, animgraph_definition::AnimGraphDefinition};

use crate::{
    camera::{Camera, Projection}, character::Character3D, index_types::*, instance::Instance, model3d_node::Model3DNode, physics_context::{LiquidWorldProperties, PhysicsContext}, skinned_model::SkinnedModel, skinned_model_node::SkinnedModelNode
};

pub struct CharactersContext {
    pub characters: Vec<Character3D>,
    pub skinned_model_node: SkinnedModelNode,
}

pub struct Scene3D {
    pub cameras: Cameras<Camera>,
    pub model_nodes: Model3DNodes<Model3DNode>,
    pub active_camera: CameraIndex,
    pub physics_context: PhysicsContext,
    pub characters_contexts: CharactersContexts<CharactersContext>,
    pub character_types_by_name: HashMap<String, CharactersContextIndex>,
}

impl Scene3D {
    pub fn new(gravity: &glam::Vec3) -> Self {
        let mut cameras = Cameras::new();
        cameras.push(Camera::new(&glam::Vec3{x: 0.0, y:0.0, z: 0.0}, &glam::Vec3{x: 0.0, y: 0.0, z: 0.0}, &glam::Vec3::Y, 1.0, 1.0, Projection::new(600, 800, 1.0, 0.0, 100.0)));
        Self {
            cameras,
            model_nodes: Model3DNodes::new(),
            active_camera: CameraIndex::new(0),
            physics_context: PhysicsContext::new(gravity, LiquidWorldProperties { has_liquid_world: false, particle_radius: 0.02, smoothing_factor: 2.0}),
            characters_contexts: CharactersContexts::new(),
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
        self.physics_context.rigid_world.step()
    }

    pub fn update_characters(&mut self, dt: web_time::Duration, skinned_models: &SkinnedModels<SkinnedModel>, queue: &wgpu::Queue) {
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
        skinned_model_idx: SkinnedModelIndex,
        instances: &Vec<Instance>,
        animgraph_definition: &AnimGraphDefinition,
        skeleton: Rc<ozz_animation_rs::Skeleton>,
        animations_by_name: &HashMap<String, Rc<ozz_animation_rs::Animation>>,
        name: String,
    ) -> Option<CharactersContextIndex> {
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
        let characters_ctx_idx = CharactersContextIndex::new(self.characters_contexts.len() - 1);
        for instance in instances {
            let character = Character3D::new(
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
        character_type_idx: CharactersContextIndex,
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
