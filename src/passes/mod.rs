use crate::model_node::ModelNode;
use crate::normal_mapped_model::*;

pub mod phong;
pub trait Pass {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, models: &Vec<NormalMappedModel>, nodes: &Vec<ModelNode>, depth_texture_view: &wgpu::TextureView, view: &wgpu::TextureView );
}
