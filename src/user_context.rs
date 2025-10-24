use std::sync::*;

use crate::{normal_mapped_model::NormalMappedModel, scene::Scene};

pub struct UserContext {
    pub models: Vec<NormalMappedModel>,
    pub scenes: Vec<Scene>,
    pub active_scene: usize,
}

impl UserContext {
    pub fn new() -> Self {
        let models = Vec::<NormalMappedModel>::new();
        let scenes = Vec::<Scene>::new();
        Self { models, scenes, active_scene: 0 }
    }
}

#[allow(unused)]
type SharedContext = Arc<Mutex<UserContext>>;

#[allow(unused)]
pub static USER_CONTEXT: LazyLock<SharedContext> = LazyLock::new(|| {
    let ctx = UserContext::new();
    Arc::new(Mutex::new(ctx))
});
