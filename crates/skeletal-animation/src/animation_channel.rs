// Each channel covers a single transform (either translation, rotation, or scale) for a single bone.
// This is because each bone may have a different set of keyframes.

use std::sync::Arc;

use noobwerkz_shared::{serialized_model3d::*, skeleton::Skeleton};

#[derive(Clone)]
pub enum KeyframeKeys {
    Translation(Vec<glam::Vec3>),
    Rotation(Vec<glam::Quat>),
    Scale(Vec<glam::Vec3>),
}

#[derive(Clone)]
pub struct Keyframe {
    pub translation: glam::Vec3,
    pub orientation: glam::Quat,
    pub scale: glam::Vec3,
    pub time: f32,
}

impl Keyframe {
    pub fn new() -> Self {
        Self {
            translation: glam::Vec3::splat(0.0),
            orientation: glam::Quat::IDENTITY,
            scale: glam::Vec3::splat(1.0),
            time: 0.0,
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
    pub keyframes: Vec<Keyframe>,
    pub seek_results: SeekResults,
}

impl AnimationChannel {
    pub fn from_serialized(
        skeleton: Arc<Skeleton>,
        serialized_keyframes: &Vec<SerializedAnimation3DKeyframeTypes>,
        times: &Vec<f32>,
        bone_index: usize,
    ) -> Option<Self> {
        let bone = skeleton.bones[bone_index];

        let mut translation_keys = vec![bone.position; times.len()];
        let mut rotation_keys = vec![bone.orientation; times.len()];
        let mut scale_keys = vec![bone.scale; times.len()];
        let mut keyframes = Vec::new();
        for serialized in serialized_keyframes {
            match serialized {
                SerializedAnimation3DKeyframeTypes::Translations(val) => {
                    for (i, t) in val.iter().enumerate() {
                        translation_keys[i] = glam::Vec3::from_array(*t);
                    }
                }
                SerializedAnimation3DKeyframeTypes::Rotations(val) => {
                    for (i, r) in val.iter().enumerate() {
                        rotation_keys[i] = glam::Quat::from_array(*r);
                    }
                }
                SerializedAnimation3DKeyframeTypes::Scales(val) => {
                    for (i, s) in val.iter().enumerate() {
                        scale_keys[i] = glam::Vec3::from_array(*s);
                    }
                }
                SerializedAnimation3DKeyframeTypes::Invalid => {
                    println!("Invalid keyframe types detected.");
                    return None;
                }
            }

            // if translation_keys.len() != rotation_keys.len()
            //     || rotation_keys.len() != scale_keys.len()
            // {
            //     println!(
            //         "Mismatch in keyframe lengths: T: {}, R: {}, S: {}, times: {}",
            //         translation_keys.len(),
            //         rotation_keys.len(),
            //         scale_keys.len(),
            //         times.len()
            //     );
            // }
        }

        for (i, t) in times.iter().enumerate() {
            let mut k = Keyframe::new();
            k.translation = translation_keys[i];
            k.orientation = rotation_keys[i];
            k.scale = scale_keys[i];
            k.time = *t;
            keyframes.push(k);
        }

        keyframes.sort_by(|a, b| {
            a.time
                .partial_cmp(&b.time)
                .expect("Got a NaN while comparing keyframes")
        });

        let channel = Self {
            bone_index,
            keyframes,
            seek_results: SeekResults {
                t_between_frames: 0.0,
                index: 0,
            },
        };
        // println!("Keyframes:");
        // for k in &channel.keyframes {
        //     print!("{} ", k.time);
        // }
        Some(channel)
    }

    //  Returns the index of the first of two animations
    pub fn seek(&self, time: f32, looping: bool) -> Keyframe {
        if self.keyframes.is_empty() {
            return Keyframe {
                translation: glam::Vec3::ZERO,
                orientation: glam::Quat::IDENTITY,
                scale: glam::Vec3::splat(1.0),
                time: 0.0,
            };
        }
        if time <= self.keyframes.first().unwrap().time {
            return self.keyframes.first().unwrap().clone();
        }
        if time >= self.keyframes.last().unwrap().time {
            if looping {
                return self.keyframes.first().unwrap().clone();
            }
            return self.keyframes.last().unwrap().clone();
        }

        let mut i = 0;
        while i < self.keyframes.len() - 1 {
            let k1 = &self.keyframes[i];
            let k2 = &self.keyframes[i + 1];
            if time >= k1.time && time <= k2.time {
                let alpha = (time - k1.time) / (k2.time - k1.time);
                let interp_translation = k1.translation.lerp(k2.translation, alpha);
                let interp_orientation = k1.orientation.slerp(k2.orientation, alpha);
                let interp_scale = k1.scale.lerp(k2.scale, alpha);
                return Keyframe {
                    translation: interp_translation,
                    orientation: interp_orientation,
                    scale: interp_scale,
                    time,
                };
            }
            i += 1;
        }
        return self.keyframes.last().unwrap().clone();
    }
    // }
}
