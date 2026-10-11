use crate::model3d_node::Model3DNode;
use crate::model3d::*;
use crate::scene3d::CharactersContext;
use crate::skinned_model::*;
use crate::index_types::*;
pub mod forward_renderer;

pub trait Pass {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, models: &Models3D<Model3D>, skinned_models: &SkinnedModels<SkinnedModel>, nodes: &Model3DNodes<Model3DNode>, skinned_model_nodes: &CharactersContexts<CharactersContext>, depth_texture_view: &wgpu::TextureView, view: &wgpu::TextureView );
}
