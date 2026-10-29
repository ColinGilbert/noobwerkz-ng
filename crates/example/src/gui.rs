use noobwerkz_client::{egui_renderer::EguiRenderer, client_user_context::ClientUserContext};

// MODIFY ME!
#[allow(unused)]
pub fn user_gui(egui_renderer: &mut EguiRenderer, user_ctx: &mut ClientUserContext) {
    noobwerkz_client::egui::Window::new("Noobwerkz engine!")
        .resizable(true)
        .min_height(80.0)
        .max_height(80.0)
        .vscroll(true)
        .default_open(true)
        .collapsible(false)
        .movable(false)
        .show(egui_renderer.context(), |ui| {
            ui.label("This is a demonstration of the game engine's capabilities. It'll get better over time.");
        });
}
