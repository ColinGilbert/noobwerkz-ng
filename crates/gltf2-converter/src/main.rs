use msgpacker::*;
use noobwerkz_shared::serialized_model3d::*;
use std::{collections::HashMap, fs::*, io::*};

fn traverse_node_recursive(
    node: &gltf::Node,
    parent_transform: glam::Mat4,
    meshes: &mut Vec<SerializedMesh3D>,
    buffers: &Vec<gltf::buffer::Data>,
) {
    // Process the current node (e.g., access its mesh, transform, etc.)
    // ...
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
                            serialized_mesh.positions.push([p[0], p[1], p[2]]);
                            // let pp = parent_transform
                            //     * glam::Mat4::from_cols_array_2d(&node.transform().matrix())
                            //     * glam::Vec4::new(p[0], p[1], p[2], 1.0);
                            // serialized_mesh.positions.push([pp[0], pp[1], pp[2]]);
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
    let trans = parent_transform * glam::Mat4::from_cols_array_2d(&node.transform().matrix());
    // Recursively visit all children
    for child in node.children() {
        traverse_node_recursive(&child, trans, meshes, buffers);
    }
}

fn run(path: &str) {
    //let file = fs::File::open(path);
    // let reader = io::BufReader::new(file);
    let (document, buffers, images) = gltf::import(path).unwrap();
    // println!("{:#?}", gltf);
    let mut scenes = document.scenes();
    let scene = scenes.next().unwrap();
    let scene_name = scene.name().unwrap_or_default();
    println!("Scene name: {}", scene_name);
    let roots = scene.nodes();
    let mut serialized_meshes = Vec::<SerializedMesh3D>::new();
    let mut serialized_materials = Vec::<SerializedMaterial>::new();

    //let mat = invert_x * invert_z;
    for r in roots {
        traverse_node_recursive(&r, glam::Mat4::IDENTITY, &mut serialized_meshes, &buffers);
    }
    for mat in document.materials() {
        let mut serialized_material = SerializedMaterial::new();
        println!("Material name: {} ", mat.name().unwrap_or_default());

        if let Some(pbr) = mat.pbr_metallic_roughness().base_color_texture() {
            let texture = document
                .textures()
                .nth(pbr.texture().index())
                .expect("Texture not found");

            let img = texture.source();
            match img.source() {
                #[allow(unused)]
                gltf::image::Source::View { view, mime_type } => {
                    println!("Embedded diffuse texture MIME type: {}", mime_type);
                    // TODO: Find out if correct
                    let _image_data = &images[img.index()].pixels;
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
                    println!("Embedded normal texture MIME type: {}", mime_type);
                    // TODO: Find out if correct
                    let _image_data = &images[view.index()].pixels;
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
    let mut joints = Vec::<SerializedBone3D>::new();
    // We need this because we iterate and change over it
    let mut joints_temp = Vec::<SerializedBone3D>::new();
    let mut joint_names = Vec::<String>::new();
    let mut joint_parents = HashMap::<usize, usize>::new();
    let mut inverse_bind_matrices = Vec::new();
    let mut raw_joint_nodes_indices_to_serialized_indices: HashMap<usize, usize> =
        HashMap::<usize, usize>::new();
    let mut serialized_indices_to_raw_joint_node_indices: HashMap<usize, usize> =
        HashMap::<usize, usize>::new();
    for skin in document.skins() {
        println!("Skin name: {:?}", skin.name().unwrap());
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

                println!("");

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
            joints_temp.push(serialized_bone);
            let serialized_joints_index = joints_temp.len() - 1;
            raw_joint_nodes_indices_to_serialized_indices
                .insert(joint_node_index, serialized_joints_index);
            serialized_indices_to_raw_joint_node_indices
                .insert(serialized_joints_index, joint_node_index);
        }
        println!("Num bones {}", num_bones);
        for j in joints_temp.iter() {
            let mut temp_joint = j.clone();
            if raw_joint_nodes_indices_to_serialized_indices.contains_key(&j.parent) {
                temp_joint.parent = raw_joint_nodes_indices_to_serialized_indices[&j.parent];
            } else {
                temp_joint.parent = INVALID_BONE;
            }
            joints.push(temp_joint);
        }

        println!("Serialized joints: ");
        for (i, j) in joints.iter().enumerate() {
            println!("    {}: {:?}", i, j);
        }
    }

    // Here we add the animations extracting code
    let anims = document.animations();
    let mut serialized_animations = Vec::<SerializedAnimation3D>::new();
    let mut anim_num = 0usize;
    println!("Num anims: {}", anims.len());
    for anim in anims {
        let mut channel_num = 0usize;
        let mut serialized_animations_channels = Vec::new();
        for channel in anim.channels() {
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
            let mut serialized_channels_keyframes: SerializedAnimation3DKeyframes = SerializedAnimation3DKeyframes { bone: target_bone_index, data: SerializedAnimation3DKeyframeTypes::Invalid, times };
            if let Some(outputs) = reader.read_outputs() {
                match outputs {
                    gltf::animation::util::ReadOutputs::Translations(translation) => {
                        let mut translations = Vec::new();
                        translation.for_each(|tr| {
                            // println!("Translation:");
                            // dbg!(tr);
                            translations.push(tr);
                        });
                        serialized_channels_keyframes.data = SerializedAnimation3DKeyframeTypes::Translations(translations.clone());
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
                        serialized_channels_keyframes.data =SerializedAnimation3DKeyframeTypes::Rotations(rotations.clone());
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
                        serialized_channels_keyframes.data = SerializedAnimation3DKeyframeTypes::Scales(scales.clone());
                        println!(
                            "Anim {}, channel {}, keyframe scales: {}",
                            anim_num,
                            channel_num,
                            scales.len()
                        );
                    }
                    _ => { } // gltf::animation::util::ReadOutputs::MorphTargetWeights(_) => {},
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
        serialized_animations.push(SerializedAnimation3D{ keyframes: serialized_animations_channels });
        anim_num += 1;
    }

    let mut serialized_model = SerializedModel3D::new();
    serialized_model.meshes = serialized_meshes;
    serialized_model.materials = serialized_materials;
    serialized_model.bones = joints;
    serialized_model.bone_names = joint_names;
    serialized_model.inverse_bind_matrices = inverse_bind_matrices;
    serialized_model.animations = serialized_animations;

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
