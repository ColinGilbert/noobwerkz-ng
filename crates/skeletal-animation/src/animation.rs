use crate::keyframe::Keyframe;

// Consists of several keyframes
#[derive(Clone)]

pub struct Animation {
    pub translation_keys: Vec<glam::Vec3>,
    pub rotation_keys: Vec<glam::Quat>,
    pub scale_keys: Vec<glam::Vec3>,
    pub timepoints: Vec<f32>,
    pub duration: f32,
    pub num_tracks: usize,
}

pub struct AnimationSeekResults {
    pub beginning: bool,
    pub finished: bool,
    pub first: usize,
    pub second: usize,
    pub time_elapsed: f32,
}

impl Animation {
    pub fn new() -> Self {
        Self {
            translation_keys: Vec::new(),
            rotation_keys: Vec::new(),
            scale_keys: Vec::new(),
            timepoints: Vec::new(),
            duration: 0.0,
            num_tracks: 1, // TODO: Change to be more flexible
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.translation_keys.push(keyframe.translation);
        self.rotation_keys.push(keyframe.rotation);
        self.scale_keys.push(keyframe.scale);
        self.timepoints.push(keyframe.time);
        self.duration += keyframe.time;
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
        while i < self.timepoints.len() {
            let i2 = i + 1;
            if i2 < self.timepoints.len() {
                let t = self.timepoints[i];
                let t2 = self.timepoints[i2];
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
            } else if i == self.timepoints.len() - 1 {
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
            first: self.timepoints.len() - 1,
            second: self.timepoints.len() - 1,
            time_elapsed: 0.0,
        }
    }
}
