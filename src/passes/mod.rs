pub mod phong;
use crate::model_node::ModelNode;
pub trait pass {
    fn draw(&mut self, surface: &wgpu::Surface, device: &wgpu::Device, queue:&wgpu::Queue, nodes: &ModelNode ) {}
}