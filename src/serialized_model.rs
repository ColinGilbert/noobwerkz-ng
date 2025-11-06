use msgpacker::prelude::*;

#[derive(Debug, PartialEq, MsgPacker)]
pub struct SerializedMesh {
    pub name: String,
    pub translation: [f32; 3],
    pub scale: [f32; 3],
    pub dimensions: [f32; 3],
    pub rotation: [f32; 4],
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub bone_indices: Vec<[u32; 4]>,
    pub bone_weights: Vec<[f32; 4]>,
    pub indices: Vec<u32>,
    pub bone_names: Vec<String>,
    pub material_index: u32,
}

impl SerializedMesh {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            translation: [0.0; 3],
            scale: [0.0; 3],
            dimensions: [0.0; 3],
            rotation: [0.0; 4],
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            bone_indices: Vec::new(),
            bone_weights: Vec::new(),
            indices: Vec::new(),
            bone_names: Vec::new(),
            material_index: 0,
        }
    }
}

#[derive(Debug, PartialEq, MsgPacker)]
pub struct SerializedMaterial {
    pub name: String,
    pub diffuse_texture_path: String,
    pub normals_texture_path: String,
    pub specular_texture_path: String,
}

impl SerializedMaterial {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            diffuse_texture_path: "".to_owned(),
            normals_texture_path: "".to_owned(),
            specular_texture_path: "".to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, MsgPacker)]
pub struct SerializedModel {
    pub meshes: Vec<SerializedMesh>,
    pub materials: Vec<SerializedMaterial>,
    pub bone_names: Vec<String>,
    pub inverse_bind_matrices: Vec<[[f32;4];4]>
}

#[derive(Copy, Clone)]
impl SerializedModel {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
            bone_names: Vec::new(),
            inverse_bind_matrices: Vec::new()
        }
    }

    pub fn rotate(self, rotation: glam::Quat) {
        let matrix = glam::Mat4::from_quat(rotation);
        let mut i = 0;
        for mut m in self.meshes {
            let p = m.positions[i];
            let mut pp = glam::Vec4::from_array([p[0], p[1], p[2], 1.0]);
            pp = matrix * pp;
            m.positions[i] = [pp[0], pp[1], pp[2]];
            i += 1; 
        }
    }
}