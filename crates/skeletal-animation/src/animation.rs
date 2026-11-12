use std::sync::Arc;

use noobwerkz_shared::skeleton::Skeleton;

use crate::animation_channel::*;
use crate::operation_result::*;

pub struct AnimationSeek {
    pub current_time: f32,
}

impl AnimationSeek {
    pub fn reset(&mut self, animation: Arc<Animation>) {
        self.seek(animation, 0.0);
    }
    
    pub fn seek(&mut self, animation: Arc<Animation>, time: f32) {
        if animation.looping {
            self.current_time = time % animation.length;
        } else {
            self.current_time = time
        }
    }
}

#[derive(Clone)]

pub struct Animation {
    pub length: f32,
    pub channels: Vec<AnimationChannel>,
    pub looping: bool,
}

impl Animation {
    pub fn new(anim_channels: &Vec<AnimationChannel>, looping: bool) -> Self {
        let mut latest_time: f32 = 0.0;
        for c in anim_channels {
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
            channels: anim_channels.clone(),
            looping,
        }
    }

    pub fn playback(
        &self,
        skeleton: Arc<Skeleton>,
        results: &mut LocalToLocalResults,
        current_time: f32,
    ) {
        results.translations.clear();
        results.rotations.clear();
        results.scales.clear();

        // This was important.
        for b in skeleton.bones.to_vec() {
            results.translations.push(b.position);
            results.rotations.push(b.orientation);
            results.scales.push(b.scale);
        }
        // Here we do the actual playback
        for c in &self.channels {
            let k = c.seek(current_time);
            results.translations[c.bone_index] = k.translation;
            results.rotations[c.bone_index] = k.orientation;
            results.scales[c.bone_index] = k.scale;
        }
    }
}
