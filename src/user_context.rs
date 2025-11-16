//use std::sync::*;
use crate::{
    asset_manager::*, scene::*, skeletal_context::SkeletalContext
};

pub struct UserContext {
    pub asset_mgr: AssetManager, 
    pub skeletals: Vec<SkeletalContext>,
    pub scenes: Vec<Scene>,
    pub active_scene: usize,
    pub animation_time_elapsed: u128,
}

impl UserContext {
    pub fn new() -> Self {
        let asset_mgr = AssetManager::new();
        let skeletals = Vec::new();
        let scenes = Vec::<Scene>::new();
        
        Self {
            asset_mgr,
            skeletals,
            scenes,
            active_scene: 0,
            animation_time_elapsed: 0,
        }
    }
}