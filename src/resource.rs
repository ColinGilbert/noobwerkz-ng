use crate::index_types::*;
use crate::material::*;
use crate::model::*; //{Material, MaterialIndex, Model, ModelVertex, TexturedMesh};
use crate::skinned_model::*;
use crate::texture;
use crate::texture::Texture;
use serde::{Deserialize, Serialize};
use std::fs;
use wgpu::util::DeviceExt;

pub enum GenericModel {
    Textured(Model),
    SkinnedTextured(SkinnedModel),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SerializedMesh {
    name: String,
    translation: [f32; 3],
    scale: [f32; 3],
    dimensions: [f32; 3],
    rotation: [f32; 4],
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    bone_indices: Vec<[u32; 4]>,
    bone_weights: Vec<[f32; 4]>,
    indices: Vec<u32>,
    bone_names: Vec<String>,
    material_index: u32,
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SerializedMaterial {
    name: String,
    diffuse_texture_path: String,
    normals_texture_path: String,
    specular_texture_path: String,
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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SerializedModel {
    meshes: Vec<SerializedMesh>,
    materials: Vec<SerializedMaterial>,
}

impl SerializedModel {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
        }
    }
}

pub fn load_model_from_json(filepath: String, filename: String) -> SerializedModel {
    let mut o = SerializedModel::new();
    let full_path = filepath.clone() + "/" + &filename;
    // println!("Full path: {}", full_path);
    let data = fs::read_to_string(full_path).unwrap() + " }";
    let parsed = json::parse(&data).unwrap();
    let mut meshes_index = 0;

    while meshes_index < parsed["model"]["meshes"].len() {
        let mut m = SerializedMesh::new();
        m.name = parsed["model"]["meshes"][meshes_index]["name"].to_string();
        let mut node = parsed["model"]["meshes"][meshes_index]["translation"].clone();
        m.translation = [
            node["value0"].as_f32().unwrap(),
            node["value1"].as_f32().unwrap(),
            node["value2"].as_f32().unwrap(),
        ];
        node = parsed["model"]["meshes"][meshes_index]["scale"].clone();
        m.scale = [
            node["value0"].as_f32().unwrap(),
            node["value1"].as_f32().unwrap(),
            node["value2"].as_f32().unwrap(),
        ];
        node = parsed["model"]["meshes"][meshes_index]["dimensions"].clone();
        m.dimensions = [
            node["value0"].as_f32().unwrap(),
            node["value1"].as_f32().unwrap(),
            node["value2"].as_f32().unwrap(),
        ];
        node = parsed["model"]["meshes"][meshes_index]["rotation"].clone();
        m.rotation = [
            node["value0"].as_f32().unwrap(),
            node["value1"].as_f32().unwrap(),
            node["value2"].as_f32().unwrap(),
            node["value3"].as_f32().unwrap(),
        ];
        // positions: Vec<[f32; 3]>,
        node = parsed["model"]["meshes"][meshes_index]["positions"].clone();
        let mut i = 0;
        while i < node.len() {
            let pos = [
                node[i]["value0"].as_f32().unwrap(),
                node[i]["value1"].as_f32().unwrap(),
                node[i]["value2"].as_f32().unwrap(),
            ];
            m.positions.push(pos);
            i += 1;
        }

        // normals: Vec<[f32; 3]>,
        node = parsed["model"]["meshes"][meshes_index]["normals"].clone();
        i = 0;
        while i < node.len() {
            let normal = [
                node[i]["value0"].as_f32().unwrap(),
                node[i]["value1"].as_f32().unwrap(),
                node[i]["value2"].as_f32().unwrap(),
            ];
            m.normals.push(normal);
            i += 1;
        }
        // uvs: Vec<[f32; 2]>,
        node = parsed["model"]["meshes"][meshes_index]["uvs"].clone();
        i = 0;
        while i < node.len() {
            let uv = [
                node[i]["value0"].as_f32().unwrap(),
                node[i]["value1"].as_f32().unwrap(),
            ];
            m.uvs.push(uv);
            i += 1;
        }
        // bone_indices: Vec<[u32; 4]>,
        node = parsed["model"]["meshes"][meshes_index]["bone_indices"].clone();
        i = 0;
        while i < node.len() {
            let bi = [
                node[i]["value0"].as_u32().unwrap(),
                node[i]["value1"].as_u32().unwrap(),
                node[i]["value2"].as_u32().unwrap(),
                node[i]["value3"].as_u32().unwrap(),
            ];
            m.bone_indices.push(bi);
            i += 1;
        }
        // bone_weights: Vec<[f32; 4]>,
        node = parsed["model"]["meshes"][meshes_index]["bone_weights"].clone();
        i = 0;
        while i < node.len() {
            let bw = [
                node[i]["value0"].as_f32().unwrap(),
                node[i]["value1"].as_f32().unwrap(),
                node[i]["value2"].as_f32().unwrap(),
                node[i]["value3"].as_f32().unwrap(),
            ];
            m.bone_weights.push(bw);
            i += 1;
        }
        // indices: Vec<u32>,
        node = parsed["model"]["meshes"][meshes_index]["indices"].clone();
        i = 0;
        while i < node.len() {
            let id = node[i].as_u32().unwrap();
            m.indices.push(id);
            i += 1;
        }
        // bone_names: Vec<String>,
        node = parsed["model"]["meshes"][meshes_index]["bone_names"].clone();
        i = 0;
        while i < node.len() {
            let bn = node[i].to_string();
            m.bone_names.push(bn);
            i += 1;
        }
        // material_index: u32,
        node = parsed["model"]["meshes"][meshes_index]["material_index"].clone();
        i = 0;
        while i < node.len() {
            let mi = node.as_u32().unwrap();
            m.material_index = mi;
            i += 1;
        }
        o.meshes.push(m);
        //println!("Mesh name {}", name.to_string());
        meshes_index += 1;
    }

    let mut materials_index = 0;

    while materials_index < parsed["model"]["meshes"].len() {
        let mut m = SerializedMaterial::new();
        let mut node = parsed["model"]["materials"][materials_index]["name"].clone();
        m.name = node.to_string();
        node = parsed["model"]["materials"][materials_index]["diffuse_texture_path"].clone();
        m.diffuse_texture_path = node.to_string();
        node = parsed["model"]["materials"][materials_index]["normal_texture_path"].clone();
        m.normals_texture_path = node.to_string();
        node = parsed["model"]["materials"][materials_index]["specular_texture_path"].clone();
        m.specular_texture_path = node.to_string();
        o.materials.push(m);

        materials_index += 1;
    }
    //println!("{:#}", parsed);

    o
}

// TODO: Robustify
pub async fn load_skinned_model_from_serialized(
    model: SerializedModel,
    default_material: Material,
    path: String,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> Option<SkinnedModel> {
    let mut model_results = SkinnedModel::new();
    for m in model.meshes {
        let mut verts = Vec::<ModelVertex>::new();
        let mut indices = Vec::<u32>::new();
        if m.positions.len() != m.normals.len() {
            println!("Not enough normals");
            return Option::None;
        }
        if m.positions.len() != m.uvs.len() {
            println!("Not enough UVs");
            return Option::None;
        }
        if m.positions.len() != m.bone_indices.len() {
            println!("Not enough bone indices");
            return Option::None;
        }
        if m.positions.len() != m.bone_weights.len() {
            println!("Not enough bone weights");
            return Option::None;
        }
        let mut i = 0;
        while i < m.positions.len() {
            let mut v = ModelVertex::new();
            v.position = m.positions[i];
            v.normal = m.normals[i];
            v.tex_coords = m.uvs[i];
            i += 1;
            verts.push(v);
        }
        i = 0;
        while i < m.indices.len() {
            indices.push(m.indices[i]);
            i += 1;
        }
        calculate_tangents_and_bitangents(&mut verts, &indices);

        let mut skinned_verts = Vec::<SkinnedModelVertex>::new();

        for (i, v) in verts.iter().enumerate() {
            let mut sv = SkinnedModelVertex::from_vert(&v);
            sv.bone_indices = m.bone_indices[i];
            sv.bone_weights = m.bone_weights[i];
            skinned_verts.push(sv);
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Skinned Vertex Buffer", m.name)),
            contents: bytemuck::cast_slice(&skinned_verts),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Skinned Index Buffer", m.name)),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        model_results.meshes.push(SkinnedTexturedMesh {
            name: m.name,
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material: MaterialIndex::new(m.material_index as usize),
            translation: glam::Vec3::from_array(m.translation),
            rotation: glam::Quat::from_array(m.rotation),
            scale: glam::Vec3::from_array(m.scale),
            dimensions: glam::Vec3::from_array(m.dimensions),
        });
    }

    if model.materials.len() == 0 {
        model_results.materials.push(default_material.clone());
    }

    for m in model.materials {
        //let mut mat = Material::new();
        // for material_serialized in materials_serialized {
        let name = m.name;
        let diffuse_texture: Texture;
        if m.diffuse_texture_path != "" {
            let full_path: String = path.clone() + "/" + &m.diffuse_texture_path;
            let diffuse_texture_result = load_texture(&full_path, false, device, queue).await;
            match diffuse_texture_result {
                Ok(value) => {
                    diffuse_texture = value;
                }
                Err(value) => {
                    println!(
                        "Could not load diffuse texture {}, error: {}",
                        m.diffuse_texture_path, value
                    );
                    diffuse_texture = default_material.diffuse_texture.clone();
                }
            }
        } else {
            diffuse_texture = default_material.diffuse_texture.clone();
        }

        let normal_texture: texture::Texture;
        if m.normals_texture_path != "" {
            let full_path: String = path.clone() + "/" + &m.normals_texture_path;

            let normal_texture_result = load_texture(&full_path, true, device, queue).await;
            match normal_texture_result {
                Ok(value) => {
                    normal_texture = value;
                }
                Err(value) => {
                    println!(
                        "Could not load normals texture {}, error: {}",
                        m.normals_texture_path, value
                    );
                    normal_texture = default_material.normal_texture.clone();
                }
            }
        } else {
            normal_texture = default_material.normal_texture.clone();
        }
        let material = Material::new(
            device,
            &name,
            diffuse_texture,
            normal_texture,
            texture_layout,
        );
        model_results.materials.push(material);
    }

    Some(model_results)
}
pub async fn load_model_from_serialized(
    model: SerializedModel,
    default_material: Material,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> Option<Model> {
    let mut model_results = Model::new();
    for m in model.meshes {
        let mut verts = Vec::<ModelVertex>::new();
        let mut indices = Vec::<u32>::new();
        if m.positions.len() != m.normals.len() {
            return Option::None;
        }
        if m.positions.len() != m.uvs.len() {
            return Option::None;
        }
        let mut i = 0;
        while i < m.positions.len() {
            let mut v = ModelVertex::new();
            v.position = m.positions[i];
            v.normal = m.normals[i];
            v.tex_coords = m.uvs[i];
            i += 1;
            verts.push(v);
        }
        i = 0;
        while i < m.indices.len() {
            indices.push(m.indices[i]);
            i += 1;
        }
        calculate_tangents_and_bitangents(&mut verts, &indices);

        // println!("Full path: {}", full_path);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", m.name)),
            contents: bytemuck::cast_slice(&verts),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", m.name)),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        model_results.meshes.push(TexturedMesh {
            name: m.name,
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material: MaterialIndex::new(m.material_index as usize),
            translation: glam::Vec3::from_array(m.translation),
            rotation: glam::Quat::from_array(m.rotation),
            scale: glam::Vec3::from_array(m.scale),
            dimensions: glam::Vec3::from_array(m.dimensions),
        });
    }

    if model.materials.len() == 0 {
        model_results.materials.push(default_material.clone());
    }

    for m in model.materials {
        //let mut mat = Material::new();
        // for material_serialized in materials_serialized {
        let name = m.name;
        let diffuse_texture: Texture;
        if m.diffuse_texture_path != "" {
            let diffuse_texture_result =
                load_texture(&m.diffuse_texture_path, false, device, queue).await;
            match diffuse_texture_result {
                Ok(value) => {
                    diffuse_texture = value;
                }
                Err(value) => {
                    println!(
                        "Could not load diffuse texture {}, error: {}",
                        m.diffuse_texture_path, value
                    );
                    diffuse_texture = default_material.diffuse_texture.clone();
                }
            }
        } else {
            diffuse_texture = default_material.diffuse_texture.clone();
        }

        let normal_texture: texture::Texture;
        if m.normals_texture_path != "" {
            let normal_texture_result =
                load_texture(&m.normals_texture_path, true, device, queue).await;
            match normal_texture_result {
                Ok(value) => {
                    normal_texture = value;
                }
                Err(value) => {
                    println!(
                        "Could not load normals texture {}, error: {}",
                        m.normals_texture_path, value
                    );
                    normal_texture = default_material.normal_texture.clone();
                }
            }
        } else {
            normal_texture = default_material.normal_texture.clone();
        }
        let material = Material::new(
            device,
            &name,
            diffuse_texture,
            normal_texture,
            texture_layout,
        );
        model_results.materials.push(material);
    }

    Some(model_results)
}

// //println!("Bone names: {:?}", bone_names);
// result_skinned.meshes.push(SkinnedTexturedMesh {
//     name,
//     vertex_buffer,
//     index_buffer,
//     num_elements: indices.len() as u32,
//     material: MaterialIndex::new(material_index as usize),
//     translation,
//     rotation,
//     scale,
//     dimensions,
//     //matrices_texture: Option::None,
// });

//     let diffuse_texture: texture::Texture;
//     if has_diffuse_map {
// for material_serialized in materials_serialized {
//     let name = material_serialized.get_name().unwrap().to_string().unwrap();
//     let has_diffuse_map: bool;
//     if material_serialized
//         .get_diffuse_t
//         let diffuse_texture_result = load_texture(&diffuse_path, false, device, queue).await;
//         match diffuse_texture_result {
//             Ok(value) => {
//                 diffuse_texture = value;
//             }
//             Err(value) => {
//                 println!(
//                     "Could not load diffuse texture {}, error: {}",
//                     diffuse_path, value
//                 );
//                 diffuse_texture = default_material.diffuse_texture.clone();
//             }
//         }
//     } else {
//         diffuse_texture = default_material.diffuse_texture.clone();
//     }

//     let normal_texture: texture::Texture;
//     if has_normals_map {
//         let normal_texture_result = load_texture(&normals_path, true, device, queue).await;
//         match normal_texture_result {
//             Ok(value) => {
//                 normal_texture = value;
//             }
//             Err(value) => {
//                 println!(
//                     "Could not load normals texture {}, error: {}",
//                     normals_path, value
//                 );
//                 normal_texture = default_material.normal_texture.clone();
//             }
//         }
//     } else {
//         normal_texture = default_material.normal_texture.clone();
//     }
//     let material = Material::new(
//         device,
//         &name,
//         diffuse_texture,
//         normal_texture,
//         texture_layout,
//     );
//     if !has_bones {
//         result_model.materials.push(material);
//     } else {
//         result_skinned.materials.push(material);
//     }
// }

// if materials_serialized.len() == 0 {
//     let material = default_material.clone();
//     if !has_bones {
//         result_model.materials.push(material);
//     } else {
//         result_skinned.materials.push(material);
//     }
// }
// if !has_bones {
//     result_model.name = filename;
// } else {
//     result_skinned.name = filename;
// }

// let result: GenericModel;
// if !has_bones {
//     result = GenericModel::Textured(result_model);
// } else {
//     result = GenericModel::SkinnedTextured(result_skinned);
// }
// Ok(result)
// }

fn calculate_tangents_and_bitangents(verts: &mut Vec<ModelVertex>, indices: &Vec<u32>) -> () {
    let mut triangles_included = vec![0; verts.len()];

    // Calculate tangents and bitangets. We're going to
    // use the triangles, so we need to loop through the
    // indices in chunks of 3
    for c in indices.chunks(3) {
        let v0 = verts[c[0] as usize];
        let v1 = verts[c[1] as usize];
        let v2 = verts[c[2] as usize];

        let pos0: glam::Vec3 = v0.position.into();
        let pos1: glam::Vec3 = v1.position.into();
        let pos2: glam::Vec3 = v2.position.into();

        let uv0: glam::Vec2 = v0.tex_coords.into();
        let uv1: glam::Vec2 = v1.tex_coords.into();
        let uv2: glam::Vec2 = v2.tex_coords.into();

        // Calculate the edges of the triangle
        let delta_pos1 = pos1 - pos0;
        let delta_pos2 = pos2 - pos0;

        // This will give us a direction to calculate the
        // tangent and bitangent
        let delta_uv1 = uv1 - uv0;
        let delta_uv2 = uv2 - uv0;

        // Solving the following system of equations will
        // give us the tangent and bitangent.
        //     delta_pos1 = delta_uv1.x * T + delta_u.y * B
        //     delta_pos2 = delta_uv2.x * T + delta_uv2.y * B
        // Luckily, the place I found this equation provided the solution!
        let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
        let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
        // We flip the bitangent to enable right-handed normal maps with wgpu texture coordinate system
        let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

        // We'll use the same tangent/bitangent for each vertex in the triangle
        verts[c[0] as usize].tangent =
            (tangent + glam::Vec3::from(verts[c[0] as usize].tangent)).into();
        verts[c[1] as usize].tangent =
            (tangent + glam::Vec3::from(verts[c[1] as usize].tangent)).into();
        verts[c[2] as usize].tangent =
            (tangent + glam::Vec3::from(verts[c[2] as usize].tangent)).into();
        verts[c[0] as usize].bitangent =
            (bitangent + glam::Vec3::from(verts[c[0] as usize].bitangent)).into();
        verts[c[1] as usize].bitangent =
            (bitangent + glam::Vec3::from(verts[c[1] as usize].bitangent)).into();
        verts[c[2] as usize].bitangent =
            (bitangent + glam::Vec3::from(verts[c[2] as usize].bitangent)).into();

        // Used to average the tangents/bitangents
        triangles_included[c[0] as usize] += 1;
        triangles_included[c[1] as usize] += 1;
        triangles_included[c[2] as usize] += 1;
    }
    // Average the tangents/bitangents
    for (i, n) in triangles_included.into_iter().enumerate() {
        let denom = 1.0 / n as f32;
        let v = &mut verts[i];
        v.tangent = (glam::Vec3::from(v.tangent) * denom).into();
        v.bitangent = (glam::Vec3::from(v.bitangent) * denom).into();
    }
}

pub async fn load_texture(
    file_name: &str,
    is_normal_map: bool,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<texture::Texture> {
    let data = load_binary(file_name).await?;
    texture::Texture::from_bytes(device, queue, &data, file_name, is_normal_map)
}

pub async fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let data = {
        let path = std::path::Path::new(file_name);
        std::fs::read(path)?
    };

    Ok(data)
}
