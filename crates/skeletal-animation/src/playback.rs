use crate::{MAX_BONES, skeleton::Skeleton, animation::Animation, operation_result::OperationResultsLocal, bone::Bone};



pub fn playback(skeleton: &Skeleton, animation:Animation, time: f32) -> OperationResultsLocal {
    OperationResultsLocal{
        bones: [Bone{ position: glam::Vec3::ZERO, orientation: glam::Quat::IDENTITY, scale: glam::Vec3::splat(1.0)}; MAX_BONES]
    }
}