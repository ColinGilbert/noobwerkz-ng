use noobwerkz_shared::serialized_model3d::*;


#[derive(clone)
pub enum KeyframeKeys {
    Translation(Vec<glam::Vec3>),
    Rotation(Vec<glam::Quat>),
    Scale(Vec<glam::Vec3>)
}

// Consists of several keyframes
#[derive(Clone)]
pub struct AnimationChannel {
    pub bone_index: usize,
    pub keys: KeyframeKeys,
    pub times: Vec<f32>,
    pub looping: bool,
    pub seek_results: SeekResults,
}

#[derive(Clone)]
pub struct SeekResults {
    pub end: bool,
    pub begin: bool,
    pub interpolation_needed: bool,
    pub t: f32,
    pub index: usize,
}

impl AnimationChannel {
    pub fn from_serialized(serialized: &SerializedModel3D) -> Vec<Self> {
        let mut results: Vec<Self> = Vec::new();

        for serialized_channel in &serialized.animations_channels {
            let bone_index = serialized_channel.bone;
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
                bone_index,
                translation_keys,
                rotation_keys,
                scale_keys,
                times: serialized_channel.times.clone(),
                looping: false,
                seek_results: SeekResults { end: false, begin: true, interpolation_needed: false, t: 0.0, index: 0 }
            };
            results.push(channel);
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
                t: 0.0,
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
                        t: 0.0,
                        index: val,
                    }
                }
                else {
                   self. seek_results = SeekResults { end:  false, begin: false, interpolation_needed: false, t: 0.0, index: val };
                }
            }
            // We didn't find the specific number
            Err(val) => {
                let t = 1.0 - (self.times[val] - time) / self.times[val];
                self.seek_results = SeekResults { end: false, begin: false, interpolation_needed: true, t, index: val };
            }
    }

    }
}
