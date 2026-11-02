use std::path::PathBuf;

use crate::{
    asset_manager::*, index_types::*, instance3d::Instance3D, model_material::ModelMaterial,
    scene3d::*,
};
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use noobwerkz_skeletal_animation::animation_graph_definition::AnimationGraphDefinition;

pub struct ClientUserContext {
    pub asset_mgr: AssetManager,
    pub scenes: Scenes3D<Scene3D>,
    pub active_scene: Scene3DIndex,
    pub audio_mgr: Option<AudioManager>,
    pub time_elapsed: u128, // TODO: Replace with web_time::Duration??
}

impl ClientUserContext {
    pub fn new() -> Self {
        let asset_mgr = AssetManager::new();
        let scenes = Scenes3D::new();
        let audio_mgr_res = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default());
        let mut audio_mgr: Option<AudioManager> = None;

        match audio_mgr_res {
            Ok(val) => audio_mgr = Some(val),
            Err(err) => {
                println!("Could not create audio manager. Error: {}", err)
            }
        }
        Self {
            asset_mgr,
            scenes,
            active_scene: Scene3DIndex::new(0),
            audio_mgr,
            time_elapsed: 0,
        }
    }

    pub fn add_scene(&mut self, scene: Scene3D) -> Scene3DIndex {
        self.scenes.push(scene);
        Scene3DIndex::from(self.scenes.len() - 1)
    }

    pub fn load_character(
        &mut self,
        path: &PathBuf,
        name: &str,
        scene_id: Scene3DIndex,
        instances: &Vec<Instance3D>,
        animgraph_definition: &AnimationGraphDefinition,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
        debug_material: &ModelMaterial,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        bone_matrices_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        let character = self.asset_mgr.load_skinned_3d_model_from_file(
            &path,
            name,
            device,
            queue,
            debug_material,
            texture_bind_group_layout,
        );

        match character {
            Ok(val) => {
                self.scenes[scene_id].add_characters(
                    device,
                    bone_matrices_bind_group_layout,
                    val,
                    instances,
                    animgraph_definition,
                    self.asset_mgr.skinned_models[val].animations.clone(),
                    self.asset_mgr.skinned_models[val].skeleton.clone(),
                    &self.asset_mgr.skinned_models[val].animations_idx_by_name,
                    name.to_owned(),
                );
            }
            Err(err) => {
                println!(
                    "Could not load skinned mode for character {}. Error: {}",
                    name, err
                );
            }
        }
    }
}
