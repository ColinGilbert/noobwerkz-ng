use std::collections::HashMap;
use std::io::Write;
use std::rc::Rc;

use msgpacker::Packable;

use noobwerkz_shared::serialized_model3d::SerializedAnimation3D;
use noobwerkz_shared::serialized_model3d::SerializedAnimation3DChannel;
use noobwerkz_shared::serialized_model3d::SerializedAnimation3DKeyframeTypes;
use noobwerkz_shared::serialized_model3d::SerializedBone3D;
use noobwerkz_shared::serialized_model3d::SerializedMesh3D;
use noobwerkz_shared::serialized_model3d::SerializedModel3D;
use noobwerkz_shared::skeleton::Skeleton;
use russimp_ng::material::*;
use russimp_ng::scene::*;
use russimp_ng::*;
// use std::path::Path;

#[derive(Clone, Copy)]
struct BoneInfo {
    pub id: u32,
    pub offset_matrix: Matrix4x4,
}

struct RecursiveBoneInfo {
    pub name: String,
    // pub id: u32,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
    pub inverse_bind_matrix: glam::Mat4,
    pub children: Vec<RecursiveBoneInfo>,
}

fn recursive_skeletal_build_helper(
    node: &Rc<russimp_ng::node::Node>,
    bone_mappings: &HashMap<String, BoneInfo>,
    parent_id: usize,
    joint_id_counter: &mut u32,
    parent_matrix: glam::Mat4,
) -> Option<RecursiveBoneInfo> {
    let node_borrow = node;
    let name = node_borrow.name.clone();
    let local_bone_mat = glam::Mat4::from_cols_array_2d(&[
        [
            node_borrow.transformation.a1,
            node_borrow.transformation.a2,
            node_borrow.transformation.a3,
            node_borrow.transformation.a4,
        ],
        [
            node_borrow.transformation.b1,
            node_borrow.transformation.b2,
            node_borrow.transformation.b3,
            node_borrow.transformation.b4,
        ],
        [
            node_borrow.transformation.c1,
            node_borrow.transformation.c2,
            node_borrow.transformation.c3,
            node_borrow.transformation.c4,
        ],
        [
            node_borrow.transformation.d1,
            node_borrow.transformation.d2,
            node_borrow.transformation.d3,
            node_borrow.transformation.d4,
        ],
    ])
    .transpose();

    let cumulative_transform = parent_matrix * local_bone_mat;
    // We handle the case where a node is a bone
    if let Some(&bone_info) = bone_mappings.get(&name) {
        let current_joint = *joint_id_counter;
        *joint_id_counter += 1;

        let (scale, rot, pos) = glam::Mat4::to_scale_rotation_translation(&cumulative_transform);

        let inverse_bind_matrix = glam::Mat4::from_cols_array_2d(&[
            [
                bone_info.offset_matrix.a1,
                bone_info.offset_matrix.a2,
                bone_info.offset_matrix.a3,
                bone_info.offset_matrix.a4,
            ],
            [
                bone_info.offset_matrix.b1,
                bone_info.offset_matrix.b2,
                bone_info.offset_matrix.b3,
                bone_info.offset_matrix.b4,
            ],
            [
                bone_info.offset_matrix.c1,
                bone_info.offset_matrix.c2,
                bone_info.offset_matrix.c3,
                bone_info.offset_matrix.c4,
            ],
            [
                bone_info.offset_matrix.d1,
                bone_info.offset_matrix.d2,
                bone_info.offset_matrix.d3,
                bone_info.offset_matrix.d4,
            ],
        ])
        .transpose();

        let mut bone_results = RecursiveBoneInfo {
            name,
            // id: current_joint,
            translation: pos,
            rotation: rot,
            scale: scale,
            inverse_bind_matrix,
            children: Vec::new(),
        };

        for child_node in &node_borrow.children.borrow().to_vec() {
            if let Some(child_bone) = recursive_skeletal_build_helper(
                child_node,
                bone_mappings,
                current_joint as usize,
                joint_id_counter,
                cumulative_transform,
            ) {
                bone_results.children.push(child_bone);
            }
        }

        return Some(bone_results);
    }

    // Now we handle the case where a node isn't a bone but has children that could potentially be bones
    for child_node in &node_borrow.children.borrow().to_vec() {
        if let Some(child_joint) = recursive_skeletal_build_helper(
            child_node,
            bone_mappings,
            parent_id,
            joint_id_counter,
            cumulative_transform,
        ) {
            return Some(child_joint);
        }
    }

    // Now
    None
}

// pub struct Skeleton {
//     pub bones: Vec<Bone>,    // These store local transforms
//     pub parents: Vec<usize>, // These store the parent bone indices.
//     pub valid: Vec<bool>,
//     pub inverse_bind_matrices: Vec<glam::Mat4>,
//     pub num_bones: usize,
//     pub bone_names_to_idx: HashMap<String, usize>,
//     pub bone_idx_to_names: HashMap<usize, String>,
// }

fn recursive_skeletal_read_helper(
    input_bone: RecursiveBoneInfo,
    output_skeleton: &mut noobwerkz_shared::skeleton::Skeleton,
    parent_idx: usize,
) {
    let mut output_bone = noobwerkz_shared::bone::Bone::new();
    output_bone.position = input_bone.translation;
    output_bone.orientation = input_bone.rotation;
    output_bone.scale = input_bone.scale;
    output_skeleton.bones.push(output_bone);
    output_skeleton.parents.push(parent_idx);
    output_skeleton
        .inverse_bind_matrices
        .push(input_bone.inverse_bind_matrix);
    let idx = output_skeleton.bones.len() - 1;
    output_skeleton
        .bone_names_to_idx
        .insert(input_bone.name.clone(), idx);
    output_skeleton
        .bone_idx_to_names
        .insert(idx, input_bone.name.clone());
    output_skeleton.num_bones += 1;
    for child in input_bone.children {
        recursive_skeletal_read_helper(child, output_skeleton, idx);
    }
    println!("    Bone name: {}", input_bone.name.clone());
}

fn recursive_mesh_node_transform_helper(
    node: &Rc<russimp_ng::node::Node>,
    parent_matrix: glam::Mat4,
    serialized_model: &mut SerializedModel3D,
) {
    let t = node.transformation;
    let current_transform = glam::Mat4::from_cols_array_2d(&[
        [t.a1, t.a2, t.a3, t.a4],
        [t.b1, t.b2, t.b3, t.b4],
        [t.c1, t.c2, t.c3, t.c4],
        [t.d1, t.d2, t.d3, t.d4],
    ])
    .transpose();

    let cumulative_transform = parent_matrix * current_transform;

    for mesh_id in &node.meshes {
        let (s, r, t) = cumulative_transform.to_scale_rotation_translation();
        serialized_model.meshes[*mesh_id as usize].translation = t.to_array();
        serialized_model.meshes[*mesh_id as usize].rotation = r.to_array();
        serialized_model.meshes[*mesh_id as usize].scale = s.to_array();
    }

    for c in &node.children.borrow().to_vec() {
        recursive_mesh_node_transform_helper(c, cumulative_transform, serialized_model);
    }
}

fn load_model(file_path: &str) -> Result<(), RussimpError> {
    // Define the post-processing steps you want
    let post_process_flags = vec![
        PostProcess::CalculateTangentSpace,
        PostProcess::Triangulate,
        PostProcess::GenerateBoundingBoxes,
        // PostProcess::JoinIdenticalVertices,
        PostProcess::SortByPrimitiveType,
        PostProcess::ValidateDataStructure,
        PostProcess::GenerateNormals,
        // PostProcess::MakeLeftHanded,
        // PostProcess::OptimizeMeshes,
        // PostProcess::OptimizeGraph,
        // PostProcess::LimitBoneWeights,
        // PostProcess::PreTransformVertices,
        // PostProcess::GenerateUVCoords, // Example: Generate UVs if missing
    ];
    let mut bone_counter = 0;
    let mut bone_mappings = HashMap::<String, BoneInfo>::new();
    // Load the scene from file
    let scene: Scene = Scene::from_file(file_path, post_process_flags)?;
    println!("Scene loaded successfully from: {}", file_path);
    println!("  Meshes: {}", scene.meshes.len());
    println!("  Materials: {}", scene.materials.len());
    // println!("  Textures: {}", scene.textures.len());
    println!("  Animations: {}", scene.animations.len());
    println!("  Lights: {}", scene.lights.len());
    println!("  Cameras: {}", scene.cameras.len());

    let mut serialized_model = SerializedModel3D::new();
    println!("Outputting meshes:");
    for m in scene.meshes {
        println!("    Mesh : {}", m.name.clone());
        let mut serialized_mesh = SerializedMesh3D::new();
        let mut max_extents = [0.0; 3];
        let mut min_extents = [0.0; 3];
        for v in &m.vertices {
            let p = [v.x, v.y, v.z];
            serialized_mesh.positions.push(p);
            for i in 0..3 {
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
            }
        }

        for n in &m.normals {
            serialized_mesh.normals.push([n.y, n.y, n.z])
        }

        for uvs in &m.texture_coords {
            match uvs {
                Some(val) => {
                    for uv in val {
                        serialized_mesh.uvs.push([uv.x, uv.y]);
                    }
                }
                None => {
                    serialized_mesh.uvs.resize(m.vertices.len(), [0.0, 0.0]);
                }
            }
        }
        // Not currently supported - engine writes its own.
        // TODO: Make option to use model tangents and bitangents
        // for t in m.tangents {}
        // for bt in m.bitangents {}

        serialized_mesh
            .bone_indices
            .resize(serialized_mesh.positions.len(), [u32::MAX; 4]);
        serialized_mesh
            .bone_weights
            .resize(serialized_mesh.positions.len(), [0.0; 4]);

        for bn in &m.bones {
            let bone_id: u32;
            if !bone_mappings.contains_key(&bn.name.to_owned()) {
                bone_mappings.insert(
                    bn.name.to_owned(),
                    BoneInfo {
                        id: bone_counter,
                        offset_matrix: bn.offset_matrix,
                    },
                );
                bone_id = bone_counter;
                bone_counter += 1;
            } else {
                bone_id = bone_mappings[&bn.name.to_owned()].id;
            }
            for vw in &bn.weights {
                let id = vw.vertex_id;
                let weight = vw.weight;
                for i in 0..4 {
                    if serialized_mesh.bone_indices[id as usize][i] == u32::MAX {
                        serialized_mesh.bone_weights[id as usize][i] = weight;
                        serialized_mesh.bone_indices[id as usize][i] = bone_id;
                        break;
                    }
                }
            }
        }

        for f in &m.faces {
            if f.0.len() == 3 {
                let index0 = f.0[0];
                let index1 = f.0[1];
                let index2 = f.0[2];
                serialized_mesh.indices.push(index0);
                serialized_mesh.indices.push(index1);
                serialized_mesh.indices.push(index2);
            } else {
                println!("Face with more or less than 3 indices!");
            }
        }

        serialized_mesh.min_extents = min_extents;
        serialized_mesh.max_extents = max_extents;
        let mut dims = [0.0 as f32; 3];
        for i in 0..3 {
            dims[i] = max_extents[i] - min_extents[i];
        }
        serialized_mesh.dimensions = dims;
        serialized_mesh.name = m.name.clone();

        serialized_model.meshes.push(serialized_mesh);
    }

    let root = scene.root.unwrap();

    let global_inverse_transform = glam::Mat4::from_cols_array_2d(&[
        [
            root.transformation.a1,
            root.transformation.a2,
            root.transformation.a3,
            root.transformation.a4,
        ],
        [
            root.transformation.b1,
            root.transformation.b2,
            root.transformation.b3,
            root.transformation.b4,
        ],
        [
            root.transformation.c1,
            root.transformation.c2,
            root.transformation.c3,
            root.transformation.c4,
        ],
        [
            root.transformation.d1,
            root.transformation.d2,
            root.transformation.d3,
            root.transformation.d4,
        ],
    ])
    .transpose()
    .inverse();

    serialized_model.global_inverse_transform = global_inverse_transform.to_cols_array_2d();

    recursive_mesh_node_transform_helper(&root, glam::Mat4::IDENTITY, &mut serialized_model);

    println!("Outputting materials:");
    for m in scene.materials {
        let texture_none = m.textures.get(&TextureType::None);
        let mut serialized_material =
            noobwerkz_shared::serialized_model3d::SerializedModelMaterial::new();
        match texture_none {
            Some(_) => {
                println!("No textures set in file. Defaulting to sane values");
                serialized_material.diffuse_texture_path = "default-diffuse.png".to_owned();
                serialized_material.normals_texture_path = "default-normal.png".to_owned();
            }
            None => {}
        }

        let texture_diffuse = m.textures.get(&TextureType::Diffuse);

        match texture_diffuse {
            Some(val) => {
                let format_hint = &val.borrow().ach_format_hint;
                println!("Diffuse texture format hint: {}", format_hint);
            }
            None => {}
        }

        let texture_normals = m.textures.get(&TextureType::Normals);

        match texture_normals {
            Some(val) => {
                let format_hint = &val.borrow().ach_format_hint;
                println!("Normals texture format hint: {}", format_hint);
            }
            None => {}
        }

        // TODO
        // let texture_specular = m.textures.get(&TextureType::Specular);
        // for mp in m.properties {
        //     let _key = mp.key;
        //     //mp.data == PropertyTypeInfo)
        // }

        // let texture_none = m.textures.get(&TextureType::None);

        // let texture_none = m.textures.get(&TextureType::None);

        // let texture_none = m.textures.get(&TextureType::None);
        serialized_model.materials.push(serialized_material);
    }

    let mut joint_id_counter = 0u32;
    let recursively_built_skeleton = recursive_skeletal_build_helper(
        &root,
        &bone_mappings,
        noobwerkz_shared::bone::INVALID_BONE,
        &mut joint_id_counter,
        glam::Mat4::IDENTITY,
    );

    println!("Outputting skeleton:");
    let mut output_skeleton = Skeleton::new();
    match recursively_built_skeleton {
        Some(val) => {
            recursive_skeletal_read_helper(
                val,
                &mut output_skeleton,
                noobwerkz_shared::bone::INVALID_BONE,
            );

            for ibm in output_skeleton.inverse_bind_matrices {
                serialized_model
                    .inverse_bind_matrices
                    .push(ibm.to_cols_array_2d());
            }

            for (i, output_bone) in output_skeleton.bones.iter().enumerate() {
                let serialized_bone = SerializedBone3D {
                    scale: output_bone.scale.to_array(),
                    translation: output_bone.position.to_array(),
                    rotation: output_bone.orientation.to_array(),
                    parent: output_skeleton.parents[i],
                };
                serialized_model.bones.push(serialized_bone);
                let name = &output_skeleton.bone_idx_to_names[&i];
                serialized_model.bone_names.push(name.clone());
            }
        }
        None => {
            println!("Skeleton could not be recursively built.");
        }
    }

    let mut anim_num = 0;
    println!("Outputting animations:");
    for a in scene.animations {
        // println!("    Animation name: {}", a.name.clone());
        let mut serialized_animation = SerializedAnimation3D {
            channels: Vec::new(),
        };
        let name;
        if a.name.clone() == "" {
            name = anim_num.to_string();
        } else {
            name = a.name.clone();
        }
        println!("   {}", name.clone());
        for c in a.channels {
            // let id = output_skeleton.bone_names_to_idx.get(&c.name.clone());
            let mut id = 0;
            let mut found = false;
            while found == false {
                if id >= serialized_model.bone_names.len() {
                    break;
                }
                if serialized_model.bone_names[id] == c.name.clone() {
                    found = true;
                    break;
                }
                id += 1;
            }
            if !found {
                return Err(RussimpError::Import(
                    "While importing animations: Couldn't find id of output bone.".to_owned(),
                ));
            }
            // We handle the positions.
            let mut positions = Vec::new();
            let mut times = Vec::new();
            for p in &c.position_keys {
                positions.push([p.value.x, p.value.y, p.value.z]);
                times.push(p.time as f32);
            }
            if c.position_keys.len() > 0 {
                let output_positions_keyframes =
                    SerializedAnimation3DKeyframeTypes::Translations(positions);
                let mut serialized_pos_channel =
                    SerializedAnimation3DChannel::new(output_positions_keyframes);
                serialized_pos_channel.bone = id;
                serialized_pos_channel.times = times.clone();
                serialized_animation.channels.push(serialized_pos_channel);
            }
            // We handle rotations.
            let mut rotations = Vec::new();
            let mut times = Vec::new();
            for r in &c.rotation_keys {
                rotations.push(
                    glam::Quat::from_array([r.value.w, r.value.x, r.value.y, r.value.z])
                        .normalize()
                        .to_array(),
                );
                times.push(r.time as f32);
            }
            if c.rotation_keys.len() > 0 {
                let output_rotations_keyframes =
                    SerializedAnimation3DKeyframeTypes::Rotations(rotations);
                let mut serialized_rot_channel =
                    SerializedAnimation3DChannel::new(output_rotations_keyframes);

                serialized_rot_channel.bone = id;
                serialized_rot_channel.times = times.clone();
                serialized_animation.channels.push(serialized_rot_channel);
            }
            // We handle scales.
            let mut scales = Vec::new();
            let mut times = Vec::new();
            for s in &c.scaling_keys {
                scales.push([s.value.x, s.value.y, s.value.z]);
                times.push(s.time as f32);
            }
            if c.scaling_keys.len() > 0 {
                let output_scales_keyframes = SerializedAnimation3DKeyframeTypes::Scales(scales);
                let mut serialized_scales_channel =
                    SerializedAnimation3DChannel::new(output_scales_keyframes);
                serialized_scales_channel.bone = id;
                serialized_scales_channel.times = times.clone();
                serialized_animation
                    .channels
                    .push(serialized_scales_channel);
            }
        }

        // for c in &serialized_animation.channels {
        //     println!("{:?}", c.data);
        //     println!("{:?}", c.times);
        // }
        serialized_model.animations.push(serialized_animation);
        serialized_model
            .animations_by_name
            .insert(name.clone(), serialized_model.animations.len() - 1);

        anim_num += 1;
    }

    // We do some validations
    if output_skeleton.bones.len() != serialized_model.bones.len() {
        return Err(russimp_ng::RussimpError::Import(
            "Discrepancy in bone vec lengths".to_owned(),
        ));
    }
    if output_skeleton.bones.len() != output_skeleton.parents.len() {
        return Err(russimp_ng::RussimpError::Import(
            "Discrepancy in output bone to parent vec lengths".to_owned(),
        ));
    }

    let mut i = 0;
    while i < output_skeleton.parents.len() {
        if serialized_model.bones[i].parent != output_skeleton.parents[i] {
            return Err(russimp_ng::RussimpError::Import(
                "Discrepancy between output skeleon parents and serialized skeleton parents"
                    .to_owned(),
            ));
        }
        i += 1;
    }

    for(i, b) in serialized_model.bones.iter().enumerate() {
        println!("Bone: {}, name: {}, parent: {}", i, serialized_model.bone_names[i], b.parent);
    }

    println!("Inverse global transform: {:?}", serialized_model.global_inverse_transform);

    // Now we write out the file
    let mut buf = Vec::new();
    let _n = serialized_model.pack(&mut buf);
    let mut file = std::fs::File::create("model.bin").unwrap();
    let results = file.write_all(&buf);
    match results {
        Ok(_data) => {
            println!("Writing file")
        }
        Err(e) => {
            println!("Error writing file: {}", e)
        }
    }

    Ok(())
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        match load_model(path.as_str()) {
            Ok(_) => println!("Finished processing model."),
            Err(e) => eprintln!("Error loading model: {}", e),
        }
    } else {
        println!("usage: noobwerkz-assimp-converter <FILE>");
    }
}
