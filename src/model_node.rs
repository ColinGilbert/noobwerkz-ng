use crate::instance::Instance;

pub struct ModelNode {
    pub model_idx: usize,
    pub instances: Vec<Instance>,
    //ub visible: Vec<bool>,
}

impl ModelNode {
    pub fn new(model_idx: usize, instances: Vec<Instance>) -> Self {
        //let len = instances.len();
        Self {
            model_idx,
            instances,
            //visible: vec![true; len],
        }
    }
}
