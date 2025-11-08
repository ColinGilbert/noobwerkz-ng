// This is where our game world resides.

use crate::{camera::Camera, model_node::ModelNode, skinned_model_node::SkinnedModelNode, physics_context::*};

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub model_nodes: Vec<ModelNode>,
    pub skinned_model_nodes: Vec<SkinnedModelNode>,
    pub active_camera: usize,
    pub physics: PhysicsContext
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cameras: Vec::<Camera>::new(),
            model_nodes: Vec::<ModelNode>::new(),
            skinned_model_nodes: Vec::<SkinnedModelNode>::new(),
            active_camera: 0,
            physics: PhysicsContext::new(),
        }
    }
}