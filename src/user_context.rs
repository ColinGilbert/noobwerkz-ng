//use std::sync::*;
use crate::{
    asset_manager::AssetManager, model::*, scene::*, skeletal_context::SkeletalContext, skinned_model::*
};

pub struct UserContext {
    pub models: Vec<Model>,
    pub skinned_models: Vec<SkinnedModel>,
    pub skeletals: Vec<SkeletalContext>,
    pub scenes: Vec<Scene>,
    pub active_scene: usize,
    pub asset_mgr: AssetManager,
    pub animation_time_elapsed: u128,
}

impl UserContext {
    pub fn new() -> Self {
        let models = Vec::new();
        let skinned_models = Vec::new();
        let skeletals = Vec::new();
        let scenes = Vec::<Scene>::new();
        
        Self {
            models,
            skinned_models,
            skeletals,
            scenes,
            active_scene: 0,
            asset_mgr: AssetManager::new(),
            animation_time_elapsed: 0,
        }
    }
}