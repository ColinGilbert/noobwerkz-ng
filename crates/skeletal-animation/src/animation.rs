use std::sync::Arc;

use noobwerkz_shared::skeleton::Skeleton;

use crate::animation_channel::*;
use crate::operation_result::*;

#[derive(Clone)]

pub struct Animation {
    pub length: f32,
    pub channels: Vec<AnimationChannel>,
    pub looping: bool,
}

impl Animation {
    pub fn new(anim_channels: Vec<AnimationChannel>) -> Self {
        let mut latest_time: f32 = 0.0;
        for c in &anim_channels {
            let last_time = c.times[c.times.len() - 1];
            latest_time = latest_time.max(last_time);
        }
        Self {
            length: latest_time,
            channels: anim_channels,
            looping: false,
        }
    }

    pub fn seek(&mut self, time: f32) {
        let new_time = time % self.length;
        for c in self.channels.iter_mut() {
            c.seek(new_time);
        }
    }

    pub fn playback(&mut self, skeleton: Arc<Skeleton>) -> LocalToLocalResults {
        let mut results = LocalToLocalResults::new();

        // This was important.
        for b in skeleton.bones.to_vec() {
            results.translations.push(b.position);
            results.rotations.push(b.orientation);
            results.scales.push(b.scale);
        }
        // Here we do the actual playback
        // For each animation channel:
        for c in self.channels.iter_mut() {
            // Check to see if we're at the end of the animation's keyframe with looping off. If so, simply set results to be the last keyframe.
            if c.seek_results.end && !self.looping {
                // println!("Playback at the end without looping");
                match &c.data {
                    KeyframeKeys::Translation(val) => {
                        results.translations[c.bone_index] = val[val.len() - 1];
                    }
                    KeyframeKeys::Rotation(val) => {
                        results.rotations[c.bone_index] = val[val.len() - 1];
                    }
                    KeyframeKeys::Scale(val) => {
                        results.scales[c.bone_index] = val[val.len() - 1];
                    }
                }
            }
            // If looping and at the end, go back to beginning
            else if c.seek_results.end && self.looping {
                // println!("Playback at the end with looping");
                match &c.data {
                    KeyframeKeys::Translation(val) => {
                        results.translations[c.bone_index] = val[0];
                    }
                    KeyframeKeys::Rotation(val) => {
                        results.rotations[c.bone_index] = val[0];
                    }
                    KeyframeKeys::Scale(val) => {
                        results.scales[c.bone_index] = val[0];
                    }
                }
            }
            // If at the first keyframe use the first keyframe
            else if c.seek_results.begin {
                // println!("Playback at the first keyframe");
                match &c.data {
                    KeyframeKeys::Translation(val) => {
                        results.translations[c.bone_index] = val[0];
                    }
                    KeyframeKeys::Rotation(val) => {
                        results.rotations[c.bone_index] = val[0];
                    }
                    KeyframeKeys::Scale(val) => {
                        results.scales[c.bone_index] = val[0];
                    }
                }
            }
            // Now we do a blend
            else if c.seek_results.interpolation_needed {
                // println!("Playback blending two frames");
                match &c.data {
                    KeyframeKeys::Translation(val) => {
                        let index = c.seek_results.index;
                        //if val.len() > 1 {
                        results.translations[c.bone_index] = glam::Vec3::lerp(
                            val[index],
                            val[index + 1],
                            c.seek_results.t_between_frames,
                        );
                        //}
                    }
                    KeyframeKeys::Rotation(val) => {
                        let index = c.seek_results.index;
                        //if val.len() > 1 {
                        results.rotations[c.bone_index] = glam::Quat::slerp(
                            val[index],
                            val[index + 1],
                            c.seek_results.t_between_frames,
                        );
                        //}
                    }
                    KeyframeKeys::Scale(val) => {
                        let index = c.seek_results.index;
                        // if val.len() > 1 {
                        results.scales[c.bone_index] = glam::Vec3::lerp(
                            val[index],
                            val[index + 1],
                            c.seek_results.t_between_frames,
                        );
                        // }
                    }
                }
            } else if !c.seek_results.interpolation_needed {
                match &c.data {
                    KeyframeKeys::Translation(val) => {
                        let index = c.seek_results.index;
                        results.translations[c.bone_index] = val[index];
                    }
                    KeyframeKeys::Rotation(val) => {
                        let index = c.seek_results.index;
                        results.rotations[c.bone_index] = val[index];
                    }
                    KeyframeKeys::Scale(val) => {
                        let index = c.seek_results.index;
                        results.scales[c.bone_index] = val[index];
                    }
                }
            } else {
                println!("Unhandled interpolation mode.")
            }
        }
        results
    }

    pub fn reset(&mut self) {
        self.seek(0.0);
    }
}
