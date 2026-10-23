// Each channel covers a single transform (either translation, rotation, or scale) for a single bone.
// This is because each bone may have a different set of keyframes.

use noobwerkz_shared::serialized_model3d::*;
use crate::{seek_results::*};

#[derive(Clone)]
pub enum KeyframeKeys {
    Translation(Vec<glam::Vec3>),
    Rotation(Vec<glam::Quat>),
    Scale(Vec<glam::Vec3>),
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
    pub fn from_serialized(serialized: &SerializedAnimation3D) -> Vec<Self> {
        let mut results: Vec<Self> = Vec::new();

        for serialized_channel in &serialized.keyframes {
            let bone_index = serialized_channel.bone;

            match &serialized_channel.data {
                SerializedAnimation3DKeyframeTypes::Translations(val) => {
                    let mut translation_keys = Vec::new();
                    for t in val {
                        translation_keys.push(glam::Vec3::from_array(*t));
                    }
                    let channel = Self {
                        bone_index,
                        data: KeyframeKeys::Translation(translation_keys),
                        times: serialized_channel.times.clone(),
                        seek_results: SeekResults {
                            end: false,
                            begin: true,
                            interpolation_needed: false,
                            t_between_frames: 0.0,
                            index: 0,
                        },
                    };
                    results.push(channel);
                }

                SerializedAnimation3DKeyframeTypes::Rotations(val) => {
                    let mut rotation_keys = Vec::new();
                    for r in val {
                        rotation_keys.push(glam::Quat::from_array(*r));
                    }
                    let channel = Self {
                        bone_index,
                        data: KeyframeKeys::Rotation(rotation_keys),
                        times: serialized_channel.times.clone(),
                        seek_results: SeekResults {
                            end: false,
                            begin: true,
                            interpolation_needed: false,
                            t_between_frames: 0.0,
                            index: 0,
                        },
                    };
                    results.push(channel);
                }

                SerializedAnimation3DKeyframeTypes::Scales(val) => {
                    let mut scale_keys = Vec::new();
                    for s in val {
                        scale_keys.push(glam::Vec3::from_array(*s));
                    }
                    let channel = Self {
                        bone_index,
                        data: KeyframeKeys::Scale(scale_keys),
                        times: serialized_channel.times.clone(),
                        seek_results: SeekResults {
                            end: false,
                            begin: true,
                            interpolation_needed: false,
                            t_between_frames: 0.0,
                            index: 0,
                        },
                    };
                    results.push(channel);
                }
                SerializedAnimation3DKeyframeTypes::Invalid => {
                    println!("Invalid keyframe types detected.")
                }
            }
        }
        results
    }

    //  Returns the index of the first of two animations
    pub fn seek(&mut self, time: f32) {
        if time == 0.0 {
            self.seek_results = SeekResults {
                begin: true,
                end: false,
                interpolation_needed: false,
                t_between_frames: 0.0,
                index: 0,
            };
        }
        let findings = self.times.binary_search_by(|t| t.total_cmp(&time));
        match findings {
            // We found the number
            Ok(val) => {
                if val == self.times.len() - 1 {
                    self.seek_results = SeekResults {
                        end: true,
                        begin: false,
                        interpolation_needed: false,
                        t_between_frames: 0.0,
                        index: val,
                    }
                } else {
                    self.seek_results = SeekResults {
                        end: false,
                        begin: false,
                        interpolation_needed: false,
                        t_between_frames: 0.0,
                        index: val,
                    };
                }
            }
            // We didn't find the specific number
            Err(val) => {
                let t_between_frames = 1.0 - (self.times[val] - time) / self.times[val];
                self.seek_results = SeekResults {
                    end: false,
                    begin: false,
                    interpolation_needed: true,
                    t_between_frames,
                    index: val,
                };
            }
        }
    }
}
