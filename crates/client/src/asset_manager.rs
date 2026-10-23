// This code serves as a simplified way for asset loading.
use crate::{
    index_types::*, material::Material, model3d::*, resource::*, skinned_model3d::*, texture::*,
};
use anyhow::*;
use kira::sound::static_sound::StaticSoundData;
use std::{collections::*, path::*, result::Result::*};

pub struct AssetManager {
    pub models_by_name: HashMap<String, Model3DIndex>,
    pub skinned_models_by_name: HashMap<String, SkinnedModel3DIndex>,
    pub textures_by_name: HashMap<String, TextureIndex>,
    pub audio_clips_by_name: HashMap<String, AudioClipIndex>,

    pub model_names: HashMap<Model3DIndex, String>,
    pub skinned_model_names: HashMap<SkinnedModel3DIndex, String>,
    pub texture_names: HashMap<TextureIndex, String>,
    pub audio_clip_names: HashMap<AudioClipIndex, String>,

    pub models: Models3D<Model3D>,
    pub skinned_models: SkinnedModels3D<SkinnedModel3D>,
    pub textures: Textures<Texture>,
    pub audio_clips: AudioClips<StaticSoundData>, // pub skeletons: Vec<Arc<ozz_animation_rs::Skeleton>>,
                                                  // pub animations: Vec<Arc<ozz_animation_rs::Animation>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            models_by_name: HashMap::new(),
            skinned_models_by_name: HashMap::new(),
            textures_by_name: HashMap::new(),
            audio_clips_by_name: HashMap::new(),
            model_names: HashMap::new(),
            skinned_model_names: HashMap::new(),
            texture_names: HashMap::new(),
            audio_clip_names: HashMap::new(),
            models: Models3D::new(),
            skinned_models: SkinnedModels3D::<SkinnedModel3D>::new(),
            textures: Textures::new(),
            audio_clips: AudioClips::new(),
            // skeletons: Vec::<Arc<ozz_animation_rs::Skeleton>>::new(),
            // animations: Vec::<Arc<ozz_animation_rs::Animation>>::new(),
        }
    }

    pub fn load_3d_model_from_file(
        &mut self,
        filepath: &std::path::PathBuf,
        name: &str,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
        default_material: &Material,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<Model3DIndex> {
        let mut serialized_opt = load_serialized_model(filepath.as_path());

        match serialized_opt {
            Some(serialized) => {
                let mut path = filepath.clone();
                path.pop();
                let model = load_3d_model_from_serialized(
                    &mut serialized,
                    default_material,
                    &path.as_path(),
                    device,
                    queue,
                    texture_layout,
                );
                match model {
                    Some(val) => {
                        self.models.push(val);
                        let idx = Model3DIndex::new(self.models.len() - 1);
                        self.model_names.insert(idx, name.to_owned());
                        self.models_by_name.insert(name.to_owned(), idx);
                        return anyhow::Result::Ok(idx);
                    }
                    None => {
                        return anyhow::Result::Err(anyhow!(format!(
                            "Could not load model {} from file",
                            name
                        )));
                    }
                }
            }
            None => Err(anyhow!("AssetManager: Could not load serialized 3D model")),
        }
    }

    pub fn load_skinned_3d_model_from_file(
        &mut self,
        filepath: &std::path::PathBuf,
        name: &str,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
        default_material: &Material,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> anyhow::Result<SkinnedModel3DIndex> {
        let mut serialized = load_serialized_model(filepath.as_path());
        let mut path = filepath.clone();
        path.pop();
        let model = load_skinned_3d_model_from_serialized(
            &mut serialized,
            default_material,
            &path.as_path(),
            device,
            queue,
            texture_layout,
        );
        match model {
            Some(val) => {
                self.skinned_models.push(val);
                let idx = SkinnedModel3DIndex::new(self.skinned_models.len() - 1);
                self.skinned_model_names.insert(idx, name.to_owned());
                self.skinned_models_by_name.insert(name.to_owned(), idx);
                return anyhow::Result::Ok(idx);
            }
            None => {
                return anyhow::Result::Err(anyhow!(format!(
                    "Could not load skinned model {} from file",
                    name
                )));
            }
        }
    }

    // pub fn load_texture_from_file(
    //     &mut self,
    //     filepath: &Path,
    //     name: &str,
    //     is_normal_map: bool,
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    // ) -> anyhow::Result<usize> {
    //     let tex = futures::executor::block_on(load_texture(filepath, is_normal_map, device, queue));
    //     match tex {
    //         anyhow::Result::Ok(val) => {
    //             self.textures.push(val);
    //             let idx = self.textures.len() - 1;
    //             self.texture_names.insert(idx, name.to_owned());
    //             self.textures_by_name.insert(name.to_owned(), idx);
    //             return anyhow::Result::Ok(idx);
    //         }
    //         Err(msg) => {
    //             return anyhow::Result::Err(anyhow!(format!(
    //                 "Could not load texture file {}",
    //                 msg
    //             )));
    //         }
    //     }
    // }

    pub fn load_audio_clip_from_file(
        &mut self,
        filepath: &Path,
        name: &str,
    ) -> anyhow::Result<AudioClipIndex> {
        let audio_bytes = StaticSoundData::from_file(filepath);
        match audio_bytes {
            anyhow::Result::Ok(val) => {
                self.audio_clips.push(val);
                let idx = AudioClipIndex::new(self.audio_clips.len() - 1);
                self.audio_clip_names.insert(idx, name.to_owned());
                self.audio_clips_by_name.insert(name.to_owned(), idx);
                return anyhow::Result::Ok(idx);
            }
            Err(msg) => {
                return anyhow::Result::Err(anyhow!(format!("Could not load audio file {}", msg)));
            }
        }
    }

    // pub fn load_skeleton_from_file(
    //     &mut self,
    //     filepath: &std::path::Path,
    //     name: &str,
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    // ) {
    // }

    // pub fn load_animation_from_file(
    //     &mut self,
    //     filepath: &std::path::Path,
    //     name: &str,
    //     device: &wgpu::Device,
    //     queue: &wgpu::Queue,
    // ) {
    // }
}
