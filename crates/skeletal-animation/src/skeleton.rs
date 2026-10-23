use std::collections::HashMap;

use crate::{MAX_BONES, bone::*};

pub struct Skeleton {
    pub bones: [Bone; MAX_BONES], // These store local transforms 
    pub parents: [usize; MAX_BONES], // These store the parent bone indices.
    pub valid: [bool; MAX_BONES],
    pub inverse_bind_matrices: [glam::Mat4; MAX_BONES],
    pub num_bones: usize,
    pub bone_names_to_idx: HashMap<String, usize>,
    pub bone_idx_to_names: HashMap<usize, String>
}