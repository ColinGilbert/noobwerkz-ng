// This is where our game world resides.

use crate::{camera::Camera, model_node::ModelNode};

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub model_nodes: Vec<ModelNode>,
    pub skinned_model_nodes: Vec<ModelNode>,
    pub active_camera: usize,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cameras: Vec::<Camera>::new(),
            model_nodes: Vec::<ModelNode>::new(),
            skinned_model_nodes: Vec::<ModelNode>::new(),
            active_camera: 0
        }
    }
}