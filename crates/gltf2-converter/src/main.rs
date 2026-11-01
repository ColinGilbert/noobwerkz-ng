use clap::Parser;
use gltf::Semantic;
use msgpacker::*;
use noobwerkz_shared::{bone::*, serialized_model3d::*};
use std::{
    collections::{HashMap, HashSet},
    fs::*,
    io::*,
    str::FromStr,
};

fn traverse_node_recursive(
    node: &gltf::Node,
    parent_transform: glam::Mat4,
    meshes: &mut Vec<SerializedMesh3D>,
    node_index_to_name_map: &mut HashMap<usize, String>, // For debugging.
    buffers: &Vec<gltf::buffer::Data>,
) {
    // Process the current node (e.g., access its mesh, transform, etc.)
    println!(
        "Node name {}, index {}",
        node.name().unwrap_or_default(),
        node.index()
    );
    node_index_to_name_map.insert(node.index(), node.name().unwrap_or_default().to_owned());
    let mesh = node.mesh();
    let decomposed = parent_transform * glam::Mat4::from_cols_array_2d(&node.transform().matrix());
    if mesh.is_some() {
        // println!("Node {} has mesh", node.name().unwrap_or_default());
        let prims = mesh.unwrap().primitives();
        for p in prims {
            let mut serialized_mesh = SerializedMesh3D::new();
            let mut min_extents = [0.0 as f32; 3];
            let mut max_extents = [0.0 as f32; 3];
            println!("Primitive found: {}", p.index());
            let mode = p.mode();
            match mode {
                gltf::mesh::Mode::Points => {}
                gltf::mesh::Mode::Lines => {}
                gltf::mesh::Mode::LineLoop => {}
                gltf::mesh::Mode::LineStrip => {}
                gltf::mesh::Mode::Triangles => {
                    let reader = p.reader(|buffer| Some(&buffers[buffer.index()]));
                    if let Some(positions) = reader.read_positions() {
                        for p in positions {
                            serialized_mesh.positions.push(p);
                            let mut i = 0;
                            while i < 3 {
                                let biggest: f32;
                                if max_extents[i] > p[i] {
                                    biggest = max_extents[i];
                                } else {
                                    biggest = p[i];
                                }
                                max_extents[i] = biggest;

                                let smallest: f32;
                                if min_extents[i] < p[i] {
                                    smallest = min_extents[i];
                                } else {
                                    smallest = p[i];
                                }
                                min_extents[i] = smallest;

                                i += 1;
                            }
                        }
                    }
                    if let Some(normals) = reader.read_normals() {
                        for n in normals {
                            serialized_mesh.normals.push(n);
                        }
                    }
                    if let Some(uvs) = reader.read_tex_coords(0) {
                        for u in uvs.into_f32() {
                            serialized_mesh.uvs.push(u);
                        }
                    }
                    if let Some(bone_indices) = reader.read_joints(0) {
                        for bi in bone_indices.into_u16() {
                            let bi_u32 = [bi[0] as u32, bi[1] as u32, bi[2] as u32, bi[3] as u32];
                            serialized_mesh.bone_indices.push(bi_u32);
                        }
                    }
                    if let Some(weights) = reader.read_weights(0) {
                        for w in weights.into_f32() {
                            serialized_mesh.bone_weights.push(w);
                        }
                    }

                    let weight_set_count = p
                        .attributes()
                        .filter(|(semantic, _)| matches!(semantic, Semantic::Weights(_)))
                        .count();

                    println!(
                        "Primitive has {} set(s) of skinning weights.",
                        weight_set_count
                    );

                    if let Some(indices) = reader.read_indices() {
                        for i in indices.into_u32() {
                            serialized_mesh.indices.push(i);
                        }
                    }
                }
                gltf::mesh::Mode::TriangleStrip => {
                    println!("Triangle strip")
                }
                gltf::mesh::Mode::TriangleFan => {
                    println!("Triangle fan")
                }
            }
            let mat = p.material();
            match mat.index() {
                Some(index) => {
                    serialized_mesh.material_index = index as u32;
                }
                None => {}
            }

            let (scale, rot, trans) = decomposed.to_scale_rotation_translation();
            serialized_mesh.translation = trans.to_array();
            serialized_mesh.rotation = rot.to_array();
            serialized_mesh.scale = scale.to_array();

            let mut dims = [0.0 as f32; 3];
            for i in 0..3 {
                dims[i] = max_extents[i] - min_extents[i];
            }
            serialized_mesh.max_extents = max_extents;
            serialized_mesh.min_extents = min_extents;
            serialized_mesh.dimensions = dims;

            meshes.push(serialized_mesh);
        }
    }
    // Recursively visit all children
    for child in node.children() {
        traverse_node_recursive(&child, decomposed, meshes, node_index_to_name_map, buffers);
    }
}

fn run(input_path: &str, output_path: &str) {
    let (document, buffers, _images) = gltf::import(input_path).unwrap();
    let mut scenes = document.scenes();
    let scene = scenes.next().unwrap();
    let scene_name = scene.name().unwrap_or_default();
    println!("Scene name: {}", scene_name);
    let roots = scene.nodes();
    let mut serialized_meshes = Vec::<SerializedMesh3D>::new();
    let mut serialized_materials = Vec::<SerializedModelMaterial>::new();
    let mut node_index_to_name_map = HashMap::new();
    println!("Roots length: {}", roots.len());
    for r in roots {
        traverse_node_recursive(
            &r,
            glam::Mat4::IDENTITY,
            &mut serialized_meshes,
            &mut node_index_to_name_map,
            &buffers,
        );
    }
    for mat in document.materials() {
        let mut serialized_material = SerializedModelMaterial::new();
        println!("ModelMaterial name: {} ", mat.name().unwrap_or_default());

        serialized_material.base_colour = mat.pbr_metallic_roughness().base_color_factor();
        serialized_material.metallic = mat.pbr_metallic_roughness().metallic_factor();
        serialized_material.roughness = mat.pbr_metallic_roughness().roughness_factor();

        if let Some(pbr) = mat.pbr_metallic_roughness().base_color_texture() {
            let texture = document
                .textures()
                .nth(pbr.texture().index())
                .expect("Texture not found");

            let img = texture.source();
            match img.source() {
                gltf::image::Source::View { view, mime_type } => {
                    println!("Embedded diffuse texture MIME type: {}", mime_type);

                    let parent_buffer = view.buffer();
                    let buffer_data = &buffers[parent_buffer.index()].0;
                    let start = view.offset();
                    let end = start + view.length();
                    let raw_bytes = &buffer_data[start..end];
                    let ext = match mime_type {
                        "image/png" => "png",
                        "image/jpeg" => "jpg",
                        _ => ".bin",
                    };

                    let mut truncated_path = std::path::PathBuf::from_str(&output_path).unwrap();
                    let file_name = format!("{}-texture_diffuse.{}", truncated_path.clone().file_stem().unwrap().to_string_lossy(), ext);
                    truncated_path.pop();
                    truncated_path.push(file_name);
                    let mut file = std::fs::File::create(truncated_path.clone()).unwrap();
                    file.write_all(raw_bytes).unwrap();
                    println!(
                        "Saved diffuse texture to {}",
                        truncated_path.clone().display()
                    );

                    serialized_material.diffuse_texture_path =
                        truncated_path.clone().file_name().unwrap().to_string_lossy().to_string();
                }
                gltf::image::Source::Uri { uri, mime_type } => {
                    println!(
                        "External diffuse texture URI: {}, MIME type: {}",
                        uri,
                        mime_type.unwrap_or_default()
                    );
                    serialized_material.diffuse_texture_path = uri.to_owned();
                }
            }
        }

        if let Some(normal_texture) = mat.normal_texture() {
            let temp = normal_texture.texture();
            let image = temp.source();
            match image.source() {
                gltf::image::Source::View { view, mime_type } => {
                    println!("Embedded diffuse texture MIME type: {}", mime_type);

                    let parent_buffer = view.buffer();
                    let buffer_data = &buffers[parent_buffer.index()].0;
                    let start = view.offset();
                    let end = start + view.length();
                    let raw_bytes = &buffer_data[start..end];
                    let ext = match mime_type {
                        "image/png" => "png",
                        "image/jpeg" => "jpg",
                        _ => ".bin",
                    };

                    let mut truncated_path = std::path::PathBuf::from_str(&output_path).unwrap();
                    let file_name = format!("{}-texture_normal.{}", truncated_path.clone().file_stem().unwrap().to_string_lossy(), ext);
                    truncated_path.pop();
                    truncated_path.push(file_name);
                    let mut file = std::fs::File::create(truncated_path.clone()).unwrap();
                    file.write_all(raw_bytes).unwrap();
                    println!(
                        "Saved normal texture to {}",
                        truncated_path.clone().display()
                    );
                    serialized_material.normals_texture_path =
                        truncated_path.clone().file_name().unwrap().to_string_lossy().to_string();
                }
                gltf::image::Source::Uri { uri, mime_type } => {
                    println!(
                        "External normals texture URI: {}, MIME type: {}",
                        uri,
                        mime_type.unwrap_or_default()
                    );
                    serialized_material.normals_texture_path = uri.to_owned();
                }
            }
        }
        serialized_materials.push(serialized_material);
    }
    let mut joint_names = Vec::<String>::new();
    let mut joint_parents = HashMap::<usize, usize>::new();
    let mut inverse_bind_matrices = Vec::new();
    let mut raw_joint_nodes_indices_to_serialized_indices: HashMap<usize, usize> =
        HashMap::<usize, usize>::new();
    let mut serialized_indices_to_raw_joint_node_indices: HashMap<usize, usize> =
        HashMap::<usize, usize>::new();
    let mut joints = Vec::<SerializedBone3D>::new();
    let mut joint_nodes_index_set = HashSet::new();
    for skin in document.skins() {
        let skin_skeleton = skin.skeleton();
        if skin_skeleton.is_some() {
            let skin_skeleton_name = skin_skeleton.unwrap().name();
            if skin_skeleton_name.is_some() {
                println!("Skin skeleton node name: {}", skin_skeleton_name.unwrap());
            }
            // let skin_skeleton_index = skin_skeleton.unwrap().index();
        }
        // println!("Skin name: {:?}", skin_name);
        if let Some(ibm_accessor) = skin.inverse_bind_matrices() {
            let inverse_bind_matrices_temp =
                extract_matrices_from_accessor(&ibm_accessor, &buffers).unwrap();

            // Validate whether or not the inverse bind matrices differ from one skin to the next.
            for (a, b) in inverse_bind_matrices
                .iter()
                .zip(inverse_bind_matrices_temp.iter())
            {
                if a != b {
                    println!("Inverse bind matrices mismatch! {:?} != {:?}", a, b);
                }
            }
            inverse_bind_matrices = inverse_bind_matrices_temp;
            println!("IBM Matrices length: {}", inverse_bind_matrices.len());
            // println!("IBM Matrices {:?}", inverse_bind_matrices);
        }
        // let mut num_bones = 0usize;
        for joint_node in skin.joints() {
            let joint_name;
            if joint_node.name().is_some() {
                // println!("Joint children: ");
                joint_node.children().for_each(|f| {
                    // println!(" {}", f.index());
                    joint_parents.insert(f.index(), joint_node.index());
                });
                joint_name = joint_node.name().unwrap().to_owned();
                joint_names.push(joint_name.clone());
            } else {
                // println!("  Joint ({}) has no name", joint_node.index().to_string());
                joint_name = joint_node.index().to_string();
                joint_names.push(joint_name.clone());
            }

            let mut joint_parent = INVALID_BONE;
            if joint_parents.contains_key(&joint_node.index()) {
                joint_parent = joint_parents[&joint_node.index()]
            }
            let (xform_scale, xform_rot, xform_trans) =
                glam::Mat4::from_cols_array_2d(&joint_node.transform().matrix())
                    .to_scale_rotation_translation();

            let serialized_bone = SerializedBone3D {
                parent: joint_parent,
                scale: xform_scale.to_array(),
                rotation: xform_rot.to_array(),
                translation: xform_trans.to_array(),
            };
            let joint_node_index = joint_node.index();
            if !joint_nodes_index_set.contains(&joint_node_index) {
                joints.push(serialized_bone);
                let serialized_joint_index = joints.len() - 1;
                raw_joint_nodes_indices_to_serialized_indices
                    .insert(joint_node_index, serialized_joint_index);
                serialized_indices_to_raw_joint_node_indices
                    .insert(serialized_joint_index, joint_node_index);
                println!(
                    "Inserting joint {} index {} to serialized index {} ",
                    joint_name.clone(),
                    joint_node_index,
                    serialized_joint_index
                );
                joint_nodes_index_set.insert(joint_node_index);
            }
        }
    }
    let mut all_joints = joints.clone();
    for j in all_joints.iter_mut() {
        if joint_nodes_index_set.contains(&j.parent) {
            j.parent = raw_joint_nodes_indices_to_serialized_indices[&j.parent];
        } else {
            println!("Joint parent not found. {}", j.parent);
            j.parent = INVALID_BONE;
        }
    }
    let mut temp_set = HashSet::<usize>::new();
    // Now we redo the bone indices for the meshes:
    for m in &mut serialized_meshes {
        for bi in m.bone_indices.iter() {
            for i in 0..4 {
                let old_index = bi[i] as usize;
                if !joint_nodes_index_set.contains(&old_index) {
                    if !temp_set.contains(&old_index) {
                        println!(
                            "Old bone index {} without a new index. Node name for old index: {}",
                            old_index, node_index_to_name_map[&old_index]
                        );
                        temp_set.insert(old_index);
                    }
                }
            }
        }
    }

    // Here we validate for non-normalized bone weights
    for m in &mut serialized_meshes {
        for bw in m.bone_weights.iter() {
            let mut accum = 0.0;
            for i in 0..4 {
                accum += bw[i];
            }
            if accum > 1.00001 || accum < 0.99999 {
                println!("Bone weights not normalized");
            }
        }
    }

    // Here we add the animations extracting code
    let anims = document.animations();
    let mut serialized_animations = Vec::<SerializedAnimation3D>::new();
    // let mut anim_num = 0usize;
    println!("Num anims: {}", anims.len());
    let mut anim_name_counter = 0;
    let mut animation_index = 0usize;
    let mut animations_by_name = HashMap::new();
    for anim in anims {
        let anim_name: String;
        match anim.name() {
            Some(val) => anim_name = val.to_owned(),
            None => {
                anim_name = anim_name_counter.to_string();
                anim_name_counter += 1;
            }
        }
        animations_by_name.insert(anim_name.clone(), animation_index);
        animation_index += 1;
        println!("Animation name: {}", anim_name.clone());
        // let mut channel_num = 0usize;
        let mut serialized_animations_channels = Vec::new();
        for channel in anim.channels() {
            let target_bone_index =
                raw_joint_nodes_indices_to_serialized_indices[&channel.target().node().index()];

            let mut times = Vec::new();
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let _keyframe_times = if let Some(inputs) = reader.read_inputs() {
                match inputs {
                    gltf::accessor::Iter::Standard(serialized_times) => {
                        times = serialized_times.collect();
                        // println!("Num timestamps: {}", times.len());
                        // println!("Timetamps: {:?}", times);
                    }
                    gltf::accessor::Iter::Sparse(_) => {
                        println!("Sparse keyframes not supported :(");
                    }
                }
            };
            let mut serialized_channels_keyframes = SerializedAnimation3DChannel {
                bone: target_bone_index,
                data: SerializedAnimation3DKeyframeTypes::Invalid,
                times,
            };
            if let Some(outputs) = reader.read_outputs() {
                match outputs {
                    gltf::animation::util::ReadOutputs::Translations(translation) => {
                        let mut translations = Vec::new();
                        translation.for_each(|tr| {
                            translations.push(tr);
                        });
                        serialized_channels_keyframes.data =
                            SerializedAnimation3DKeyframeTypes::Translations(translations.clone());
                    }
                    gltf::animation::util::ReadOutputs::Rotations(rotation) => {
                        let mut rotations = Vec::new();
                        rotation.into_f32().for_each(|rot| {
                            rotations.push(rot);
                        });
                        serialized_channels_keyframes.data =
                            SerializedAnimation3DKeyframeTypes::Rotations(rotations.clone());
                    }
                    gltf::animation::util::ReadOutputs::Scales(scale) => {
                        let mut scales = Vec::new();
                        scale.for_each(|s| {
                            scales.push(s);
                        });
                        serialized_channels_keyframes.data =
                            SerializedAnimation3DKeyframeTypes::Scales(scales.clone());
                    }
                    gltf::animation::util::ReadOutputs::MorphTargetWeights(_) => {
                        println!("Morph weights not supported");
                    }
                }
            };
            serialized_animations_channels.push(serialized_channels_keyframes.clone());
        }
        serialized_animations.push(SerializedAnimation3D {
            channels: serialized_animations_channels.clone(),
        });
    }

    let mut serialized_model = SerializedModel3D::new();
    serialized_model.meshes = serialized_meshes;
    serialized_model.materials = serialized_materials;
    serialized_model.bones = all_joints;
    serialized_model.bone_names = joint_names;
    serialized_model.inverse_bind_matrices = inverse_bind_matrices;
    serialized_model.animations = serialized_animations;
    serialized_model.animations_by_name = animations_by_name;
    serialized_model.global_inverse_transform = glam::Mat4::IDENTITY.to_cols_array_2d();

    let mut buf = Vec::new();
    let _n = serialized_model.pack(&mut buf);
    let mut file = File::create(output_path).unwrap();
    let results = file.write_all(&buf);
    match results {
        Ok(_data) => {
            println!("Writing file")
        }
        Err(e) => {
            println!("Error writing file: {}", e)
        }
    }
}

fn extract_matrices_from_accessor(
    accessor: &gltf::Accessor,
    buffers: &[gltf::buffer::Data],
) -> anyhow::Result<Vec<[[f32; 4]; 4]>> {
    // Ensure the accessor data is in the correct format (4x4 float matrices)
    let view = accessor
        .view()
        .ok_or(anyhow::anyhow!("Accessor has no buffer view"))?;
    if accessor.data_type() == gltf::accessor::DataType::F32
        && accessor.dimensions() == gltf::accessor::Dimensions::Mat4
    {
        let buffer_data = &buffers[view.buffer().index()];
        let start_offset = view.offset() + accessor.offset();
        let stride = view.stride().unwrap_or(accessor.size());
        let count = accessor.count();

        let mut matrices = Vec::with_capacity(count);

        for i in 0..count {
            let byte_offset = start_offset + i * stride;
            let bytes = &buffer_data[byte_offset..byte_offset + 64];

            let matrix: [[f32; 4]; 4] = unsafe { std::ptr::read(bytes.as_ptr() as *const _) };
            matrices.push(matrix);
        }

        Ok(matrices)
    } else {
        Err(anyhow::anyhow!("Invalid buffer format"))
    }
}
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    pub input: String,
    pub output: String,
}

fn main() {
    let args = Args::parse();
    run(args.input.as_str(), args.output.as_str());
}
