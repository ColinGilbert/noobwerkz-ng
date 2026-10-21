use msgpacker::*;

pub const INVALID_BONE: usize = usize::MAX;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, MsgPacker)]
pub struct SerializedBone3D {
    pub parent: usize,
    pub scale: [f32; 3],
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedAnimationKeyframes {
    pub scales: Vec<[f32; 3]>,
    pub translations: Vec<[f32; 3]>,
    pub rotations: Vec<[f32; 4]>,
    pub times: Vec<f32>,

}

impl SerializedAnimationKeyframes {
    pub fn new() -> Self {
        Self {
            scales: Vec::new(),
            translations: Vec::new(),
            rotations: Vec::new(),
            times: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedMesh3D {
    pub name: String,
    pub translation: [f32; 3],
    pub scale: [f32; 3],
    pub max_extents: [f32; 3],
    pub min_extents: [f32; 3],
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

impl SerializedMesh3D {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            translation: [0.0; 3],
            scale: [1.0; 3],
            max_extents: [0.0; 3],
            min_extents: [0.0; 3],
            dimensions: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
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

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedMaterial {
    pub name: String,
    pub base_colour: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub diffuse_texture_path: String,
    pub normals_texture_path: String,
    pub specular_texture_path: String,
}

impl SerializedMaterial {
    pub fn new() -> Self {
        Self {
            name: "".to_owned(),
            base_colour: [1.0; 4],
            metallic: 0.0,
            roughness: 0.0,
            diffuse_texture_path: "".to_owned(),
            normals_texture_path: "".to_owned(),
            specular_texture_path: "".to_owned(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedModel3D {
    pub meshes: Vec<SerializedMesh3D>,
    pub materials: Vec<SerializedMaterial>,
    pub bones: Vec<SerializedBone3D>,
    pub bone_names: Vec<String>,
    pub inverse_bind_matrices: Vec<[[f32; 4]; 4]>,
    pub animations_channels: Vec<SerializedAnimationKeyframes>,
    //pub inverse_transform: [[f32; 4]; 4],
}

impl SerializedModel3D {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
            bones: Vec::new(),
            bone_names: Vec::new(),
            inverse_bind_matrices: Vec::new(),
            animations_channels: Vec::new(),
        }
    }

    // pub fn rotate(&mut self, rotation: glam::Quat) {
    //     let matrix = glam::Mat4::from_quat(rotation);
    //     for m in self.meshes.iter_mut() {
    //         let mut i = 0;
    //         while i < m.positions.len() {
    //             let p = m.positions[i];
    //             let mut pp = glam::Vec4::from_array([p[0], p[1], p[2], 1.0]);
    //             pp = matrix * pp;
    //             m.positions[i] = [pp[0], pp[1], pp[2]];
    //             i += 1;
    //         }
    //     }
    // }
}
