use crate::mesh::*;


safe_index::new! {
  Pipeline,
  map: Pipelines
}

safe_index::new! {
    VertexBufferObject,
    map: VBOs
}

pub struct GraphicsContext {
    pub meshes: Meshes3d<Mesh3d>,
    pub pipelines: Pipelines<wgpu::RenderPipeline>,
    pub vbos : VBOs<wgpu::Buffer>
}

impl GraphicsContext {
  pub fn new() -> Self {
    GraphicsContext {
        meshes: Meshes3d::<Mesh3d>::new(),
        pipelines: Pipelines::<wgpu::RenderPipeline>::new(),
        vbos: VBOs::<wgpu::Buffer>::new()
    } 
  }
}

// This is the main rendering loop
pub fn render(wgpu_backend: &mut wgpu::Device, graphics_context: &GraphicsContext) {


}

// Here are helper functions
