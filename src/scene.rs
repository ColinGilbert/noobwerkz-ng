// This is where our game world resides.

use crate::{camera::Camera, model_node::NormalMappedModelNode};

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub model_nodes: Vec<NormalMappedModelNode>,
    pub active_camera: usize,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cameras: Vec::<Camera>::new(),
            model_nodes: Vec::<NormalMappedModelNode>::new(),
            active_camera: 0
        }
    }
}