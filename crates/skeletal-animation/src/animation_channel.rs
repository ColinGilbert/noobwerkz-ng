use noobwerkz_shared::serialized_model3d::*;

// Consists of several keyframes
#[derive(Clone)]
pub struct AnimationChannel {
    pub bone: usize,
    pub translation_keys: Vec<glam::Vec3>,
    pub rotation_keys: Vec<glam::Quat>,
    pub scale_keys: Vec<glam::Vec3>,
    pub times: Vec<f32>,
    pub looping: bool,
}

pub struct SeekResults {
    pub end: bool,
    pub begin: bool,
    pub interpolation_needed: bool,
    pub index: usize,
}

impl AnimationChannel {
    pub fn from_serialized(serialized: &SerializedModel3D) -> Vec<Self> {
        let mut results: Vec<Self> = Vec::new();

        for serialized_channel in &serialized.animations_channels {
            let bone = serialized_channel.bone;
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
                bone,
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

    //  Returns the index of the first of two animations
    pub fn seek(&self, time: f32) -> SeekResults {
        if time == 0.0 {
            return SeekResults {
                begin: true,
                end: false,
                interpolation_needed: false,
                index: 0,
            };
        }
        let mut seek_results: SeekResults;
        let findings = self.times.binary_search_by(|t| t.total_cmp(&time));
        match findings {
            // We found the number
            Ok(val) => {
                if val == self.times.len() - 1 {
                    seek_results = SeekResults {
                        end: true,
                        begin: false,
                        interpolation_needed: false,
                        index: val,
                    }
                }
                else {
                    seek_results = SeekResults { end:  false, begin: false, interpolation_needed: true, index: val };
                }
            }
            // We didn't find the specific number
            Err(val) => {
                seek_results = SeekResults { end: false, begin: false, interpolation_needed: true, index: val };
            }
    }

        seek_results
    }
}
