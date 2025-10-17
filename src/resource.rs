use capnp::message::*;
use capnp::*;
use std::fs;
use std::path::Path;
use wgpu::MemoryBudgetThresholds;
use wgpu::util::DeviceExt;

use crate::graphics::GraphicsContext;
use crate::model::{MaterialIndex, Model, ModelVertex, TexturedMesh};
use crate::model3d_schema_capnp::*;
use crate::scene;
use crate::texture;
use std::fmt;

mod model3d_schema_capnp {
    include!("model3d_schema_capnp.rs");
}
#[derive(Debug, Clone)]
struct MeshError;
impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad mesh")
    }
}
pub fn load_model_from_serialized(filepath: String, device: &mut wgpu::Device) -> Result<Model> {
    let data: Vec<u8> = fs::read(filepath.clone()).unwrap();
    println!("Data length {}", data.len());
    use model3d_schema_capnp::{array2f, array3f, array4f, array4u, mesh, model};
    let options: ReaderOptions = ReaderOptions {
        traversal_limit_in_words: Some(4000 as usize * 1024 as usize * 1024 as usize),
        nesting_limit: 8,
    };
    let message_reader =
        capnp::serialize_packed::read_message(&mut data.as_slice(), options).unwrap();
    let message = message_reader.get_root::<model::Reader>(); //::<model3d_capnp::Reader>();
    let meshes_serialized = message.unwrap().get_meshes().unwrap();
    let mut verts = Vec::<ModelVertex>::new();
    let mut indices = Vec::<u32>::new();
    let mut result = Model::new();
    for mesh_serialized in meshes_serialized {
        if mesh_serialized.has_positions() {
            //let vertices_serialized = mesh_serialized.get_vertices().unwrap();
            if !mesh_serialized.has_normals() {
                ()
            }
            if !mesh_serialized.has_uvs() {
                ()
            }
            let positions = mesh_serialized.get_positions().unwrap();
            let normals = mesh_serialized.get_normals().unwrap();
            let tex_coords = mesh_serialized.get_uvs().unwrap();
            if positions.len() != normals.len() {
                ()
            }
            if positions.len() != tex_coords.len() {
                ()
            }

            let mut i  = 0;
            while i < positions.len() {
                let mut v = ModelVertex::new();
                v.position[0] = positions.get(i).get_array3f_x();
                v.position[1] = positions.get(i).get_array3f_y();
                v.position[2] = positions.get(i).get_array3f_y();
                verts.push(v);
                i += 1;
            }
            i = 0;
            while i < normals.len() {
                let mut v: ModelVertex = verts[i as usize];
                v.normal[0] = normals.get(i).get_array3f_x();
                v.normal[1] = normals.get(i).get_array3f_y();
                v.normal[2] = normals.get(i).get_array3f_y();
                verts[i as usize] = v;
                i += 1;
            }
            i = 0;
            while i < tex_coords.len() {
                let mut v = verts[i as usize];
                v.tex_coords[0] = tex_coords.get(i).get_array2f_x();
                v.tex_coords[1] = tex_coords.get(i).get_array2f_y();
                verts[i as usize] = v;
                i += 1;
            }
        }

        if mesh_serialized.has_indices() {
            let indices_serialized = mesh_serialized.get_indices().unwrap();
            println!("{}", indices_serialized.len());
            for i in indices_serialized {
                indices.push(i as u32);
            }
        }

        calculate_tangents_and_bitangents(&mut verts, &indices);
        // println!(
        //     "Translation: {} {} {} ",
        //     mesh_serialized.get_translation_x(),
        //     mesh_serialized.get_translation_y(),
        //     mesh_serialized.get_translation_z()
        // );
        // println!(
        //     "Rotation: {} {} {} ",
        //     mesh_serialized.get_rotation_x(),
        //     mesh_serialized.get_rotation_y(),
        //     mesh_serialized.get_rotation_z()
        // );
        // println!(
        //     "Scale: {} {} {} ",
        //     mesh_serialized.get_scale_x(),
        //     mesh_serialized.get_scale_y(),
        //     mesh_serialized.get_scale_z()
        // );
        // println!(
        //     "Dimensions: {} {} {} ",
        //     mesh_serialized.get_dimensions_x(),
        //     mesh_serialized.get_dimensions_y(),
        //     mesh_serialized.get_dimensions_z()
        // );
        let name: String;
        if mesh_serialized.has_name() {
            name = mesh_serialized.get_name().unwrap().to_string().unwrap();
        } else {
            name = filepath.clone();
        }
        // if mesh_serialized.has_bone_names() {
        //     print!("Bones: ");
        //     let bones = mesh_serialized.get_bone_names().unwrap();
        //     for b in bones {
        //         print!("{} ", b.unwrap().to_string().unwrap());
        //     }
        //     println!();
        // }
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

        result.meshes.push(TexturedMesh {
            name,
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material: MaterialIndex::new(0), // TODO: FIX m.mesh.material_id.unwrap_or(0),
        });
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
