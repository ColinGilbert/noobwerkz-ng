use std::cell::RefCell;
use std::rc::Rc;

// use simple_particles::EffectDraw;

use crate::model3d_node::Model3DNode;
use crate::model3d::*;
use crate::particles3d::Particle3DContext;
use crate::scene3d::Characters3DContext;
use crate::skinned_model3d::*;
use crate::index_types::*;
use crate::terrain_model::TerrainModel3D;
use crate::terrain_model_node::TerrainModel3DNode;

pub mod forward_renderer3d;

pub trait Pass {
    fn draw(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, models: &Models3D<Model3D>, skinned_models: &SkinnedModels3D<SkinnedModel3D>, terrain_models: &TerrainModels3D<TerrainModel3D>, nodes: &Model3DNodes<Model3DNode>, navmesh_model_node: &Option<Model3DNode>, terrain_nodes: &TerrainModel3DNodes<TerrainModel3DNode>, characters_contexts: &Vec<Characters3DContext>, particles_context: &Rc<RefCell<Particle3DContext>>, depth_texture_view: &wgpu::TextureView, view: &wgpu::TextureView);
}
