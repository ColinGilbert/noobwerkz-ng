use crate::model_node::ModelNode;
use crate::model::*;

pub mod forward_renderer;

pub trait Pass {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, models: &Vec<Model>, skinned_models: &Vec<SkinnedModel>, nodes: &Vec<ModelNode>, skinned_model_nodes: &Vec<ModelNode>, depth_texture_view: &wgpu::TextureView, view: &wgpu::TextureView );
}
