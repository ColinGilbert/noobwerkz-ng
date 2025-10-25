use crate::instance::Instance;
use crate::generic_model::*;

pub enum ModelType {
    DiffuseMapped, NormalMapped, SkinnedDiffusedMapped, SkinnedNormalMapped
}
pub struct ModelNode {
    pub model_type: GenericModel, 
    pub model_idx: usize,
    pub instances: Vec<Instance>,
    pub visible: Vec<bool>,
}

impl ModelNode {
    pub fn new(model_type: GenericModel, model_idx: usize, instances: Vec<Instance>) -> Self {
        let len = instances.len();
        Self {
            model_type,
            model_idx,
            instances,
            visible: vec![true; len],
        }
    }
}