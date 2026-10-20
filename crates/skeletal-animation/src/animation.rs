use crate::keyframe::KeyFrame;

// Consists of several keyframes
pub struct Animation {
    pub translation_keys: Vec<glam::Vec3>,
    pub rotation_keys: Vec<glam::Quat>,
    pub scale_keys: Vec<glam::Vec3>,
    pub timepoints: Vec<f32>,
    pub duration: f32,
    pub num_tracks: usize,
}