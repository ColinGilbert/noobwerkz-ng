use crate::bone::*;

use std::collections::HashMap;

use msgpacker::*;
use noobwerkz_proto::{protobuf_schema::*, *};

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

impl SerializedAnimation3DKeyframeTypes {
    pub fn from_protobuf(other: SerializedAnimation3DKeyframeTypesProtobuf) -> Self {
        if other.translations.len() > 0 {
            let mut translations = Vec::new();
            for t in other.translations {
                translations.push([t.vec3f_x, t.vec3f_y, t.vec3f_z]);
            }
            return SerializedAnimation3DKeyframeTypes::Translations(translations);
        } else if other.rotations.len() > 0 {
            let mut rotations = Vec::new();
            for r in other.rotations {
                rotations.push([r.vec4f_x, r.vec4f_y, r.vec4f_z, r.vec4f_w])
            }
            return SerializedAnimation3DKeyframeTypes::Rotations(rotations);
        } else if other.scales.len() > 0 {
            let mut scales = Vec::new();
            for s in other.scales {
                scales.push([s.vec3f_x, s.vec3f_y, s.vec3f_z]);
            }
            return SerializedAnimation3DKeyframeTypes::Scales(scales);
        } else {
            return SerializedAnimation3DKeyframeTypes::Invalid;
        }
    }

    pub fn to_protobuf(&self) -> SerializedAnimation3DKeyframeTypesProtobuf {
        let mut results = SerializedAnimation3DKeyframeTypesProtobuf::default();
        match self {
            SerializedAnimation3DKeyframeTypes::Translations(val) => {
                for v in val {
                    let mut pv = Vec3fProtobuf::default();
                    pv.vec3f_x = v[0];
                    pv.vec3f_y = v[1];
                    pv.vec3f_z = v[2];
                    results.translations.push(pv);
                }
            }
            SerializedAnimation3DKeyframeTypes::Rotations(val) => {
                for v in val {
                    let mut pv = Vec4fProtobuf::default();
                    pv.vec4f_x = v[0];
                    pv.vec4f_y = v[1];
                    pv.vec4f_z = v[2];
                    pv.vec4f_w = v[3];
                    results.rotations.push(pv);
                }
            }
            SerializedAnimation3DKeyframeTypes::Scales(val) => {
                for v in val {
                    let mut pv = Vec3fProtobuf::default();
                    pv.vec3f_x = v[0];
                    pv.vec3f_y = v[1];
                    pv.vec3f_z = v[2];
                    results.scales.push(pv);
                }
            }
            SerializedAnimation3DKeyframeTypes::Invalid => {}
        }

        results
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedBone3D {
    pub parent: usize,
    pub scale: [f32; 3],
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
}

impl SerializedBone3D {
    pub fn from_protobuf(other: SerializedBone3DProtobuf) -> Option<Self> {
        if other.scale.is_none() || other.translation.is_none() || other.rotation.is_none() {
            return None;
        }
        Some(Self {
            parent: other.parent as usize,
            scale: [
                other.scale.unwrap().vec3f_x,
                other.scale.unwrap().vec3f_y,
                other.scale.unwrap().vec3f_z,
            ],
            translation: [
                other.translation.unwrap().vec3f_x,
                other.translation.unwrap().vec3f_y,
                other.translation.unwrap().vec3f_z,
            ],
            rotation: [
                other.rotation.unwrap().vec4f_x,
                other.rotation.unwrap().vec4f_y,
                other.rotation.unwrap().vec4f_z,
                other.rotation.unwrap().vec4f_w,
            ],
        })
    }
    pub fn to_protobuf(&self) -> SerializedBone3DProtobuf {
        let mut results = SerializedBone3DProtobuf::default();
        results.parent = self.parent as u64;
        let mut scale = Vec3fProtobuf::default();
        scale.vec3f_x = self.scale[0];
        scale.vec3f_y = self.scale[1];
        scale.vec3f_z = self.scale[2];
        results.scale = Some(scale);
        let mut translation = Vec3fProtobuf::default();
        translation.vec3f_x = self.translation[0];
        translation.vec3f_y = self.translation[1];
        translation.vec3f_z = self.translation[2];
        results.translation = Some(translation);
        let mut rotation = Vec4fProtobuf::default();
        rotation.vec4f_x = self.rotation[0];
        rotation.vec4f_y = self.rotation[1];
        rotation.vec4f_z = self.rotation[2];
        rotation.vec4f_w = self.rotation[3];
        results.rotation = Some(rotation);

        results
    }
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

    pub fn from_protobuf(other: SerializedAnimation3DChannelProtobuf) -> Option<Self> {
        if other.data.is_none() {
            return None;
        }
        Some(Self {
            bone: other.bone as usize,
            data: SerializedAnimation3DKeyframeTypes::from_protobuf(other.data.unwrap()),
            times: other.times,
        })
    }

    pub fn to_protobuf(&self) -> SerializedAnimation3DChannelProtobuf {
        let mut results = SerializedAnimation3DChannelProtobuf::default();
        results.bone = self.bone as u64;
        results.data = Some(SerializedAnimation3DKeyframeTypes::to_protobuf(&self.data));
        results.times = self.times.clone();

        results
    }
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedMesh3D {
    pub name: String,
    pub translation: [f32; 3],
    pub scale: [f32; 3],
    pub rotation: [f32; 4],
    pub max_extents: [f32; 3],
    pub min_extents: [f32; 3],
    pub dimensions: [f32; 3],
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
            rotation: glam::Quat::IDENTITY.to_array(),
            max_extents: [0.0; 3],
            min_extents: [0.0; 3],
            dimensions: [0.0; 3],
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

    pub fn from_protobuf(other: SerializedMesh3DProtobuf) -> Option<Self> {
        if other.translation.is_none()
            || other.scale.is_none()
            || other.rotation.is_none()
            || other.max_extents.is_none()
            || other.min_extents.is_none()
            || other.dimensions.is_none()
        {
            return None;
        }

        let mut positions = Vec::new();
        for p in other.positions {
            positions.push([p.vec3f_x, p.vec3f_y, p.vec3f_z]);
        }
        let mut normals = Vec::new();
        for n in other.normals {
            normals.push([n.vec3f_x, n.vec3f_y, n.vec3f_z]);
        }
        let mut uvs = Vec::new();
        for u in other.uvs {
            uvs.push([u.vec2f_x, u.vec2f_y]);
        }
        let mut bone_indices = Vec::new();
        for bi in other.bone_indices {
            bone_indices.push([bi.vec4u_x, bi.vec4u_y, bi.vec4u_z, bi.vec4u_w]);
        }
        let mut bone_weights = Vec::new();
        for bw in other.bone_weights {
            bone_weights.push([bw.vec4f_x, bw.vec4f_y, bw.vec4f_z, bw.vec4f_w]);
        }
        Some(Self {
            name: other.name,
            translation: [
                other.translation.unwrap().vec3f_x,
                other.translation.unwrap().vec3f_y,
                other.translation.unwrap().vec3f_z,
            ],
            scale: [
                other.scale.unwrap().vec3f_x,
                other.scale.unwrap().vec3f_y,
                other.scale.unwrap().vec3f_z,
            ],
            rotation: [
                other.rotation.unwrap().vec4f_x,
                other.rotation.unwrap().vec4f_y,
                other.rotation.unwrap().vec4f_z,
                other.rotation.unwrap().vec4f_w,
            ],
            max_extents: [
                other.max_extents.unwrap().vec3f_x,
                other.max_extents.unwrap().vec3f_y,
                other.max_extents.unwrap().vec3f_z,
            ],
            min_extents: [
                other.min_extents.unwrap().vec3f_x,
                other.min_extents.unwrap().vec3f_y,
                other.min_extents.unwrap().vec3f_z,
            ],
            dimensions: [
                other.dimensions.unwrap().vec3f_x,
                other.dimensions.unwrap().vec3f_y,
                other.dimensions.unwrap().vec3f_z,
            ],
            positions,
            normals,
            uvs,
            bone_indices,
            bone_weights,
            indices: other.indices.clone(),
            bone_names: other.bone_names.clone(),
            material_index: other.material_index,
        })
    }

    pub fn to_protobuf(&self) -> SerializedMesh3DProtobuf {
        let mut results = SerializedMesh3DProtobuf::default();
        results.name = self.name.clone();
        results.translation = Some(Vec3fProtobuf {
            vec3f_x: self.translation[0],
            vec3f_y: self.translation[1],
            vec3f_z: self.translation[2],
        });
        results.scale = Some(Vec3fProtobuf {
            vec3f_x: self.scale[0],
            vec3f_y: self.scale[1],
            vec3f_z: self.scale[2],
        });
        results.rotation = Some(Vec4fProtobuf {
            vec4f_x: self.rotation[0],
            vec4f_y: self.rotation[1],
            vec4f_z: self.rotation[2],
            vec4f_w: self.rotation[3],
        });
        results.min_extents = Some(Vec3fProtobuf {
            vec3f_x: self.min_extents[0],
            vec3f_y: self.min_extents[1],
            vec3f_z: self.min_extents[2],
        });
        results.max_extents = Some(Vec3fProtobuf {
            vec3f_x: self.max_extents[0],
            vec3f_y: self.max_extents[1],
            vec3f_z: self.max_extents[2],
        });
        results.dimensions = Some(Vec3fProtobuf {
            vec3f_x: self.dimensions[0],
            vec3f_y: self.dimensions[1],
            vec3f_z: self.dimensions[2],
        });

        // let mut positions = Vec::new();
        for p in &self.positions {
            results.positions.push(Vec3fProtobuf {
                vec3f_x: p[0],
                vec3f_y: p[1],
                vec3f_z: p[2],
            });
        }
        // let mut normals = Vec::new();
        for n in &self.normals {
            results.normals.push(Vec3fProtobuf {
                vec3f_x: n[0],
                vec3f_y: n[1],
                vec3f_z: n[2],
            });
        }
        // let mut uvs = Vec::new();
        for u in &self.uvs {
            results.uvs.push(Vec2fProtobuf {
                vec2f_x: u[0],
                vec2f_y: u[1],
            });
        }
        // let mut bone_indices = Vec::new();
        for bi in &self.bone_indices {
            results.bone_indices.push(Vec4uProtobuf {
                vec4u_x: bi[0],
                vec4u_y: bi[1],
                vec4u_z: bi[2],
                vec4u_w: bi[3],
            });
        }
        // let mut bone_weights = Vec::new();
        for bw in &self.bone_weights {
            results.bone_weights.push(Vec4fProtobuf {
                vec4f_x: bw[0],
                vec4f_y: bw[1],
                vec4f_z: bw[2],
                vec4f_w: bw[3],
            });
        }

        results.indices = self.indices.clone();
        results.bone_names = self.bone_names.clone();
        results.material_index = self.material_index;

        results
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
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, MsgPacker)]
pub struct SerializedTerrainModel3D {
    pub meshes: Vec<SerializedMesh3D>,
    pub materials: Vec<SerializedModelMaterial>,
}

impl SerializedTerrainModel3D {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
        }
    }
}
