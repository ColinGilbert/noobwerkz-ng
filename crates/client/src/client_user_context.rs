use crate::{asset_manager::*, scene3d::*, skeletal3d_context::Skeletal3DContext, index_types::*};
use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend,
};

pub struct ClientUserContext {
    pub asset_mgr: AssetManager,
    pub skeletals: Skeletal3DContexts<Skeletal3DContext>,
    pub scenes: Scenes3D<Scene3D>,
    pub active_scene: Scene3DIndex,
    pub audio_mgr: Option<AudioManager>,
    pub time_elapsed: u128, // TODO: Replace with web_time::Duration??
}

impl ClientUserContext {
    pub fn new() -> Self {
        let asset_mgr = AssetManager::new();
        let skeletals = Skeletal3DContexts::new();
        let scenes = Scenes3D::new();
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
            active_scene: Scene3DIndex::new(0),
            audio_mgr,
            time_elapsed: 0,
        }
    }
}
