use crate::{asset_manager::*, scene3d::*, skeletal_context::SkeletalContext, index_types::*};
use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend,
};

pub struct UserContext {
    pub asset_mgr: AssetManager,
    pub skeletals: SkeletalContexts<SkeletalContext>,
    pub scenes: Scenes3D<Scene3D>,
    pub audio_mgr: Option<AudioManager>,
    pub active_scene: Scene3DIndex,
    pub time_elapsed: u128,
}

impl UserContext {
    pub fn new() -> Self {
        let asset_mgr = AssetManager::new();
        let skeletals = SkeletalContexts::new();
        let mut scenes = Scenes3D::new();
        scenes.push(Scene3D::new(&glam::Vec3::new(0.0, -9.86, 0.0)));
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
            active_scene: Scene3DIndex::new(0),
            time_elapsed: 0,
        }
    }
}
