use std::sync::Arc;

use noobwerkz_shared::skeleton::Skeleton;

use crate::animation_channel::*;
use crate::operation_result::*;

#[derive(Clone)]

pub struct Animation {
    pub length: f32,
    pub seek: f32,
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
        // println!("Latest anim time: {}", latest_time);
        Self {
            length: latest_time,
            seek: 0.0,
            channels: anim_channels,
            looping: false,
        }
    }

    pub fn seek(&mut self, time: f32) {
        let new_time = time % self.length;
        // let mut temp_finished = true;
        for c in self.channels.iter_mut() {
            c.seek(new_time);
        }
    }

    pub fn playback(&mut self, skeleton: Arc<Skeleton>) -> LocalToLocalResults {
        // println!("Animations num_bones: {}", self.num_bones);
        let mut results = LocalToLocalResults::new();
        results
            .translations
            .resize(skeleton.num_bones, glam::Vec3::ZERO);
        results
            .rotations
            .resize(skeleton.num_bones, glam::Quat::IDENTITY);
        results
            .scales
            .resize(skeleton.num_bones, glam::Vec3::splat(1.0));

        for i in 0..skeleton.num_bones {
            results.translations[i] = skeleton.bones[i].position;
            results.rotations[i] = skeleton.bones[i].orientation;
            results.scales[i] = skeleton.bones[i].scale;
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
            // For all other cases, we blend between two sequential keyframes
            else {
                // println!("Playback blending two frames");
                match &c.data {
                    KeyframeKeys::Translation(val) => {
                        let index: usize;
                        index = c.seek_results.index;
                        // if c.seek_results.index == val.len() {
                        //     index = c.seek_results.index - 1;
                        // } else {
                        //     index = c.seek_results.index;
                        // }
                        if val.len() > 1 && index < val.len() - 1 {
                            results.translations[c.bone_index] = glam::Vec3::lerp(
                                val[index],
                                val[index + 1],
                                c.seek_results.t_between_frames,
                            );
                        } else {
                            results.translations[c.bone_index] = val[index];
                        }
                    }
                    KeyframeKeys::Rotation(val) => {
                        let index: usize;
                        index = c.seek_results.index;

                        // if c.seek_results.index == val.len() {
                        //     index = c.seek_results.index - 1;
                        // } else {
                        //     index = c.seek_results.index;
                        // }
                        if val.len() > 1 && index < val.len() - 1 {
                            results.rotations[c.bone_index] = glam::Quat::slerp(
                                val[index],
                                val[index + 1],
                                c.seek_results.t_between_frames,
                            );
                        } 
                        else {
                            results.rotations[c.bone_index] = val[index];
                        }
                    }
                    KeyframeKeys::Scale(val) => {
                        let index: usize;
                        index = c.seek_results.index;

                        // if c.seek_results.index == val.len() {
                        //     index = c.seek_results.index - 1;
                        // } else {
                        //     index = c.seek_results.index;
                        // }
                        if val.len() > 1 && index < val.len() - 1 {
                            results.scales[c.bone_index] = glam::Vec3::lerp(
                                val[index],
                                val[index + 1],
                                c.seek_results.t_between_frames,
                            );
                        } else {
                            results.scales[c.bone_index] = val[index];
                        }
                    }
                }
            }
        }
        results
    }

    pub fn reset(&mut self) {
        self.seek(0.0);
    }
}
