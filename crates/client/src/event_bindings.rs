use std::collections::HashMap;

pub enum KeyboardDrivenEvent {
    CameraBack,
    CameraForward,
    CameraLeft,
    CameraRight,
    
}

pub struct EventsContext {
    pub keybindings: HashMap<winit::keyboard::KeyCode, KeyboardDrivenEvent>,
}