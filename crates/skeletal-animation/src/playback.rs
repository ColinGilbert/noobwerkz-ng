use crate::{
    MAX_BONES, animation_channel::AnimationChannel, bone::*, operation_result::OperationResults,
    skeleton::Skeleton,
};
use std::sync::Arc;

pub struct PlaybackContext {
    pub skeleton: Arc<Skeleton>,
    pub animation: Arc<AnimationChannel>,
    pub timepoint_index: usize,
    pub finished: bool,
}

impl PlaybackContext {
    pub fn new(skeleton: Arc<Skeleton>, animation: Arc<AnimationChannel>) -> Self {
        Self {
            skeleton: skeleton.clone(),
            animation: animation.clone(),
            timepoint_index: 0,
            finished: false,
        }
    }

    pub fn set_time(&mut self, time: f32) {
        let results = self.animation.seek(time, None);
        self.timepoint_index = results.first;
        self.finished = results.finished;
    }

    pub fn playback(&mut self, duration: f32) -> OperationResults {
        let mut results = OperationResults::new();
        // Here we do the actual playback
        // Check to see if we're at the end of the animation's keyframes. If so, simply set the output to be the last keyframe.
        
        // Now, we blend between the two

        results
    }

    pub fn reset(&mut self) {

    }
}
