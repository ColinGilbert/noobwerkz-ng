//use std::sync::*;
use crate::{
    asset_manager::*, scene::*, skeletal_context::SkeletalContext, ui::{NullUIControls, UIControls}
};

pub struct UserContext {
    pub asset_mgr: AssetManager, 
    pub skeletals: Vec<SkeletalContext>,
    pub scenes: Vec<Scene>,
    pub active_scene: usize,
    pub ui_controls: Box<dyn UIControls>,
    pub animation_time_elapsed: u128,
}

impl UserContext {
    pub fn new() -> Self {
        let asset_mgr = AssetManager::new();
        let skeletals = Vec::new();
        let scenes = Vec::<Scene>::new();
        let ui_controls = Box::<NullUIControls>::new(NullUIControls {});
        Self {
            asset_mgr,
            skeletals,
            scenes,
            ui_controls,
            active_scene: 0,
            animation_time_elapsed: 0,
        }
    }
}