use once_cell::sync::Lazy;
use std::sync::*;
use crate::{server_user_context::*};


pub static USER_SETUP_CALLBACK: Lazy<Mutex<Option<fn(&mut ServerUserContext)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_setup_callback(callback: fn (&mut ServerUserContext)) {
    *USER_SETUP_CALLBACK.lock().unwrap() = Some(callback);
}

pub static USER_UPDATE_CALLBACK: Lazy<Mutex<Option<fn(&mut ServerUserContext, std::time::Duration)>>> = Lazy::new(|| Mutex::new(None));

pub fn init_user_update_callback(callback: fn (&mut ServerUserContext, dt: std::time::Duration)) {
    *USER_UPDATE_CALLBACK.lock().unwrap() = Some(callback);
}