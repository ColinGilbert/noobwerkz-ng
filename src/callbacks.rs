use once_cell::sync::Lazy;
use std::sync::*;
use crate::graphics_context::*;


pub static USER_SETUP_CALLBACK: Lazy<Mutex<Option<fn(&mut GraphicsContext)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_setup_callback(callback: fn (gfx_ctx: &mut GraphicsContext)) {
    *USER_SETUP_CALLBACK.lock().unwrap() = Some(callback);
}