use crate::model_node::ModelNode;
use crate::model3d::*;
use crate::scene3d::CharactersContext;
use crate::skinned_model::*;
pub mod forward_renderer;

pub trait Pass {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, models: &Vec<Model3D>, skinned_models: &Vec<SkinnedModel>, nodes: &Vec<ModelNode>, skinned_model_nodes: &Vec<CharactersContext>, depth_texture_view: &wgpu::TextureView, view: &wgpu::TextureView );
}
