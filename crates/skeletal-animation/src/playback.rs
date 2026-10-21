use crate::{
    animation_channel::{AnimationChannel, SeekResults},
    operation_result::OperationResults,
    skeleton::Skeleton,
};
use std::sync::Arc;

pub struct PlaybackContext {
    pub skeleton: Arc<Skeleton>,
    pub animation_channels: Vec<AnimationChannel>,
    pub finished: bool,
    pub looping: bool,
}

impl PlaybackContext {
    pub fn new(skeleton: Arc<Skeleton>, animation: Vec<AnimationChannel>) -> Self {
        Self {
            skeleton: skeleton.clone(),
            animation_channels: animation,
            finished: false,
            looping: true,
        }
    }

    pub fn seek(&mut self, time: f32) {
        // let mut temp_finished = true;
        for c in self.animation_channels.iter_mut() {
           c.seek(time);
        }
    }

    pub fn playback(&mut self) -> OperationResults {
        let mut results = OperationResults::new();
        // Here we do the actual playback

        // For each animation channel:
        for c in self.animation_channels.iter() {
            //      Check to see if we're at the end of the animation's keyframes. If so, simply set results to be the last keyframe.
            if c.seek_results.end && !self.looping {
                results.translations[c.bone_index] = c.translation_keys[c.translation_keys.len() - 1];
                results.rotations[c.bone_index] = c.rotation_keys[c.rotation_keys.len() - 1];
                results.scales[c.bone_index] = c.scale_keys[c.scale_keys.len() - 1];
            }
            //      If looping and at the end, go back to beginning
            else if c.seek_results.end && self.looping {
                results.translations[c.bone_index] = c.translation_keys[0];
                results.rotations[c.bone_index] = c.rotation_keys[0];
                results.scales[c.bone_index] = c.scale_keys[0];
            }
            //      If at the first keyframe, blend between original positions and first keyframe
            else if c.seek_results.begin {}
            //      For all other cases, we blend between the two sequential keyframes
            else c.seek_results {}
        }
        results
    }

    pub fn reset(&mut self) {
        self.seek(0.0);
        self.finished = false;
    }
}
