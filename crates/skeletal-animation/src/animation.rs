use std::sync::Arc;

use noobwerkz_shared::skeleton::Skeleton;

use crate::animation_channel::*;
use crate::operation_result::*;

#[derive(Clone)]

pub struct Animation {
    pub length: f32,
    pub channels: Vec<AnimationChannel>,
    pub current_time: f32,
    pub looping: bool,
    
}

impl Animation {
    pub fn new(anim_channels: Vec<AnimationChannel>, looping: bool) -> Self {
        let mut latest_time: f32 = 0.0;
        for c in &anim_channels {
            let mut last_t = 0.0;
            for (i, k) in c.keyframes.iter().enumerate() {
                if k.time < last_t {
                    println!("Animation channel times out of order, index: {}", i);
                }
                last_t = k.time;
            }
            let last_time = c.keyframes[c.keyframes.len() - 1].time;
            latest_time = latest_time.max(last_time);
        }

        Self {
            length: latest_time,
            channels: anim_channels,
            current_time: 0.0,
            looping,
        }
    }

    pub fn seek(&mut self, time: f32) {
        if self.looping {
            self.current_time = time % self.length;
        }
        
        // for c in self.channels.iter_mut() {
        //     c.seek(new_time, self.looping);
        // }
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
        for c in &self.channels {
            let k = c.seek(self.current_time, self.looping);
            results.translations[c.bone_index] = k.translation;
            results.rotations[c.bone_index] = k.orientation;
            results.scales[c.bone_index] = k.scale;
            // let t = c.seek_results.t_between_frames;
            // let index = c.seek_results.index;
            // results.translations[c.bone_index] = c.keyframes[index]
            //     .translation
            //     .lerp(c.keyframes[index + 1].translation, t);
            // results.rotations[c.bone_index] = c.keyframes[index]
            //     .orientation
            //     .slerp(c.keyframes[index + 1].orientation, t);
            // results.scales[c.bone_index] = c.keyframes[index]
            //     .scale
            //     .lerp(c.keyframes[index + 1].scale, t);
        }

        results
    }

    pub fn reset(&mut self) {
        self.seek(0.0);
    }
}
