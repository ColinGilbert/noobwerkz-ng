use crate::instance::Instance;
use crate::model::*;
pub struct ModelNode {
    pub model: Model,
    pub instances: Vec<Instance>,
    pub visible: Vec<bool>,
}

impl ModelNode {
    pub fn new(model: Model, instances: Vec<Instance>) -> Self {
        let len = instances.len();
        Self {
            model,
            instances,
            visible: vec![true; len],
        }
    }
}
