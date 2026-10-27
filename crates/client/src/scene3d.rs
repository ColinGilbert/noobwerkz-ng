// This is where our game world resides.

use std::{collections::HashMap, sync::Arc};

// use simple_animgraph::{animgraph::AnimGraph, animgraph_definition::AnimGraphDefinition};
use noobwerkz_shared::{physics3d_context::*, skeleton::*};
use noobwerkz_skeletal_animation::{
    animation::Animation,
    animation_graph::{AnimationGraph, AnimationIndex, Animations},
    animation_graph_definition::AnimationGraphDefinition,
};

use simple_particles::{ParticleSystem, render::ParticleRenderer};

use crate::{
    camera3d::*, character3d_renderable::Character3DRenderable, index_types::*, instance3d::Instance3D, model3d_node::Model3DNode, skinned_model3d::SkinnedModel3D, skinned_model3d_node::SkinnedModel3DNode,
};

pub struct Characters3DContext {
    pub characters: Characters3D<Character3DRenderable>,
    pub skinned_model_node: SkinnedModel3DNode,
}

pub struct Scene3D {
    pub cameras: Cameras3D<Camera3D>,
    pub model_nodes: Model3DNodes<Model3DNode>,
    pub active_camera: Camera3DIndex,
    pub physics_context: Physics3DContext,
    pub characters_contexts: Characters3DContexts<Characters3DContext>,
    pub character_types_by_name: HashMap<String, Characters3DContextIndex>,
    pub particle_system: ParticleSystem,
}

impl Scene3D {
    pub fn new(gravity: &glam::Vec3) -> Self {
        let cameras = Cameras3D::new();
        //cameras.push(Camera::new(&glam::Vec3{x: 0.0, y:0.0, z: 0.0}, &glam::Vec3{x: 0.0, y: 0.0, z: 0.0}, &glam::Vec3::Y, 1.0, 1.0, Projection::new(600, 800, 1.0, 0.0, 100.0)));
        Self {
            cameras,
            model_nodes: Model3DNodes::new(),
            active_camera: Camera3DIndex::new(0),
            physics_context: Physics3DContext::new(gravity),
            characters_contexts: Characters3DContexts::new(),
            character_types_by_name: HashMap::new(),
            particle_system: ParticleSystem::new(),
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

    pub fn update_particles(&mut self, dt: web_time::Duration) {
        self.particle_system.update(dt.as_secs_f32());
    }


    pub fn update_characters(
        &mut self,
        dt: web_time::Duration,
        skinned_models: &SkinnedModels3D<SkinnedModel3D>,
        queue: &wgpu::Queue,
    ) {
        for characters_ctx in self.characters_contexts.iter_mut() {
            characters_ctx.skinned_model_node.instances.clear();
            characters_ctx.skinned_model_node.bone_matrices.clear();

            for c in &mut characters_ctx.characters {
                let r = c.anim_graph.evaluate(dt);
                match r {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Could not evaluate character's anim graph: {}", err)
                    }
                }
                let instance = Instance3D {
                    position: c.common.position.into(),
                    orientation: c.common.orientation,
                    scale: glam::Vec3A::splat(1.0),
                };

                characters_ctx.skinned_model_node.instances.push(instance);

                let output = &c.anim_graph.output_model;

                for (i, o) in output.model_matrices.iter().enumerate() {
                    characters_ctx.skinned_model_node.bone_matrices.push(
                        o * skinned_models[characters_ctx.skinned_model_node.skinned_model_idx]
                            .skeleton
                            .inverse_bind_matrices[i],
                    );
                }
            }

            let bones_storage_buffer = &characters_ctx.skinned_model_node.bones_storage_buffer;
            queue.write_buffer(
                &bones_storage_buffer,
                0,
                bytemuck::cast_slice(&characters_ctx.skinned_model_node.bone_matrices),
            );
        }
    }

    pub fn add_characters(
        &mut self,
        device: &mut wgpu::Device,
        bone_matrices_bind_group_layout: &wgpu::BindGroupLayout,
        skinned_model_idx: SkinnedModel3DIndex,
        instances: &Vec<Instance3D>,
        animgraph_definition: &AnimationGraphDefinition,
        animations: Animations<Animation>,
        skeleton: Arc<Skeleton>,
        animations_by_name: &HashMap<String, AnimationIndex>,
        name: String,
    ) -> Option<Characters3DContextIndex> {
        self.characters_contexts.push(Characters3DContext {
            characters: Characters3D::new(),
            skinned_model_node: SkinnedModel3DNode::new(
                device,
                bone_matrices_bind_group_layout,
                skinned_model_idx,
                instances,
                skeleton.clone(),
            ),
        });
        println!("ADDING CHARACTERS: Instance count: {}", instances.len());
        let characters_ctx_idx = Characters3DContextIndex::new(self.characters_contexts.len() - 1);
        for instance in instances {
            let character = Character3DRenderable::new(
                skeleton.clone(),
                instance.position.into(),
                instance.orientation,
                animgraph_definition,
                animations.clone(),
                animations_by_name,
            );
            match character {
                Some(val) => {
                    self.characters_contexts[characters_ctx_idx]
                        .characters
                        .push(val);
                }
                None => {
                    println!("[Scene] add_character: Could not create anim graph from definition");
                    return None;
                }
            }
        }

        self.character_types_by_name
            .insert(name, characters_ctx_idx);

        Some(characters_ctx_idx)
    }

    pub fn change_anim_graphs(
        &mut self,
        character_type_idx: Characters3DContextIndex,
        definition: &AnimationGraphDefinition,
        skeleton: Arc<Skeleton>,
        animations: Animations<Animation>,
        animations_by_name: &HashMap<String, AnimationIndex>,
    ) -> bool {
        for c in self.characters_contexts[character_type_idx]
            .characters
            .iter_mut()
        {
            let updated_anim_graph = AnimationGraph::new(
                skeleton.clone(),
                animations.clone(),
                definition,
                animations_by_name,
            );
            match updated_anim_graph {
                Ok(val) => c.anim_graph = val,
                Err(err) => {
                    println!(
                        "[Scene] Trying to change character to use an anim graph with an invalid definition, {}",
                        err
                    );
                    return false;
                }
            }
        }
        true
    }
}
