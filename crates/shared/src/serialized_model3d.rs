use crate::bone::*;

use std::collections::HashMap;

use msgpacker::*;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedAnimation3D {
    pub channels: Vec<SerializedAnimation3DChannel>,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub enum SerializedAnimation3DKeyframeTypes {
    Translations(Vec<[f32; 3]>),
    Rotations(Vec<[f32; 4]>),
    Scales(Vec<[f32; 3]>),
    Invalid,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedBone3D {
    pub parent: usize,
    pub scale: [f32; 3],
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedAnimation3DChannel {
    pub bone: usize,
    pub data: SerializedAnimation3DKeyframeTypes,
    pub times: Vec<f32>,
}

impl SerializedAnimation3DChannel {
    pub fn new(data: SerializedAnimation3DKeyframeTypes) -> Self {
        Self {
            bone: INVALID_BONE,
            data,
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
            rotation: [0.0, 1.0, 0.0, 0.0],
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

    pub fn recalculate_normals(&mut self) {
        // for (int i = 0; i < num_vertices; i++) {
        //     vertex[i].normal = Vector3(0, 0, 0);
        // }
        self.normals.clear();
        for _ in &self.positions {
            self.normals.push([0.0, 0.0, 0.0]);
        }

        for indices in self.indices.chunks(3) {
            let id_a = indices[0];
            let id_b = indices[1];
            let id_c = indices[2];

            let a = glam::Vec3::from_array(self.positions[id_a as usize]);
            let b = glam::Vec3::from_array(self.positions[id_b as usize]);
            let c = glam::Vec3::from_array(self.positions[id_c as usize]);

            let edge_1 = b - a;
            let edge_2 = c - a;
            let face_normal = edge_1.cross(edge_2);

            let normal_a = glam::Vec3::from_array(self.normals[id_a as usize]) + face_normal;
            let normal_b = glam::Vec3::from_array(self.normals[id_b as usize]) + face_normal;
            let normal_c = glam::Vec3::from_array(self.normals[id_c as usize]) + face_normal;

            self.normals[id_a as usize] = normal_a.to_array();
            self.normals[id_b as usize] = normal_b.to_array();
            self.normals[id_c as usize] = normal_c.to_array();
        }
        #[allow(unused)]
        for mut n in self.normals.iter_mut() {
            let normalized = glam::Vec3::new(n[0], n[1], n[2]).normalize();
            n = &mut normalized.to_array();
        }

        // // 2. Loop through all triangles
        // for (int i = 0; i < num_triangles; i++) {
        //     Index idxA = triangle[i].a;
        //     Index idxB = triangle[i].b;
        //     Index idxC = triangle[i].c;

        //     Vector3 A = vertex[idxA].position;
        //     Vector3 B = vertex[idxB].position;
        //     Vector3 C = vertex[idxC].position;

        //     // Calculate face normal via cross product
        //     Vector3 edge1 = B - A;
        //     Vector3 edge2 = C - A;
        //     Vector3 faceNormal = cross(edge1, edge2);

        //     // Accumulate onto each vertex of the triangle
        //     vertex[idxA].normal += faceNormal;
        //     vertex[idxB].normal += faceNormal;
        //     vertex[idxC].normal += faceNormal;
        // }

        // // 3. Normalize all vertex normals
        // for (int i = 0; i < num_vertices; i++) {
        //     vertex[i].normal = normalize(vertex[i].normal);
        // }
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedModelMaterial {
    pub name: String,
    pub base_colour: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub diffuse_texture_path: String,
    pub normals_texture_path: String,
    pub specular_texture_path: String,
}

impl SerializedModelMaterial {
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
    pub materials: Vec<SerializedModelMaterial>,
    pub bones: Vec<SerializedBone3D>,
    pub bone_names: Vec<String>,
    pub inverse_bind_matrices: Vec<[[f32; 4]; 4]>,
    pub animations: Vec<SerializedAnimation3D>,
    pub animations_by_name: HashMap<String, usize>,
    pub global_inverse_transform: [[f32; 4]; 4],
}

impl SerializedModel3D {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
            bones: Vec::new(),
            bone_names: Vec::new(),
            inverse_bind_matrices: Vec::new(),
            animations: Vec::new(),
            animations_by_name: HashMap::new(),
            global_inverse_transform: glam::Mat4::IDENTITY.to_cols_array_2d(),
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
