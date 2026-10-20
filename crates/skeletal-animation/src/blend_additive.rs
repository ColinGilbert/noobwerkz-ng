use crate::{MAX_BONES, operation_result::OperationResultsLocal, bone::Bone, skeleton::*};

// Stubbed out
pub fn blend_madditive(skeleton: &Skeleton, first: &OperationResultsLocal, second : &OperationResultsLocal, weight: f32, mask: u64) -> OperationResultsLocal {
    OperationResultsLocal{
        bones: [Bone{ position: glam::Vec3::ZERO, orientation: glam::Quat::IDENTITY, scale: glam::Vec3::splat(1.0)}; MAX_BONES]
    }
}