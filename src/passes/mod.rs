use crate::model_node::ModelNode;
pub mod phong;
pub trait Pass {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, nodes: &Vec<ModelNode>, depth_texture_view: &wgpu::TextureView, view: &wgpu::TextureView );
}
