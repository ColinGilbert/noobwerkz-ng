#[derive(Clone, Copy)]
pub struct Bone {
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    pub scale: glam::Vec3,
}

impl Bone {
    pub fn new() -> Self {
        Self {
            position: glam::Vec3::ZERO,
            orientation: glam::Quat::IDENTITY,
            scale: glam::Vec3::splat(1.0),
        }
    }
}
