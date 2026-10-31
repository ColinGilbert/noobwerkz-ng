// Each channel covers a single transform (either translation, rotation, or scale) for a single bone.
// This is because each bone may have a different set of keyframes.

use noobwerkz_shared::serialized_model3d::*;

#[derive(Clone)]
pub enum KeyframeKeys {
    Translation(Vec<glam::Vec3>),
    Rotation(Vec<glam::Quat>),
    Scale(Vec<glam::Vec3>),
}

#[derive(Clone)]
pub struct KeyframeResult {
    pub translation: glam::Vec3,
    pub orientation: glam::Quat,
    pub scale: glam::Vec3,
}

impl KeyframeResult {
    pub fn new() -> Self {
        Self {
            translation: glam::Vec3::splat(0.0),
            orientation: glam::Quat::IDENTITY,
            scale: glam::Vec3::splat(1.0),
        }
    }
}

#[derive(Clone)]
pub struct SeekResults {
    pub t_between_frames: f32,
    pub index: usize,
}

// Consists of several keyframes
#[derive(Clone)]
pub struct AnimationChannel {
    pub bone_index: usize,
    pub data: KeyframeKeys,
    pub times: Vec<f32>,
    pub seek_results: SeekResults,
}

impl AnimationChannel {
    pub fn from_serialized(serialized: &SerializedAnimation3DChannel) -> Option<Self> {
        let bone_index = serialized.bone;

        match &serialized.data {
            SerializedAnimation3DKeyframeTypes::Translations(val) => {
                let mut translation_keys = Vec::new();
                for t in val {
                    translation_keys.push(glam::Vec3::from_array(*t));
                }
                let channel = Self {
                    bone_index,
                    data: KeyframeKeys::Translation(translation_keys),
                    times: serialized.times.clone(),
                    seek_results: SeekResults {
                        t_between_frames: 0.0,
                        index: 0,
                    },
                };
                Some(channel)
            }

            SerializedAnimation3DKeyframeTypes::Rotations(val) => {
                let mut rotation_keys = Vec::new();
                for r in val {
                    rotation_keys.push(glam::Quat::from_array(*r));
                }
                let channel = Self {
                    bone_index,
                    data: KeyframeKeys::Rotation(rotation_keys),
                    times: serialized.times.clone(),
                    seek_results: SeekResults {
                        t_between_frames: 0.0,
                        index: 0,
                    },
                };
                Some(channel)
            }

            SerializedAnimation3DKeyframeTypes::Scales(val) => {
                let mut scale_keys = Vec::new();
                for s in val {
                    scale_keys.push(glam::Vec3::from_array(*s));
                }
                let channel = Self {
                    bone_index,
                    data: KeyframeKeys::Scale(scale_keys),
                    times: serialized.times.clone(),
                    seek_results: SeekResults {
                        t_between_frames: 0.0,
                        index: 0,
                    },
                };
                Some(channel)
            }
            SerializedAnimation3DKeyframeTypes::Invalid => {
                println!("Invalid keyframe types detected.");
                None
            }
        }
    }

    //  Returns the index of the first of two animations
    pub fn seek(&mut self, time: f32, looping: bool) {

        let findings = self.times.binary_search_by(|t| t.total_cmp(&time));

        match findings {
            // We found the number
            Ok(val) => {
                if val >= self.times.len() - 1 && looping {
                    self.seek_results = SeekResults {
                        t_between_frames: 0.0,
                        index: 0,
                    }
                } else if val >= self.times.len() - 1 && !looping {
                    self.seek_results = SeekResults {
                        t_between_frames: 0.0,
                        index: val - 1,
                    };
                } else {
                    self.seek_results = SeekResults {
                        t_between_frames: 0.0,
                        index: val,
                    }
                }
            }
            // We didn't find the specific number
            Err(val) => {
                // println!("Animation Channel: Didn't find index.");
                if val >= self.times.len() - 1 && looping {
                    self.seek_results = SeekResults {
                        t_between_frames: (time - self.times[0]) / (self.times[1] - self.times[0]),
                        index: 0,
                    };
                } else if val >= self.times.len() - 1 && !looping {
                    println!("Found end of animation.");
                    self.seek_results = SeekResults {
                        t_between_frames: 0.0,
                        index: val - 1,
                    };
                } else {
                    self.seek_results = SeekResults {
                        t_between_frames: (time - self.times[val]) / (self.times[val + 1] - self.times[val]),
                        index: val,
                    };
                }
            }
        }
    }
    // }
}
