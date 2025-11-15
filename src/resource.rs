use crate::index_types::*;
use crate::material::*;
use crate::model::*; //{Material, MaterialIndex, Model, ModelVertex, TexturedMesh};
use crate::serialized_model::*;
use crate::skeletal_context::SkeletalContext;
use crate::skinned_model::*;
use crate::texture;
use crate::texture::Texture;
use msgpacker::prelude::*;
use wgpu::util::DeviceExt;


pub fn load_serialized_model(filepath: String, filename: String) -> SerializedModel {
    let full_path = filepath.clone() + "/" + &filename;
    println!("Full path {}", full_path);
    let data = std::fs::read(full_path).unwrap();
    let (_n, deserialized) = SerializedModel::unpack(&data).unwrap();
    println!("Dimensions {:?}", deserialized.meshes[0].dimensions);
    println!("Scale {:?}", deserialized.meshes[0].scale);
    println!("Translation {:?}", deserialized.meshes[0].translation);
    println!("Rotation {:?}", deserialized.meshes[0].rotation);
    println!("Diffuse texture {}", deserialized.materials[0].diffuse_texture_path);
    //println!("Inverse bind pose {:?}", deserialized.inverse_bind_matrices);
    deserialized
}

// TODO: Robustify
pub fn load_skinned_model_from_serialized(
    model: &mut SerializedModel,
    default_material: Material,
    path: String,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
    skeletal_context: &SkeletalContext,
) -> Option<SkinnedModel> {
    let mut model_results = SkinnedModel::new();
    for m in model.meshes.iter_mut() {
        println!("Mesh {}", m.name);
        let mut verts = Vec::<ModelVertex>::new();
        let mut indices = Vec::<u32>::new();
        if m.positions.len() != m.normals.len() {
            println!("Not enough normals");
            // TODO: Generate normals
            return Option::None;
        }
        if m.positions.len() != m.uvs.len() {
            println!("Not enough UVs");
            m.uvs.resize(m.positions.len(), [0.0, 0.0]);
        }
        if m.positions.len() != m.bone_indices.len() {
            m.bone_indices.resize(m.positions.len(), [0, 0, 0, 0]);
            //return Option::None;
        }
        if m.positions.len() != m.bone_weights.len() {
            m.bone_weights.resize(m.positions.len(), [0.0, 0.0, 0.0, 0.0]);
            //return Option::None;
        }
        let mut i = 0;
        
        // let matrix = glam::Mat4::from_quat(glam::Quat::from_axis_angle(glam::Vec3{x: 1.0, y: 0.0, z: 0.0}, 90.0));
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
            let mut bi = [0; 4]; //m.bone_indices[i];
            for (ii, bone_index) in m.bone_indices[i].iter().enumerate() {
                let bone_name = model.bone_names[*bone_index as usize].clone();
                let new_index = skeletal_context.skeleton.joint_by_name(&bone_name).unwrap();
                bi[ii] = new_index as u32;
            }
            sv.bone_indices = bi;
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
            name: m.name.clone(),
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

    for m in &model.materials {
        let name = &m.name;
        let diffuse_texture: Texture;
        if m.diffuse_texture_path != "" {
            let full_path: String = path.clone() + "/" + &m.diffuse_texture_path;
            let diffuse_texture_result =
                futures::executor::block_on(load_texture(&full_path, false, device, queue));
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

            let normal_texture_result =
                futures::executor::block_on(load_texture(&full_path, true, device, queue));
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

    let mut inverse_bind_poses = Vec::<[[f32; 4]; 4]>::new();
    inverse_bind_poses.resize(skeletal_context.skeleton.num_joints(), [[0.0; 4]; 4]);

    let mut bone_names_reshuffled = Vec::<String>::new();
    bone_names_reshuffled.resize(skeletal_context.skeleton.num_joints(), "".to_owned());

    for bone_idx in 0..model.bone_names.len() {
        let bone_name = &model.bone_names[bone_idx].clone();
        let bone_newpos = skeletal_context.skeleton.joint_by_name(&bone_name).unwrap() as usize;
        inverse_bind_poses[bone_newpos] = model.inverse_bind_matrices[bone_idx];
        bone_names_reshuffled[bone_newpos] = bone_name.to_string();
    }

    // println!("Bones              {:?}", model.bone_names);
    // !("Bones (reshuffled) {:?}", bone_names_reshuffled);

    for ibp in inverse_bind_poses {
        model_results
            .inverse_bind_matrices
            .push(glam::Mat4::from_cols_array_2d(&ibp));
    }

    Some(model_results)
}

pub fn load_model_from_serialized(
    model: &mut SerializedModel,
    default_material: Material,
    path: String,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> Option<Model> {
    let mut model_results = Model::new();
    for m in model.meshes.iter_mut() {
        let mut verts = Vec::<ModelVertex>::new();
        let mut indices = Vec::<u32>::new();
        if m.positions.len() != m.normals.len() {
            return Option::None;
        }
        if m.positions.len() != m.uvs.len() {
            m.uvs.resize(m.positions.len(), [0.0, 0.0]);
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
            name: m.name.clone(),
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

    for m in model.materials.iter_mut() {
        let name = &m.name;
        let diffuse_texture: Texture;
        if m.diffuse_texture_path != "" {
            let diffuse_path = path.clone() + &"/".to_owned() + &m.diffuse_texture_path;
            let diffuse_texture_result =
                futures::executor::block_on(load_texture(&diffuse_path, false, device, queue));
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
        let normals_path = path.clone() + &"/".to_owned() + &m.normals_texture_path;
        if m.normals_texture_path != "" {
            let normal_texture_result =
                futures::executor::block_on(load_texture(&normals_path, true, device, queue));
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
