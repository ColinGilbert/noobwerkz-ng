use crate::instance::Instance;
pub struct NormalMappedModelNode {
    pub model_idx: usize,
    pub instances: Vec<Instance>,
    pub visible: Vec<bool>,
}

impl NormalMappedModelNode {
    pub fn new(model_idx: usize, instances: Vec<Instance>) -> Self {
        let len = instances.len();
        Self {
            model_idx,
            instances,
            visible: vec![true; len],
        }
    }
}
