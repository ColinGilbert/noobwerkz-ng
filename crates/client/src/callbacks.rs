use once_cell::sync::Lazy;
use std::sync::*;
use crate::{camera3d::CameraContext, egui_renderer::EguiRenderer, graphics::*, light3d::*, client_user_context::*};


pub static USER_SETUP_CALLBACK: Lazy<Mutex<Option<fn(&mut GraphicsContext, &mut ClientUserContext, &mut Vec<Light3DUniform>)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_setup_callback(callback: fn (gfx_ctx: &mut GraphicsContext, &mut ClientUserContext, &mut Vec<Light3DUniform>)) {
    *USER_SETUP_CALLBACK.lock().unwrap() = Some(callback);
}

pub static USER_UPDATE_CALLBACK: Lazy<Mutex<Option<fn(&mut GraphicsContext, &mut CameraContext, &mut Light3DContext, &mut ClientUserContext, std::time::Duration)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_update_callback(callback: fn (gfx_ctx: &mut GraphicsContext, &mut CameraContext, &mut Light3DContext, &mut ClientUserContext, dt: std::time::Duration)) {
    *USER_UPDATE_CALLBACK.lock().unwrap() = Some(callback);
}

pub static USER_GUI_CALLBACK: Lazy<Mutex<Option<fn(&mut EguiRenderer, &mut ClientUserContext)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_gui_callback(callback: fn (&mut EguiRenderer, &mut ClientUserContext)) {
    *USER_GUI_CALLBACK.lock().unwrap() = Some(callback);
}
