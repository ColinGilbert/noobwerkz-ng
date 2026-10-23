use crate::{skeleton::*, operation_result::*};
use std::sync::Arc;
use std::iter::zip;

pub fn local_to_model(skeleton: Arc<Skeleton>, locals: &LocalToLocalResults) -> LocalToModelResults {
    let mut results = LocalToModelResults::new();
    for (i, parent_tuple) in zip(skeleton.parents, skeleton.valid).enumerate() {
        if parent_tuple.1 {
            let local_matrix = glam::Mat4::from_scale_rotation_translation(locals.scales[i], locals.rotations[i], locals.translations[i]);
            let parent_matrix = glam::Mat4::from_scale_rotation_translation(locals.scales[parent_tuple.0], locals.rotations[parent_tuple.0], locals.translations[parent_tuple.0]);
            results.model_matrices[i] = parent_matrix * local_matrix;
        }
    }
    results
}