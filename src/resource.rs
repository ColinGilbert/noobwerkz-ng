use asset_importer_rs_gltf::{
    Gltf2Importer,
    Gltf2ImportError,
};
use std::path::Path;
use asset_importer_rs::*;
use asset_importer_rs_core::*;
use asset_importer_rs_scene::{AiMesh, AiScene};



pub fn import_gltf2(path_string: &str) {
    let scene = gltf_load(path_string);
}

fn gltf_load(path_string: &str) -> AiScene {
  // Create an importer
  let importer = Gltf2Importer::new();

  // Import a glTF file
  let scene = importer.read_file(Path::new(path_string), asset_importer_rs_core::default_file_loader).unwrap();

//   // Access scene data
//   println!("Scene has {} meshes", scene.meshes.len());
//   for m in scene.meshes {
//     println!("  Mesh {} has {} verts, {} normals, {} tangents, and {} bi-tangents", m.name, m.vertices.len(), m.normals.len(), m.tangents.len(), m.bi_tangents.len());
//   }
//   println!("Scene has {} materials", scene.materials.len());

  scene
}