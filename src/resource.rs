use crate::model;
use crate::scene;
use crate::texture;
use asset_importer_rs::*;
use asset_importer_rs_core::*;
use asset_importer_rs_gltf::{Gltf2ImportError, Gltf2Importer};
use asset_importer_rs_scene::{AiMesh, AiScene};
use std::path::Path;

pub fn import_gltf(path_string: &str) -> model::Model {
    let gltf_scene = gltf_load(path_string);
    //println!("  Mesh {} has {} verts, {} normals, {} tangents, and {} bi-tangents", m.name, m.vertices.len(), m.normals.len(), m.tangents.len(), m.bi_tangents.len());
    let mut results = model::Model::new();
    for gltf_mesh in gltf_scene.meshes {
        // We only process trimeshes
        if gltf_mesh.primitive_types != asset_importer_rs_scene::AiPrimitiveType::Triangle {
            continue;
        }
        let mut verts = Vec::<model::ModelVertex>::new();
        let mut indices = Vec::<u32>::new();
        verts.reserve(gltf_mesh.vertices.len());
        for (i, vert) in gltf_mesh.vertices.into_iter().enumerate() {
            let &(mut v) = &verts[i];
            v.position = [vert.x, vert.y, vert.z];
        }
        for (i, n) in gltf_mesh.normals.into_iter().enumerate() {
            let &(mut v) = &verts[i];
            v.normal = [n.x, n.y, n.z];
        }
        for (i, t) in gltf_mesh.texture_coords[0].unwrap().into_iter().enumerate() {
            let &(mut v) = &verts[i];
            v.tex_coords = [t.x, 1.0 - t.y];
        }

        let indices = &gltf_mesh.faces;
        let mut triangles_included = vec![0; gltf_mesh.vertices.len()];

        // Calculate tangents and bitangets. We're going to
        // use the triangles, so we need to loop through the
        // indices in chunks of 3
        for c in &gltf_mesh.faces {
            indices.push(c[0]);
            indices.push(c[1]);
            indices.push(c[2]);
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
            // Luckily, the place I found this equation provided
            // the solution!
            let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
            let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
            // We flip the bitangent to enable right-handed normal
            // maps with wgpu texture coordinate system
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

        results.meshes.push(model::TexturedMesh {
            name: path_string.to_string(),
            vertex_buffer,
            index_buffer,
            num_elements,
            material,
        });
    }
    //println!("Scene has {} materials", scene.materials.len());
    for mat in gltf_scene.materials {}

    results
    //    let mut results = scene::Scene::new();

    //    if gltf_scene.nodes.arena.len() > 0 {
    //         let mut stack = Vec::<(glam::Mat4, usize)>::new();
    //         let mut traversing = true;
    //         let transformation = &gltf_scene.nodes.arena[gltf_scene.nodes.root.unwrap()].transformation;
    //         let stack_idx = stack.push((asset_import_mat4_to_glam(transformation), 0));
    //         // TODO
    //         while traversing {

    //         }
    //     }
    //     results
}

fn asset_import_mat4_to_glam(transformation: &asset_importer_rs_scene::AiMatrix4x4) -> glam::Mat4 {
    glam::Mat4 {
        x_axis: glam::Vec4::new(
            transformation.a1,
            transformation.a2,
            transformation.a3,
            transformation.a4,
        ),
        y_axis: glam::Vec4::new(
            transformation.b2,
            transformation.b2,
            transformation.b3,
            transformation.b4,
        ),
        z_axis: glam::Vec4::new(
            transformation.c2,
            transformation.c2,
            transformation.c3,
            transformation.c4,
        ),
        w_axis: glam::Vec4::new(
            transformation.d2,
            transformation.d2,
            transformation.d3,
            transformation.d4,
        ),
    }
}

fn gltf_load(path_string: &str) -> asset_importer_rs_scene::AiScene {
    // Create an importer
    let importer = asset_importer_rs_gltf::Gltf2Importer::new();

    // Import a glTF file
    let scene = importer
        .read_file(
            Path::new(path_string),
            asset_importer_rs_core::default_file_loader,
        )
        .unwrap();

    scene
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
