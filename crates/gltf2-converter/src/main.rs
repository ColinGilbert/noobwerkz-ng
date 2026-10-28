use gltf::json::serialize;
use msgpacker::*;
use noobwerkz_shared::{bone::*, serialized_model3d::*};
use std::{collections::{HashMap, HashSet}, fs::*, io::*};

fn traverse_node_recursive(
    node: &gltf::Node,
    parent_transform: glam::Mat4,
    meshes: &mut Vec<SerializedMesh3D>,
    buffers: &Vec<gltf::buffer::Data>,
) {
    // Process the current node (e.g., access its mesh, transform, etc.)
    println!("Node name {}", node.name().unwrap_or_default());
    let mesh = node.mesh();
    if mesh.is_some() {
        println!("Node {} has mesh", node.name().unwrap_or_default());
        let prims = mesh.unwrap().primitives();
        let mut serialized_mesh = SerializedMesh3D::new();
        let mut min_extents = [0.0 as f32; 3];
        let mut max_extents = [0.0 as f32; 3];
        for p in prims {
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
                            let pp = glam::Vec3::from_array(p);
                            let final_pos = pp * parent_transform.transform_point3(pp);
                            serialized_mesh.positions.push(final_pos.to_array());
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
                    if let Some(indices) = reader.read_indices() {
                        for i in indices.into_u32() {
                            serialized_mesh.indices.push(i);
                        }
                    }
                }
                gltf::mesh::Mode::TriangleStrip => {}
                gltf::mesh::Mode::TriangleFan => {}
            }
            let mat = p.material();
            match mat.index() {
                Some(index) => {
                    serialized_mesh.material_index = index as u32;
                }
                None => {}
            }
        }
        //println!("{:#?}", node.transform());
        let decomposed =
            parent_transform * glam::Mat4::from_cols_array_2d(&node.transform().matrix());
        let (scale, rot, trans) = decomposed.to_scale_rotation_translation();
        serialized_mesh.translation = trans.into();
        serialized_mesh.rotation = rot.into();
        serialized_mesh.scale = scale.into();

        let mut dims = [0.0 as f32; 3];
        let mut i = 0;
        while i < 3 {
            dims[i] = max_extents[i] - min_extents[i];
            i += 1;
        }
        serialized_mesh.max_extents = max_extents;
        serialized_mesh.min_extents = min_extents;
        serialized_mesh.dimensions = dims;
        meshes.push(serialized_mesh);
    }
    let trans = glam::Mat4::from_cols_array_2d(&node.transform().matrix()) * parent_transform;
    // Recursively visit all children
    for child in node.children() {
        traverse_node_recursive(&child, trans, meshes, buffers);
    }
}

fn run(path: &str) {
    let (document, buffers, _images) = gltf::import(path).unwrap();
    let mut scenes = document.scenes();
    let scene = scenes.next().unwrap();
    let scene_name = scene.name().unwrap_or_default();
    println!("Scene name: {}", scene_name);
    let roots = scene.nodes();
    let mut serialized_meshes = Vec::<SerializedMesh3D>::new();
    let mut serialized_materials = Vec::<SerializedModelMaterial>::new();

    println!("Roots length: {}", roots.len());
    for r in roots {
        traverse_node_recursive(&r, glam::Mat4::IDENTITY, &mut serialized_meshes, &buffers);
    }
    for mat in document.materials() {
        let mut serialized_material = SerializedModelMaterial::new();
        println!("ModelMaterial name: {} ", mat.name().unwrap_or_default());

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

                    let file_name = format!("texture_diffuse.{}", ext);
                    let mut file = std::fs::File::create(file_name.clone()).unwrap();
                    file.write_all(raw_bytes).unwrap();
                    println!("Saved diffuse texture to {}", file_name.clone());
                    serialized_material.diffuse_texture_path = file_name.clone();
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

                    let file_name = format!("texture_normal.{}", ext);
                    let mut file = std::fs::File::create(file_name.clone()).unwrap();
                    file.write_all(raw_bytes).unwrap();
                    println!("Saved normal texture to {}", file_name.clone());
                    serialized_material.normals_texture_path = file_name.clone();
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
    // let mut skin_counter = 0;
    let mut all_joints = Vec::<SerializedBone3D>::new();
    
    for skin in document.skins() {
        // We need this because we iterate and change over it
        let mut joints = Vec::<SerializedBone3D>::new();
        let mut skin_name = "";
        if skin.name().is_some() {
            skin_name = skin.name().unwrap();
        }
        let skin_skeleton = skin.skeleton();
        if skin_skeleton.is_some() {
            let skin_skeleton_name = skin_skeleton.unwrap().name();
            if skin_skeleton_name.is_some() {
                println!("Skin skeleton node name: {}", skin_skeleton_name.unwrap());
            }
            // let skin_skeleton_index = skin_skeleton.unwrap().index();
        }
        println!("Skin name: {:?}", skin_name);
        if let Some(ibm_accessor) = skin.inverse_bind_matrices() {
            inverse_bind_matrices =
                extract_matrices_from_accessor(&ibm_accessor, &buffers).unwrap();
            println!("IBM Matrices {:?}", inverse_bind_matrices);
        }
        let mut num_bones = 0usize;
        for joint_node in skin.joints() {
            if joint_node.name().is_some() {
                println!(
                    "  Joint (Bone) Name: {}, index: {}",
                    joint_node.name().unwrap(),
                    joint_node.index(),
                );
                println!("Joint children: ");
                joint_node.children().for_each(|f| {
                    println!(" {}", f.index());
                    joint_parents.insert(f.index(), joint_node.index());
                });

                joint_names.push(joint_node.name().unwrap().to_owned());
            } else {
                println!("  Joint ({}) has no name", joint_node.index().to_string());
                joint_names.push(joint_node.index().to_string());
            }
            let xform = glam::Mat4::from_cols_array_2d(&joint_node.transform().matrix())
                .to_scale_rotation_translation();

            let mut joint_parent = INVALID_BONE;
            if joint_parents.contains_key(&joint_node.index()) {
                joint_parent = joint_parents[&joint_node.index()]
            }

            let serialized_bone = SerializedBone3D {
                parent: joint_parent,
                scale: xform.0.to_array(),
                rotation: xform.1.to_array(),
                translation: xform.2.to_array(),
            };
            num_bones += 1;
            let joint_node_index = joint_node.index();
            joints.push(serialized_bone);
            let serialized_joint_index = joints.len() - 1;
            raw_joint_nodes_indices_to_serialized_indices
                .insert(joint_node_index, serialized_joint_index);
            serialized_indices_to_raw_joint_node_indices
                .insert(serialized_joint_index, joint_node_index);

            
        }
        println!("Num bones {}", num_bones);
        let mut joints_temp = joints.clone();
        all_joints.clear();
        for (i,j )in joints_temp.iter_mut().enumerate() {
            if raw_joint_nodes_indices_to_serialized_indices.contains_key(&j.parent) {
                j.parent = raw_joint_nodes_indices_to_serialized_indices[&j.parent];
            } else {
                j.parent = INVALID_BONE;
            }
            // 
            if serialized_indices_to_raw_joint_node_indices.contains_key(&i) {
                all_joints.push(j.clone());
            }
        }

        // for j in joints_temp {
        //     all_joints.push(j);
        // }
        println!("Serialized joints: ");
        for (i, j) in all_joints.iter().enumerate() {
            println!("    {}: {:?}", i, j);
        }
        // skin_counter += 1;
    }

    // Here we add the animations extracting code
    let anims = document.animations();
    let mut serialized_animations = Vec::<SerializedAnimation3D>::new();
    let mut anim_num = 0usize;
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
        animations_by_name.insert(anim_name, animation_index);
        animation_index += 1;
        let mut channel_num = 0usize;
        let mut serialized_animations_channels = Vec::new();
        for channel in anim.channels() {
            //channel.target().index();
            let s = channel.sampler();
            println!("Sampler interpolation: {:?}", s.interpolation());
            let target_bone_index =
                raw_joint_nodes_indices_to_serialized_indices[&channel.target().node().index()];

            println!("Target index {}", target_bone_index);

            let mut times = Vec::new();
            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
            let _keyframe_times = if let Some(inputs) = reader.read_inputs() {
                match inputs {
                    gltf::accessor::Iter::Standard(serialized_times) => {
                        times = serialized_times.collect();
                        println!("Num timestamps: {}", times.len());
                        println!("Timetamps: {:?}", times);
                    }
                    gltf::accessor::Iter::Sparse(_) => {
                        println!("Sparse keyframes not supported :(");
                    }
                }
            };
            let mut serialized_channels_keyframes: SerializedAnimation3DChannel =
                SerializedAnimation3DChannel {
                    bone: target_bone_index,
                    data: SerializedAnimation3DKeyframeTypes::Invalid,
                    times,
                };
            if let Some(outputs) = reader.read_outputs() {
                match outputs {
                    gltf::animation::util::ReadOutputs::Translations(translation) => {
                        let mut translations = Vec::new();
                        translation.for_each(|tr| {
                            // println!("Translation:");
                            // dbg!(tr);
                            translations.push(tr);
                        });
                        serialized_channels_keyframes.data =
                            SerializedAnimation3DKeyframeTypes::Translations(translations.clone());
                        println!(
                            "Anim {}, channel {}, keyframe translations: {}",
                            anim_num,
                            channel_num,
                            translations.len()
                        );
                    }
                    gltf::animation::util::ReadOutputs::Rotations(rotation) => {
                        let mut rotations = Vec::new();
                        rotation.into_f32().for_each(|rot| {
                            rotations.push(rot);
                        });
                        serialized_channels_keyframes.data =
                            SerializedAnimation3DKeyframeTypes::Rotations(rotations.clone());
                        println!(
                            "Anim {}, channel {}, keyframe rotations: {}",
                            anim_num,
                            channel_num,
                            rotations.len()
                        );
                    }
                    gltf::animation::util::ReadOutputs::Scales(scale) => {
                        let mut scales = Vec::new();
                        scale.for_each(|s| {
                            scales.push(s);
                        });
                        serialized_channels_keyframes.data =
                            SerializedAnimation3DKeyframeTypes::Scales(scales.clone());
                        println!(
                            "Anim {}, channel {}, keyframe scales: {}",
                            anim_num,
                            channel_num,
                            scales.len()
                        );
                    }
                    _ => {} // gltf::animation::util::ReadOutputs::MorphTargetWeights(_) => {},
                }
            };

            serialized_animations_channels.push(serialized_channels_keyframes.clone());
            println!(
                "Anim {}, channel {}, bone {}, bone name {}",
                anim_num,
                channel_num,
                serialized_channels_keyframes.clone().bone,
                joint_names[serialized_channels_keyframes.clone().bone]
            );
            channel_num += 1;
        }
        serialized_animations.push(SerializedAnimation3D {
            channels: serialized_animations_channels,
        });
        anim_num += 1;
    }

    let mut serialized_model = SerializedModel3D::new();
    serialized_model.meshes = serialized_meshes;
    serialized_model.materials = serialized_materials;
    serialized_model.bones = all_joints;
    serialized_model.bone_names = joint_names;
    serialized_model.inverse_bind_matrices = inverse_bind_matrices;
    serialized_model.animations = serialized_animations;
    serialized_model.animations_by_name = animations_by_name;

    let mut buf = Vec::new();
    let _n = serialized_model.pack(&mut buf);
    let mut file = File::create("model.bin").unwrap();
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

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        run(&path);
    } else {
        println!("usage: noobwerkz-gltf-converter <FILE>");
    }
}
