use crate::{index_types::Model3DIndex, instance::Instance};

pub struct Model3DNode {
    pub model_idx: Model3DIndex,
    pub instances: Vec<Instance>,
    // pub visible: Vec<bool>,
}

impl Model3DNode {
    pub fn new(model_idx: Model3DIndex, instances: Vec<Instance>) -> Self {
        // let len = instances.len();
        Self {
            model_idx,
            instances,
            // visible: vec![true; len],
        }
    }
}
