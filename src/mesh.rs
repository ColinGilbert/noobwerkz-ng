use std::collections::*;
use glam::*;
use safe_index::*;
use crate::types::*;

safe_index::new! {
  Mesh3dIndex,
  map: Meshes3d
}

safe_index::new! {
    VertIndex,
    map: Verts
}

pub struct Vert {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: glam::Vec2

}

pub struct Mesh3d {
    pub verts: Verts<Vert>
}

impl Mesh3d {
    pub fn new() -> Self {
        Self {
           verts: Verts::<Vert>::new(),
        }
    }
}


// This represents a mesh that lives on the GPU
pub struct GPUMesh {
  pub name: String,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
}

