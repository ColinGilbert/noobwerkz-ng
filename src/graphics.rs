use crate::mesh::*;
use crate::camera::*;

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
    //pub pipeline: wgpu::RenderPipeline,
    pub pipelines: Pipelines<wgpu::RenderPipeline>,
    pub camera: Camera,
    pub projection: CameraProjection,
    // pub camera_controller: CameraController,
    pub camera_uniform: CameraUniform,
    
}

impl GraphicsContext {
  pub fn new(height: f32, width: f32) -> Self {
    GraphicsContext {
        meshes: Meshes3d::<Mesh3d>::new(),
        pipelines: Pipelines::<wgpu::RenderPipeline>::new(),
        camera: Camera::new(),
        projection: CameraProjection::new(height, width),
        camera_uniform: CameraUniform::new(),
        //vbos: VBOs::<wgpu::Buffer>::new()
    } 
  }
}

pub struct GPUMesh {
  pub name: String,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
}

// This is the main rendering loop
pub fn render(wgpu_backend: &mut wgpu::Device, graphics_context: &GraphicsContext) {


}

// Here are helper functions
