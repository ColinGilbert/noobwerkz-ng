// use crate::model;
// use crate::texture;

// safe_index::new! {
//   SceneNodeIndex,
//   map: Children,
// }

// safe_index::new! {
//   SceneIndex,
//   map: Scenes,
// }

// pub struct SceneNode {
//   transform: glam::Mat4,
//   meshes: Vec<model::TexturedMeshIndex>,
//   children: Children<SceneNode>,
// }

// impl SceneNode {
//   pub fn new() -> Self {
//     Self {
//       transform: glam::Mat4::IDENTITY,
//       meshes: Vec::new(),
//       children: Children::<SceneNode>::new(),
//     }
//   }
// }

// pub struct Scene {
//   meshes: model::TexturedMeshes<model::TexturedMesh>,
//   materials: model::Materials<model::Material>,
//   root: SceneNode,
// }

// impl Scene {
//   pub fn new() -> Self {
//     Self {
//       meshes: model::TexturedMeshes::new(),
//       materials: model::Materials::new(),
//       root: SceneNode::new(),
//     }
//   }
// }