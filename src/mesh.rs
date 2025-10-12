use std::collections::*;
use glam::*;

struct TexturedMesh3d {
  verts: Vec<glam::Vec3>,
  normals: Vec<glam::Vec3>,
  uvs: Vec<glam::Vec2>,
}