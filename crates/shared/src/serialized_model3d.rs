use crate::bone::*;

use std::collections::HashMap;

use noobwerkz_proto::{protobuf_schema::*};

#[derive(Clone, Debug, PartialEq)]
pub struct SerializedAnimation3D {
    pub channels: Vec<SerializedAnimation3DChannel>,
}

impl SerializedAnimation3D {
    pub fn from_protobuf(other: SerializedAnimation3DProtobuf) -> Option<Self> {
        let mut channels = Vec::new();
        for channel_proto in other.channels {
            let c = SerializedAnimation3DChannel::from_protobuf(channel_proto);
            if c.is_none() {
                return None;
            }
            channels.push(c.unwrap());
        }
        Some(Self { channels })
    }

    pub fn to_protobuf(&self) -> SerializedAnimation3DProtobuf {
        let mut results = SerializedAnimation3DProtobuf::default();
        for c in &self.channels {
            results
                .channels
                .push(SerializedAnimation3DChannel::to_protobuf(c));
        }
        results
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
        let data = SerializedAnimation3DKeyframeTypes::from_protobuf(other.data.unwrap());
        if data == SerializedAnimation3DKeyframeTypes::Invalid {
            println!("Invalid keyframes found while deserializing keyframes.");
            return None;
        }
        Some(Self {
            bone: other.bone as usize,
            data,
            times: other.times.clone(),
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

#[derive(Clone, Debug, PartialEq)]
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
            println!("Could not deserialize mesh from protobuf");
            return None;
        }

        let mut positions = Vec::new();
        for p in other.positions {
            positions.push([p.vec3f_x, p.vec3f_y, p.vec3f_z]);
        }
        // println!("DEBUG: SerializedMesh3D - Positions {}", positions.len());
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
            name: other.name.clone(),
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

#[derive(Clone, Debug, PartialEq)]
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

    pub fn from_protobuf(other: SerializedModelMaterialProtobuf) -> Option<Self> {
        if other.base_colour.is_none() {
            return None;
        }
        Some(SerializedModelMaterial {
            name: other.name.clone(),
            base_colour: [
                other.base_colour.unwrap().vec4f_x,
                other.base_colour.unwrap().vec4f_y,
                other.base_colour.unwrap().vec4f_z,
                other.base_colour.unwrap().vec4f_w,
            ],
            metallic: other.metallic,
            roughness: other.roughness,
            diffuse_texture_path: other.diffuse_texture_path.clone(),
            normals_texture_path: other.normals_texture_path.clone(),
            specular_texture_path: other.specular_texture_path.clone(),
        })
    }

    pub fn to_protobuf(&self) -> SerializedModelMaterialProtobuf {
        let mut results = SerializedModelMaterialProtobuf::default();
        results.name = self.name.clone();
        results.base_colour = Some(Vec4fProtobuf {
            vec4f_x: self.base_colour[0],
            vec4f_y: self.base_colour[1],
            vec4f_z: self.base_colour[2],
            vec4f_w: self.base_colour[3],
        });
        results.metallic = self.metallic;
        results.roughness = self.roughness;
        results.diffuse_texture_path = self.diffuse_texture_path.clone();
        results.normals_texture_path = self.normals_texture_path.clone();
        results.specular_texture_path = self.specular_texture_path.clone();

        results
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SerializedModel3D {
    pub meshes: Vec<SerializedMesh3D>,
    pub materials: Vec<SerializedModelMaterial>,
    pub bones: Vec<SerializedBone3D>,
    pub bone_names: Vec<String>,
    pub inverse_bind_matrices: Vec<[[f32; 4]; 4]>,
    pub animations: Vec<SerializedAnimation3D>,
    pub animations_by_name: HashMap<String, usize>,
    // pub global_inverse_transform: [[f32; 4]; 4],
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
            // global_inverse_transform: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn from_protobuf(other: SerializedModel3DProtobuf) -> Option<Self> {
        let mut meshes = Vec::new();
        for mesh_proto in other.meshes {
            let m = SerializedMesh3D::from_protobuf(mesh_proto);
            if m.is_none() {
                return None;
            }
            meshes.push(m.unwrap());
        }

        let mut materials = Vec::new();
        for mat_proto in other.materials {
            let m = SerializedModelMaterial::from_protobuf(mat_proto);
            if m.is_none() {
                return None;
            }
            materials.push(m.unwrap());
        }

        let mut bones = Vec::new();
        for bone_proto in other.bones {
            let b = SerializedBone3D::from_protobuf(bone_proto);
            if b.is_none() {
                return None;
            }
            bones.push(b.unwrap());
        }

        let mut inverse_bind_matrices = Vec::new();
        for ibm_proto in other.inverse_bind_matrices {
            if ibm_proto.mat4f_a.is_none()
                || ibm_proto.mat4f_b.is_none()
                || ibm_proto.mat4f_c.is_none()
                || ibm_proto.mat4f_d.is_none()
            {
                return None;
            }
            let a = ibm_proto.mat4f_a.unwrap();
            let b = ibm_proto.mat4f_b.unwrap();
            let c = ibm_proto.mat4f_c.unwrap();
            let d = ibm_proto.mat4f_d.unwrap();
            inverse_bind_matrices.push([
                [a.vec4f_x, a.vec4f_y, a.vec4f_z, a.vec4f_w],
                [b.vec4f_x, b.vec4f_y, b.vec4f_z, b.vec4f_w],
                [c.vec4f_x, c.vec4f_y, c.vec4f_z, c.vec4f_w],
                [d.vec4f_x, d.vec4f_y, d.vec4f_z, d.vec4f_w],
            ]);
        }

        let mut animations = Vec::new();
        for anim_proto in other.animations {
            let a = SerializedAnimation3D::from_protobuf(anim_proto);
            if a.is_none() {
                return None;
            }
            animations.push(a.unwrap());
        }

        let mut animations_by_name = HashMap::new();

        for anim_name_proto in other.animations_by_name {
            let i = anim_name_proto.index as usize;
            let name = anim_name_proto.name;
            animations_by_name.insert(name, i);
        }

        Some(Self {
            meshes,
            materials,
            bones,
            bone_names: other.bone_names.clone(),
            inverse_bind_matrices,
            animations,
            animations_by_name,
        })
    }

    pub fn to_protobuf(&self) -> SerializedModel3DProtobuf {
        let mut results = SerializedModel3DProtobuf::default();

        for m in &self.meshes {
            results.meshes.push(m.to_protobuf());
        }

        for m in &self.materials {
            results
                .materials
                .push(m.to_protobuf());
        }

        for b in &self.bones {
            results.bones.push(b.to_protobuf());
        }

        results.bone_names = self.bone_names.clone();
        for ibm in &self.inverse_bind_matrices {
            results.inverse_bind_matrices.push(Mat4fProtobuf {
                mat4f_a: Some(Vec4fProtobuf {
                    vec4f_x: ibm[0][0],
                    vec4f_y: ibm[0][1],
                    vec4f_z: ibm[0][2],
                    vec4f_w: ibm[0][3],
                }),
                mat4f_b: Some(Vec4fProtobuf {
                    vec4f_x: ibm[1][0],
                    vec4f_y: ibm[1][1],
                    vec4f_z: ibm[1][2],
                    vec4f_w: ibm[1][3],
                }),
                mat4f_c: Some(Vec4fProtobuf {
                    vec4f_x: ibm[2][0],
                    vec4f_y: ibm[2][1],
                    vec4f_z: ibm[2][2],
                    vec4f_w: ibm[2][3],
                }),
                mat4f_d: Some(Vec4fProtobuf {
                    vec4f_x: ibm[3][0],
                    vec4f_y: ibm[3][1],
                    vec4f_z: ibm[3][2],
                    vec4f_w: ibm[3][3],
                }),
            })
        }

        for a in &self.animations {
            results.animations.push(a.to_protobuf());
        }

        for (name, index) in &self.animations_by_name {
            results.animations_by_name.push(AnimationByNameValuesProtobuf { name: name.clone(), index: *index as u64 })
        }

        results
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn from_protobuf(other: SerializedTerrainModel3DProtobuf) -> Option<Self> {

        let mut results = SerializedTerrainModel3D::new();
        for mesh_proto in other.meshes {
            let m = SerializedMesh3D::from_protobuf(mesh_proto);
            if m.is_none() { return None; }
            results.meshes.push(m.unwrap());
        }
        for mat_proto in other.materials {
            let m = SerializedModelMaterial::from_protobuf(mat_proto);
            if m.is_none() { return None; }
            results.materials.push(m.unwrap());
        }

        Some(results)
    }


    pub fn to_protobuf(&self) -> SerializedTerrainModel3DProtobuf {
        let mut results = SerializedTerrainModel3DProtobuf::default();
        for m in &self.meshes {
            results.meshes.push(m.to_protobuf());
        }
        for m in &self.materials {
            results.materials.push(m.to_protobuf());
        }
        results
    }
}
