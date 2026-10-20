use crate::keyframe::Keyframe;
use noobwerkz_shared::serialized_model3d::*;

// Consists of several keyframes
#[derive(Clone)]
pub struct AnimationChannel {
    pub translation_keys: Vec<glam::Vec3>,
    pub rotation_keys: Vec<glam::Quat>,
    pub scale_keys: Vec<glam::Vec3>,
    pub times: Vec<f32>,
    pub looping: bool,
}

pub struct AnimationSeekResults {
    pub beginning: bool,
    pub finished: bool,
    pub first: usize,
    pub second: usize,
    pub time_elapsed: f32,
}

impl AnimationChannel {
    pub fn from_serialized(serialized: &SerializedModel3D) -> Vec<Self> {
        let mut results: Vec<Self> = Vec::new();

        for serialized_channel in &serialized.animations_by_channel {
            let mut translation_keys = Vec::new();
            for t in &serialized_channel.translations {
                translation_keys.push(glam::Vec3::from_array(*t));
            }
            let mut rotation_keys = Vec::new();
            for r in &serialized_channel.rotations {
                rotation_keys.push(glam::Quat::from_array(*r));
            }
            let mut scale_keys = Vec::new();
            for s in &serialized_channel.scales {
                scale_keys.push(glam::Vec3::from_array(*s));
            }

            let channel = Self {
                translation_keys,
                rotation_keys,
                scale_keys,
                times: serialized_channel.times.clone(),
                looping: false,
            };
            results.push(channel);
        }
        results
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.translation_keys.push(keyframe.translation);
        self.rotation_keys.push(keyframe.rotation);
        self.scale_keys.push(keyframe.scale);
        self.times.push(keyframe.time);
    }

    //  Returns the index of the first of two animations
    pub fn seek(&self, time: f32, start: Option<usize>) -> AnimationSeekResults {
        let mut i;
        match start {
            Some(val) => i = val,
            None => i = 0,
        };
        if time == 0.0 {
            let results = AnimationSeekResults {
                beginning: true,
                finished: false,
                first: 0,
                second: 1,
                time_elapsed: 0.0,
            };
            return results;
        }
        while i < self.times.len() {
            let i2 = i + 1;
            if i2 < self.times.len() {
                let t = self.times[i];
                let t2 = self.times[i2];
                if time >= t && time <= t2 {
                    let results: AnimationSeekResults = AnimationSeekResults {
                        beginning: false,
                        finished: false,
                        first: i,
                        second: i2,
                        time_elapsed: time - t,
                    };
                    return results;
                }
            } else if i == self.times.len() - 1 {
                let results: AnimationSeekResults = AnimationSeekResults {
                    beginning: false,
                    finished: true,
                    first: i,
                    second: i,
                    time_elapsed: 0.0,
                };
                return results;
            }
            i += 1;
        }

        AnimationSeekResults {
            beginning: false,
            finished: true,
            first: self.times.len() - 1,
            second: self.times.len() - 1,
            time_elapsed: 0.0,
        }
    }
}
