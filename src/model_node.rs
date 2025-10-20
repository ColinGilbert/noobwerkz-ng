use crate::instance::Instance;
use crate::model::*;
pub struct ModelNode {
    pub model: Model,
    pub instances: Vec<Instance>,
}