use std::collections::*;
use glam::*;
use safe_index::*;
use crate::types::*;

safe_index::new! {
  Mesh3dHandle,
  map: Meshes3d
}

pub struct Mesh3d {
  verts: Vec<NoobVec3f32>,
  normals: Vec<NoobVec3f32>,
  uvs: Vec<glam::Vec2>,
}