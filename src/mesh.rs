use std::collections::*;
use glam::*;
use safe_index::*;
use crate::types::*;

safe_index::new! {
  Mesh3dHandle,
  map: Meshes3d
}

pub struct Mesh3d {
  verts: Vec<NoobVec3>,
  normals: Vec<NoobVec3>,
  uvs: Vec<glam::Vec2>,
}

