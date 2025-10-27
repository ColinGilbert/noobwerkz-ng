use once_cell::sync::Lazy;
use std::sync::*;
use crate::{camera_context::CameraContext, graphics_context::*, light::*, user_context::*};


pub static USER_SETUP_CALLBACK: Lazy<Mutex<Option<fn(&mut GraphicsContext, &mut UserContext, &mut Vec<LightUniform>)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_setup_callback(callback: fn (gfx_ctx: &mut GraphicsContext, &mut UserContext, &mut Vec<LightUniform>)) {
    *USER_SETUP_CALLBACK.lock().unwrap() = Some(callback);
}

pub static USER_UPDATE_CALLBACK: Lazy<Mutex<Option<fn(&mut GraphicsContext, &mut CameraContext, &mut LightContext, &mut UserContext, std::time::Duration)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_update_callback(callback: fn (gfx_ctx: &mut GraphicsContext, &mut CameraContext, &mut LightContext, &mut UserContext, dt: std::time::Duration)) {
    *USER_UPDATE_CALLBACK.lock().unwrap() = Some(callback);
}
