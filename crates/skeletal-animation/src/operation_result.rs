use crate::{MAX_BONES, bone::Bone};

pub struct OperationResults {
    pub translations: [glam::Vec3; MAX_BONES],
    pub rotations: [glam::Quat; MAX_BONES],
    pub scales: [glam::Vec3; MAX_BONES]
}

impl OperationResults {
    pub fn new() -> Self {
        Self {
            translations: [glam::Vec3::ZERO; MAX_BONES],
            rotations: [glam::Quat::IDENTITY; MAX_BONES],
            scales: [glam::Vec3::splat(1.0); MAX_BONES]
        }
    }
}