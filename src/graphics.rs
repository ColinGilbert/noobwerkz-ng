use wgpu::*;
use crate::mesh::*;

pub struct GraphicsContext {
    pub meshes: Mesh3dMap<Mesh3d>,
}

// This is the main rendering loop
pub fn render(wgpu_backend: &mut wgpu::Device, graphics_context: &GraphicsContext) {


}

// Here are helper functions