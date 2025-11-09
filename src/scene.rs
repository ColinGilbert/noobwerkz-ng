// This is where our game world resides.

use crate::{
    camera::Camera, model_node::ModelNode, physics_context::PhysicsContext, skinned_model_node::SkinnedModelNode,
};

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub model_nodes: Vec<ModelNode>,
    pub skinned_model_nodes: Vec<SkinnedModelNode>,
    pub active_camera: usize,
    pub physics_context: PhysicsContext,
}

impl Scene {
    pub fn new(gravity: &glam::Vec3) -> Self {
        Self {
            cameras: Vec::<Camera>::new(),
            model_nodes: Vec::<ModelNode>::new(),
            skinned_model_nodes: Vec::<SkinnedModelNode>::new(),
            active_camera: 0,
            physics_context: PhysicsContext::new(gravity),
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
}
