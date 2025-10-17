// THis file defines our graphics context and the render loop.

use crate::camera::*;


// Here are the typesafe indices that we need
safe_index::new! {
  PipelineIndex,
  map: Pipelines
}

// This is the graphics context used by the windowing subsystem 
pub struct GraphicsContext {
    //pub pipeline: wgpu::RenderPipeline,
    pub pipelines: Pipelines<wgpu::RenderPipeline>,
    pub camera: Camera,
    pub projection: Projection,
    // pub camera_controller: CameraController,
    pub camera_uniform: CameraUniform,
    
}

impl GraphicsContext {
  pub fn new(height: u32, width: u32) -> Self {
    GraphicsContext {
        pipelines: Pipelines::<wgpu::RenderPipeline>::new(),
        camera: Camera::new(),
        projection: Projection::new(height, width),
        camera_uniform: CameraUniform::new(),
    } 
  }
}




// This is the main rendering loop
pub fn render(_wgpu_backend: &mut wgpu::Device, _graphics_context: &GraphicsContext) {


}