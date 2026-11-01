use noobwerkz_shared::bone::INVALID_BONE;
use noobwerkz_shared::{skeleton::*};
use crate::{operation_result::*};
use std::sync::Arc;

pub fn local_to_model(skeleton: Arc<Skeleton>, locals: &LocalToLocalResults) -> LocalToModelResults {
    let mut results = LocalToModelResults::new();
    // This made no difference as to the animation bug
    for b in skeleton.bones.to_vec() {
        results.model_matrices.push(glam::Mat4::from_scale_rotation_translation(b.scale, b.orientation, b.position));
    }
    for (i, parent) in skeleton.parents.iter().enumerate() {
        let local_matrix = glam::Mat4::from_scale_rotation_translation(locals.scales[i], locals.rotations[i], locals.translations[i]);
        if *parent != INVALID_BONE {
            // let parent_matrix = glam::Mat4::from_scale_rotation_translation(locals.scales[*parent], locals.rotations[*parent], locals.translations[*parent]);
            results.model_matrices[i] = results.model_matrices[*parent] * local_matrix;
        }
        else {
            results.model_matrices[i] = local_matrix;
        }
    }
    results
}