use crate::instance::Instance;

pub enum ModelType {
    Textured, TexturedSkinned,
}
pub struct ModelNode {
    pub model_type: ModelType, 
    pub model_idx: usize,
    pub instances: Vec<Instance>,
    pub visible: Vec<bool>,
}

impl ModelNode {
    pub fn new(model_type: ModelType, model_idx: usize, instances: Vec<Instance>) -> Self {
        let len = instances.len();
        Self {
            model_type,
            model_idx,
            instances,
            visible: vec![true; len],
        }
    }
}