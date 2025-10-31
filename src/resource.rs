use crate::index_types::*;
use crate::material::*;
use crate::model::*; //{Material, MaterialIndex, Model, ModelVertex, TexturedMesh};
use crate::skinned_model::*;
use crate::texture;
use capnp::message::*;
use capnp::*;
use model3d_schema_capnp::model;
// use ozz_animation_rs::*;
use std::fs;
use wgpu::util::DeviceExt;
mod model3d_schema_capnp {
    include!("model3d_schema_capnp.rs");
}

pub enum GenericModel {
    Textured(Model),
    SkinnedTextured(SkinnedModel),
}

// TODO: Robustify
pub async fn load_model_from_serialized(
    filepath: String,
    filename: String,
    default_material: Material,
    device: &mut wgpu::Device,
    queue: &mut wgpu::Queue,
    texture_layout: &wgpu::BindGroupLayout,
) -> Result<GenericModel> {
    let full_path = filepath.clone() + "/" + &filename;
    // println!("Full path: {}", full_path);
    let data = fs::read(full_path).unwrap();
    // println!("Data length {}", data.len());
    let options: ReaderOptions = ReaderOptions {
        traversal_limit_in_words: Some(4000 as usize * 1024 as usize * 1024 as usize),
        nesting_limit: 4,
    };
    let message_reader =
        capnp::serialize_packed::read_message(&mut data.as_slice(), options).unwrap();
    let message = message_reader.get_root::<model::Reader>(); //::<model3d_capnp::Reader>();
    let meshes_serialized = message.as_ref().unwrap().get_meshes().unwrap();
    let materials_serialized = message.as_ref().unwrap().get_materials().unwrap();
    let mut result_model = Model::new();
    let mut result_skinned = SkinnedModel::new();

    let mut has_bones = false;
    for mesh_serialized in meshes_serialized {
        if mesh_serialized.get_bone_names().unwrap().len() > 0 {
            has_bones = true;
        }
    }

    for mesh_serialized in meshes_serialized {
        let mut verts = Vec::<ModelVertex>::new();
        let mut verts_skinned = Vec::<SkinnedModelVertex>::new();
        let mut indices = Vec::<u32>::new();
        if mesh_serialized.has_positions() {
            if !mesh_serialized.has_normals() {
                () // TODO: Generate normals
            }
            if !mesh_serialized.has_uvs() {
                () // TODO: Handle
            }
            let positions = mesh_serialized.get_positions().unwrap();
            let normals = mesh_serialized.get_normals().unwrap();
            let tex_coords = mesh_serialized.get_uvs().unwrap();
            if positions.len() != normals.len() {
                () // TODO: Return error
            }
            if positions.len() != tex_coords.len() {
                () // TODO: Handle
            }

            let mut i = 0;
            while i < positions.len() {
                let mut v = ModelVertex::new();
                let p = positions.get(i);
                v.position[0] = p.get_array3f_x();
                v.position[1] = p.get_array3f_y();
                v.position[2] = p.get_array3f_z();
                verts.push(v);
                i += 1;
            }
            i = 0;
            while i < normals.len() {
                let mut v: ModelVertex = verts[i as usize];
                let n = normals.get(i);
                v.normal[0] = n.get_array3f_x();
                v.normal[1] = n.get_array3f_y();
                v.normal[2] = n.get_array3f_z();
                verts[i as usize] = v;
                i += 1;
            }
            i = 0;
            while i < tex_coords.len() {
                let mut v = verts[i as usize];
                let t = tex_coords.get(i);
                v.tex_coords[0] = t.get_array2f_x();
                v.tex_coords[1] = 1.0 - t.get_array2f_y();
                verts[i as usize] = v;
                i += 1;
            }
        }

        if mesh_serialized.has_indices() {
            let indices_serialized = mesh_serialized.get_indices().unwrap();
            //println!("{}", indices_serialized.len());
            for i in indices_serialized {
                indices.push(i as u32);
            }
        }

        if mesh_serialized.has_uvs()
            && mesh_serialized.get_uvs().unwrap().len()
                == mesh_serialized.get_positions().unwrap().len()
        {
            calculate_tangents_and_bitangents(&mut verts, &indices);
        }

        let name: String;
        if mesh_serialized.has_name() {
            name = mesh_serialized.get_name().unwrap().to_string().unwrap();
        } else {
            name = filename.clone();
        }

        let material_index = mesh_serialized.get_material_index();

        let translation = glam::Vec3::new(
            mesh_serialized.get_translation_x(),
            mesh_serialized.get_translation_y(),
            mesh_serialized.get_translation_z(),
        );

        let rotation = glam::Quat::from_xyzw(
            mesh_serialized.get_rotation_x(),
            mesh_serialized.get_rotation_y(),
            mesh_serialized.get_rotation_z(),
            mesh_serialized.get_rotation_w(),
        );

        let scale = glam::Vec3::new(
            mesh_serialized.get_scale_x(),
            mesh_serialized.get_scale_y(),
            mesh_serialized.get_scale_z(),
        );

        let dimensions = glam::Vec3::new(
            mesh_serialized.get_dimensions_x(),
            mesh_serialized.get_dimensions_y(),
            mesh_serialized.get_dimensions_z(),
        );

        if !has_bones {
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", name)),
                contents: bytemuck::cast_slice(&verts),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", name)),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            result_model.meshes.push(TexturedMesh {
                name,
                vertex_buffer,
                index_buffer,
                num_elements: indices.len() as u32,
                material: MaterialIndex::new(material_index as usize),
                translation,
                rotation,
                scale,
                dimensions,
            });
        } else {
            // We extract the bones components now
            let bone_indices_serialized = mesh_serialized.get_bone_indices().unwrap();
            let mut s = String::new();
            s += "Bone indices:\n";
            let mut i: usize = 0;
            while i < bone_indices_serialized.len() as usize {
                let mut v = SkinnedModelVertex::from_vert(&verts[i]);
                let bone_indices = bone_indices_serialized.get(i as u32);
                v.bone_indices[0] = bone_indices.get_array4u_x() as u32;
                v.bone_indices[1] = bone_indices.get_array4u_y() as u32;
                v.bone_indices[2] = bone_indices.get_array4u_z() as u32;
                v.bone_indices[3] = bone_indices.get_array4u_w() as u32;
                verts_skinned.push(v);
                s+= "(";
                for bi in v.bone_indices{
                    s += &bi.to_string();
                    s += " ";
                }
                s+= ") ";

                i += 1;
            }
            println!("{}", s);
            let bone_weights_serialized = mesh_serialized.get_bone_weights().unwrap();
            i = 0;
            while i < bone_weights_serialized.len() as usize {
                let mut v = verts_skinned[i];
                let bone_weights = bone_weights_serialized.get(i as u32);
                v.bone_weights[0] = bone_weights.get_array4f_x();
                v.bone_weights[1] = bone_weights.get_array4f_y();
                v.bone_weights[2] = bone_weights.get_array4f_z();
                v.bone_weights[3] = bone_weights.get_array4f_w();
                verts_skinned[i] = v;
                i += 1;
            }

            let bone_names_serialized = mesh_serialized.get_bone_names().unwrap();
            let mut bone_names = Vec::<String>::new();
            for (i, _n) in bone_names_serialized.iter().enumerate() {
                bone_names.push(
                    bone_names_serialized
                        .get(i as u32)
                        .unwrap()
                        .to_string()
                        .unwrap(),
                );
            }

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Skinned Vertex Buffer", name)),
                contents: bytemuck::cast_slice(&verts_skinned),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Skinned Index Buffer", name)),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            //println!("Bone names: {:?}", bone_names);
            result_skinned.meshes.push(SkinnedTexturedMesh {
                name,
                vertex_buffer,
                index_buffer,
                num_elements: indices.len() as u32,
                material: MaterialIndex::new(material_index as usize),
                translation,
                rotation,
                scale,
                dimensions,
                //matrices_texture: Option::None,
            });
        }
    }

    for material_serialized in materials_serialized {
        let name = material_serialized.get_name().unwrap().to_string().unwrap();
        let has_diffuse_map: bool;
        if material_serialized
            .get_diffuse_texture_path()
            .unwrap()
            .to_string()
            .unwrap()
            == ""
        {
            has_diffuse_map = false;
        } else {
            has_diffuse_map = true;
        }

        let has_normals_map: bool;
        if material_serialized
            .get_normals_texture_path()
            .unwrap()
            .to_string()
            .unwrap()
            == ""
        {
            has_normals_map = false;
        } else {
            has_normals_map = true;
        }

        let diffuse_path = filepath.clone()
            + "/"
            + &material_serialized
                .get_diffuse_texture_path()
                .unwrap()
                .to_string()
                .unwrap();
        let normals_path = filepath.clone()
            + "/"
            + &material_serialized
                .get_normals_texture_path()
                .unwrap()
                .to_string()
                .unwrap();

        let diffuse_texture: texture::Texture;
        if has_diffuse_map {
            let diffuse_texture_result = load_texture(&diffuse_path, false, device, queue).await;
            match diffuse_texture_result {
                Ok(value) => {
                    diffuse_texture = value;
                }
                Err(value) => {
                    println!(
                        "Could not load diffuse texture {}, error: {}",
                        diffuse_path, value
                    );
                    diffuse_texture = default_material.diffuse_texture.clone();
                }
            }
        } else {
            diffuse_texture = default_material.diffuse_texture.clone();
        }

        let normal_texture: texture::Texture;
        if has_normals_map {
            let normal_texture_result = load_texture(&normals_path, true, device, queue).await;
            match normal_texture_result {
                Ok(value) => {
                    normal_texture = value;
                }
                Err(value) => {
                    println!(
                        "Could not load normals texture {}, error: {}",
                        normals_path, value
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
        if !has_bones {
            result_model.materials.push(material);
        } else {
            result_skinned.materials.push(material);
        }
    }

    if materials_serialized.len() == 0 {
        let material = default_material.clone();
        if !has_bones {
            result_model.materials.push(material);
        } else {
            result_skinned.materials.push(material);
        }
    }
    if !has_bones {
        result_model.name = filename;
    } else {
        result_skinned.name = filename;
    }

    let result: GenericModel;
    if !has_bones {
        result = GenericModel::Textured(result_model);
    } else {
        result = GenericModel::SkinnedTextured(result_skinned);
    }
    Ok(result)
}

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
