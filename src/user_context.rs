//use std::sync::*;
use crate::{asset_manager::*, scene::*, skeletal_context::SkeletalContext};
use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend,
};

pub struct UserContext {
    pub asset_mgr: AssetManager,
    pub skeletals: Vec<SkeletalContext>,
    pub scenes: Vec<Scene>,
    pub audio_mgr: Option<AudioManager>,
    pub active_scene: usize,
    pub time_elapsed: u128,
}

impl UserContext {
    pub fn new() -> Self {
        let asset_mgr = AssetManager::new();
        let skeletals = Vec::new();
        let scenes = Vec::<Scene>::new();
        let audio_mgr_res =  AudioManager::<DefaultBackend>::new(AudioManagerSettings::default());
        let mut audio_mgr: Option<AudioManager> = None;

        match audio_mgr_res {
            Ok(val) => { audio_mgr = Some(val) }
            Err(err) => { println!("Could not create audio manager. Error: {}", err) }
        }
        
        Self {
            asset_mgr,
            skeletals,
            scenes,
            audio_mgr,
            active_scene: 0,
            time_elapsed: 0,
        }
    }
}
