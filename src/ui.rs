// use iced_wgpu::graphics::*;
use iced_wgpu::Renderer;
use iced_widget::{bottom, column}; // row, slider, text, text_input};
// use iced_winit::Clipboard;
// use iced_winit::conversion;
// use iced_winit::core::mouse;
use iced_winit::core::{Element, Theme};
// use iced_winit::core::time::Instant;
// use iced_winit::core::window;
// use iced_winit::core::*;
// use iced_winit::futures;
// use iced_winit::runtime::user_interface::{self, UserInterface};
// use iced_winit::winit;
pub struct UI {}

impl UI {
    pub fn new() -> Self {
        Self {}
    }
}
#[derive(Debug, Clone)]
pub enum UIMessage {
    ButtonPressed(usize),
    TextEntered(usize, String),
    SliderValueChanged(usize, f32),
}

pub trait UIControls {
    fn update(&mut self, message: UIMessage);
    fn view(&self) -> Element<'_, UIMessage, Theme, Renderer>;
}

// A null UI we use to initialize the app.
pub struct NullUIControls {}

impl UIControls for NullUIControls {
    fn update(&mut self, _message: UIMessage) {}
    fn view(&self) -> Element<'_, UIMessage, Theme, Renderer> {
        bottom(column![]).into()
    }
}
