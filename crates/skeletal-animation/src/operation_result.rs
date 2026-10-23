use crate::{MAX_BONES};

pub struct LocalToLocalResults {
    pub translations: [glam::Vec3; MAX_BONES],
    pub rotations: [glam::Quat; MAX_BONES],
    pub scales: [glam::Vec3; MAX_BONES]
}

impl LocalToLocalResults {
    pub fn new() -> Self {
        Self {
            translations: [glam::Vec3::ZERO; MAX_BONES],
            rotations: [glam::Quat::IDENTITY; MAX_BONES],
            scales: [glam::Vec3::splat(1.0); MAX_BONES]
        }
    }
}

pub struct LocalToModelResults {
    pub model_matrices: [glam::Mat4; MAX_BONES],
}

impl LocalToModelResults {
    pub fn new() -> Self {
        Self {
            model_matrices: [glam::Mat4::IDENTITY; MAX_BONES]
        }
    }
}