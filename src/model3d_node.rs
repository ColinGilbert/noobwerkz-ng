use crate::{index_types::*, instance::Instance};

pub struct Model3DNode {
    pub model_idx: Model3DIndex,
    pub instances: Instances<Instance>,
    // pub visible: Vec<bool>,
}

impl Model3DNode {
    pub fn new(model_idx: Model3DIndex, instances: Instances<Instance>) -> Self {
        // let len = instances.len();
        Self {
            model_idx,
            instances,
            // visible: vec![true; len],
        }
    }
}
