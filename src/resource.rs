use crate::material::*;
use crate::model::*; //{Material, MaterialIndex, Model, ModelVertex, TexturedMesh};
use crate::skinned_model::*;
use crate::texture;
// use ozz_animation_rs::*;
use serde::{Deserialize, Serialize};
use std::fs;

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
#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SerializedMaterial {
    name: String,
    diffuse_texture_path: String,
    normals_texture_path: String,
    specular_texture_path: String,
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

pub fn load_model_from_json(
    filepath: String,
    filename: String,
    default_material: Material,
) -> SerializedModel {
    let mut o = SerializedModel::new();
    let full_path = filepath.clone() + "/" + &filename;
    // println!("Full path: {}", full_path);
    let data = fs::read_to_string(full_path).unwrap();
    let parsed = json::parse(&data).unwrap();
    let mut meshes_index = 0;

    while meshes_index < parsed["meshes"].len() {
        let name = &parsed["meshes"][meshes_index]["name"];
        println!("Mesh name {}", name.to_string());
        meshes_index += 1;
    }
    println!("{:#}", parsed);

    o
}

// TODO: Robustify
pub async fn load_model_from_serialized(
    filepath: String,
    filename: String,
    default_material: Material,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> Option<Model> {
    let full_path = filepath.clone() + "/" + &filename;
    // println!("Full path: {}", full_path);
    let data = fs::read(full_path).unwrap();
    let deserialized_data: SerializedModel =
        rmp_serde::from_slice(&data).expect("Serialized model did not load");
    // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some(&format!("{:?} Vertex Buffer", name)),
    //     contents: bytemuck::cast_slice(&verts),
    //     usage: wgpu::BufferUsages::VERTEX,
    // });
    // let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some(&format!("{:?} Index Buffer", name)),
    //     contents: bytemuck::cast_slice(&indices),
    //     usage: wgpu::BufferUsages::INDEX,
    // });

    // result_model.meshes.push(TexturedMesh {
    //     name,
    //     vertex_buffer,
    //     index_buffer,
    //     num_elements: indices.len() as u32,
    //     material: MaterialIndex::new(material_index as usize),
    //     translation,
    //     rotation,
    //     scale,
    //     dimensions,
    // });

    // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some(&format!("{:?} Skinned Vertex Buffer", name)),
    //     contents: bytemuck::cast_slice(&verts_skinned),
    //     usage: wgpu::BufferUsages::VERTEX,
    // });

    // let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some(&format!("{:?} Skinned Index Buffer", name)),
    //     contents: bytemuck::cast_slice(&indices),
    //     usage: wgpu::BufferUsages::INDEX,
    // });

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
    let model = Model::new();
    Some(model)
}

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
