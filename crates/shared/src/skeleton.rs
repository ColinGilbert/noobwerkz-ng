use std::collections::HashMap;


use crate::{bone::*};

pub struct Skeleton {
    pub bones: Vec<Bone>,    // These store local transforms
    pub parents: Vec<usize>, // These store the parent bone indices.
    pub valid: Vec<bool>,
    pub inverse_bind_matrices: Vec<glam::Mat4>,
    pub num_bones: usize,
    pub bone_names_to_idx: HashMap<String, usize>,
    pub bone_idx_to_names: HashMap<usize, String>,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            bones: Vec::new(),
            parents: Vec::new(), // These store the parent bone indices.
            valid: Vec::new(),
            inverse_bind_matrices: Vec::new(),
            num_bones: 0,
            bone_names_to_idx: HashMap::new(),
            bone_idx_to_names: HashMap::new(),
        }
    }
}
