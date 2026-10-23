use crate::index_types::*;
use crate::material::*;
use crate::model3d::*;
use crate::skinned_model3d::*;
use crate::texture;
use crate::texture::Texture;
use msgpacker::*;
use noobwerkz_shared::bone::Bone;
use noobwerkz_shared::item3d_common::ItemStatus::InWorld;
use noobwerkz_shared::serialized_model3d::*;
use noobwerkz_shared::skeleton::Skeleton;
use noobwerkz_skeletal_animation::animation::Animation;
use noobwerkz_skeletal_animation::animation_channel::AnimationChannel;
use noobwerkz_skeletal_animation::animation_graph::AnimationIndex;
use wgpu::wgc::command::ComputePassErrorInner::BindGroupIndexOutOfRange;
use std::path::*;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use image::*;

pub fn load_serialized_model(filepath: &std::path::Path) -> Option<SerializedModel3D> {
    let mut path = PathBuf::new();
    for p in filepath {
        path.push(p);
    }
    println!("Full path {:?}", path.as_path());
    let data = std::fs::read(path.as_path()).unwrap();
    let deserialized_results = SerializedModel3D::unpack(&data);

    match deserialized_results {
        Ok(deserialized) => {
            println!("Dimensions {:?}", deserialized.meshes[0].dimensions);
            println!("Scale {:?}", deserialized.meshes[0].scale);
            println!("Translation {:?}", deserialized.meshes[0].translation);
            println!("Rotation {:?}", deserialized.meshes[0].rotation);
            println!(
                "Diffuse texture {}",
                deserialized.materials[0].diffuse_texture_path
            );
            println!(
                "Normals texture {}",
                deserialized.materials[0].normals_texture_path
            );

            Some(deserialized)
        }
        Err(err) => {
            println!("Error loading 3D model: {}", err);
            None
        }
    }
}

// TODO: Robustify
pub fn load_skinned_3d_model_from_serialized(
    model: &mut SerializedModel3D,
    default_material: &Material,
    path: &std::path::Path,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> Option<SkinnedModel3D> {
    let mut model_results = SkinnedModel3D::new();
    for m in model.meshes.iter_mut() {
        println!("Mesh {}", m.name);
        let mut verts = Vec::<Model3DVertex>::new();
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
            m.bone_weights
                .resize(m.positions.len(), [0.0, 0.0, 0.0, 0.0]);
            //return Option::None;
        }
        let mut i = 0;

        // let matrix = glam::Mat4::from_quat(glam::Quat::from_axis_angle(glam::Vec3{x: 1.0, y: 0.0, z: 0.0}, 90.0));
        while i < m.positions.len() {
            let mut v = Model3DVertex::new();
            v.position = m.positions[i];
            v.normal = m.normals[i];
            v.tex_coords = [m.uvs[i][0], 1.0 as f32 - m.uvs[i][1]];
            i += 1;
            verts.push(v);
        }

        i = 0;
        while i < m.indices.len() {
            indices.push(m.indices[i]);
            i += 1;
        }

        calculate_tangents_and_bitangents(&mut verts, &indices);

        let mut skinned_verts = Vec::<SkinnedModelVertex3D>::new();

        // let skeleton = Skeleton::new();

        for (i, v) in verts.iter().enumerate() {
            let mut sv = SkinnedModelVertex3D::from_vert(&v);
            let mut bi = [0; 4]; //m.bone_indices[i];
            for (ii, bone_index) in m.bone_indices[i].iter().enumerate() {
                // let bone_name = model.bone_names[*bone_index as usize].clone();
                // let new_index = model.bones[*bone_index as usize];
                // .bone_names_to_idx[&bone_name];
                bi[ii] = *bone_index as u32;
            }
            sv.bone_indices = bi;
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

        model_results.meshes.push(SkinnedTexturedMesh3D {
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
            let mut full_path = PathBuf::new();
            for p in path {
                full_path.push(p);
            }
            full_path.push(&m.diffuse_texture_path);
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
            let mut full_path = PathBuf::new();
            for p in path {
                full_path.push(p);
            }
            full_path.push(&m.normals_texture_path);
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

    let mut skeleton = Skeleton::new();

    // let mut bone_names_reshuffled = Vec::<String>::new();
    // bone_names_reshuffled.resize(model.bones.len() - 1, "".to_owned());

    /*
        pub struct Skeleton
        {
            pub bones: [Bone; MAX_BONES],    // These store local transforms
            pub parents: [usize; MAX_BONES], // These store the parent bone indices.
            pub valid: [bool; MAX_BONES],
            pub inverse_bind_matrices: [glam::Mat4; MAX_BONES],
            pub num_bones: usize,
            pub bone_names_to_idx: HashMap<String, usize>,
            pub bone_idx_to_names: HashMap<usize, String>,
        }

        pub struct SerializedModel3D
        {
            pub meshes: Vec<SerializedMesh3D>,
            pub materials: Vec<SerializedMaterial>,
            pub bones: Vec<SerializedBone3D>,
            pub bone_names: Vec<String>,
            pub inverse_bind_matrices: Vec<[[f32; 4]; 4]>,
            pub animations: Vec<SerializedAnimation3D>,
            pub animations_by_name: HashMap<String, usize>
        }

        */

    
    for b in &model.bones {
        let mut bone = Bone::new();
        bone.position = glam::Vec3::from_array(b.translation);
        bone.orientation = glam::Quat::from_array(b.rotation);
        bone.scale = glam::Vec3::from_array(b.scale);
        skeleton.bones.push(bone);
        skeleton.parents.push(b.parent);
    }

    skeleton.num_bones = model.bones.len();

    for (i, b) in model.bone_names.iter().enumerate() {
        skeleton.bone_names_to_idx.insert(b.clone(), i);
        skeleton.bone_idx_to_names.insert(i, b.clone());
    }
    // for bone_idx in 0..model.bone_names.len() {
    //     let bone_name = &model.bone_names[bone_idx].clone();
    //     println!("Searching for new position for {}", bone_name);
    //     let bone_newpos = skeleton.bone_names_to_idx[bone_name];
    //     inverse_bind_poses[bone_newpos] = glam::Mat4::from_cols_array_2d(&model.inverse_bind_matrices[bone_idx]);
    //     // bone_names_reshuffled[bone_newpos] = bone_name.to_string();
    // }

    println!("Bones              {:?}", model.bone_names);
    // println!("Bones (reshuffled) {:?}", bone_names_reshuffled);

    for ibp in &model.inverse_bind_matrices {
        skeleton
            .inverse_bind_matrices
            .push(glam::Mat4::from_cols_array_2d(ibp));
    }

    let mut anim_channels = Vec::new();

    for a in model.animations.clone() {
        for s in &a.channels {
            let channel = AnimationChannel::from_serialized(&s);
            match channel {
                Some(val) => {
                    anim_channels.push(val);
                }
                None => {
                    println!("Invalid animation channel detected.");
                    return None;
                }
            }
        }

        let animation = Animation::new(anim_channels.clone(), skeleton.num_bones);

        model_results.animations.push(animation.clone());
    }

    for (n, i) in &model.animations_by_name {
        model_results
            .animation_names
            .insert(AnimationIndex::new(*i), n.clone());
        model_results
            .animations_idx_by_name
            .insert(n.clone(), AnimationIndex::new(*i));
    }

    println!("Animation names {:?},", model_results.animation_names);



    model_results.skeleton = Arc::new(skeleton);

    Some(model_results)
}

pub fn load_3d_model_from_serialized(
    model: &mut SerializedModel3D,
    default_material: &Material,
    filepath: &std::path::Path,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> Option<Model3D> {
    let mut model_results = Model3D::new();
    for m in model.meshes.iter_mut() {
        let mut verts = Vec::<Model3DVertex>::new();
        let mut indices = Vec::<u32>::new();
        if m.positions.len() != m.normals.len() {
            return Option::None;
        }
        if m.positions.len() != m.uvs.len() {
            m.uvs.resize(m.positions.len(), [0.0, 0.0]);
        }
        let mut i = 0;
        while i < m.positions.len() {
            let mut v = Model3DVertex::new();
            v.position = m.positions[i];
            v.normal = m.normals[i];
            v.tex_coords = [m.uvs[i][0], 1.0 - m.uvs[i][1]];
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

        model_results.meshes.push(TexturedMesh3D {
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
            let mut path = PathBuf::new();
            for p in filepath {
                path.push(p);
            }
            path.push(&m.diffuse_texture_path);
            let diffuse_path = path.as_path();
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
        // let normals_path = path.clone() + &"/".to_owned() + &m.normals_texture_path;
        if m.normals_texture_path != "" {
            let mut path = PathBuf::new();
            for p in filepath {
                path.push(p);
            }
            path.push(&m.normals_texture_path);
            println!("Normal texture path {:?}", path);
            let normal_texture_result =
                futures::executor::block_on(load_texture(path.as_path(), true, device, queue));
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
            name,
            diffuse_texture,
            normal_texture,
            texture_layout,
        );
        model_results.materials.push(material);
    }

    Some(model_results)
}

pub async fn load_texture(
    filepath: &Path,
    is_normal_map: bool,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<texture::Texture> {
    let data = load_binary(filepath).await?;
    texture::Texture::from_bytes(
        device,
        queue,
        &data,
        filepath.to_str().expect("Filepath must exist"),
        is_normal_map,
    )
}

pub async fn load_binary(filepath: &Path) -> anyhow::Result<Vec<u8>> {
    let data = { ImageReader::open(filepath)? }.with_guessed_format()?.decode()?;

    Ok(data)
}
