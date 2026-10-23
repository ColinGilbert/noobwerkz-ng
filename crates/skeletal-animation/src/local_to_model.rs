use noobwerkz_shared::bone::INVALID_BONE;
use noobwerkz_shared::{skeleton::*};
use crate::{operation_result::*};
use std::sync::Arc;

pub fn local_to_model(skeleton: Arc<Skeleton>, locals: &LocalToLocalResults) -> LocalToModelResults {
    let mut results = LocalToModelResults::new();
    for (i,parent) in skeleton.parents.iter().enumerate() {
        if *parent != INVALID_BONE {
            let local_matrix = glam::Mat4::from_scale_rotation_translation(locals.scales[i], locals.rotations[i], locals.translations[i]);
            let parent_matrix = glam::Mat4::from_scale_rotation_translation(locals.scales[*parent], locals.rotations[*parent], locals.translations[*parent]);
            results.model_matrices[i] = parent_matrix * local_matrix;
        }
        else {
            results.model_matrices[i] = glam::Mat4::from_scale_rotation_translation(locals.scales[i], locals.rotations[i], locals.translations[i]);
        }
    }
    results
}