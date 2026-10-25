use crate::model3d_node::Model3DNode;
use crate::model3d::*;
use crate::scene3d::Characters3DContext;
use crate::skinned_model3d::*;
use crate::index_types::*;

pub mod forward_renderer3d;
pub mod particle_renderer3d;

pub trait Pass {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, models: &Models3D<Model3D>, skinned_models: &SkinnedModels3D<SkinnedModel3D>, nodes: &Model3DNodes<Model3DNode>, characters_contexts: &Characters3DContexts<Characters3DContext>, depth_texture_view: &wgpu::TextureView, view: &wgpu::TextureView );
}
