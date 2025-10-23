use std::sync::*;

pub struct UserContext {
pub x: f32,
}

impl UserContext {
    pub fn new() -> Self {
        Self {
            x: 47.0,
        }
    }
}

type SharedContext = Arc<Mutex<UserContext>>;

static USER_CONTEXT : LazyLock<SharedContext> = LazyLock::new(|| {
    let ctx = UserContext::new();
    Arc::new(Mutex::new(ctx))
});