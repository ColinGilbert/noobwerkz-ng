use std::sync::*;

use crate::{
    model::{Model, SkinnedModel},
    scene::Scene,
};

pub struct UserContext {
    pub models: Vec<Model>,
    pub skinned_models: Vec<SkinnedModel>,
    pub scenes: Vec<Scene>,
    pub active_scene: usize,
}

impl UserContext {
    pub fn new() -> Self {
        let models = Vec::<Model>::new();
        let skinned_models = Vec::<SkinnedModel>::new();
        let scenes = Vec::<Scene>::new();
        Self {
            models,
            skinned_models,
            scenes,
            active_scene: 0,
        }
    }
}

#[allow(unused)]
type SharedContext = Arc<Mutex<UserContext>>;

#[allow(unused)]
pub static USER_CONTEXT: LazyLock<SharedContext> = LazyLock::new(|| {
    let ctx = UserContext::new();
    Arc::new(Mutex::new(ctx))
});
