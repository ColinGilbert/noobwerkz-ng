use noobwerkz_skeletal_animation::{skeleton::*, animation::*};
use std::sync::Arc;
use std::collections::HashMap;

pub struct SkeletalAnimation3DContext {
    pub skeleton: Arc<Skeleton>,
    pub animations: Vec<Arc<Animation>>,
    pub animations_idx_by_name: HashMap<String, usize>,
}