use std::collections::*;
use glam::*;
use safe_index::*;

safe_index::new! {
  Mesh3dHandle,
  map: Mesh3dMap
}

pub struct Mesh3d {
  verts: Vec<glam::Vec3>,
  normals: Vec<glam::Vec3>,
  uvs: Vec<glam::Vec2>,
}