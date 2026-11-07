use crate::{csg::*, serialized_model3d::*};
use rapier3d::prelude::*;

use binary_greedy_meshing as bgm;
use rerecast::AreaType;
use std::collections::BTreeSet;

pub enum Shape3D {
    Cube(f32),
    Cuboid(f32, f32, f32),
    Sphere(f32),
    CapsuleX(f32, f32),
    CapsuleY(f32, f32),
    Trimesh(SerializedMesh3D),
    Voxels(glam::Vec3, Vec<glam::IVec3>),
}

pub fn shape_to_collider(shape: &Shape3D) -> Option<Collider> {
    match shape {
        Shape3D::Cube(scale) => {
            return Some(
                ColliderBuilder::new(SharedShape::cuboid(scale / 2.0, scale / 2.0, scale / 2.0))
                    .build(),
            );
        }
        Shape3D::Cuboid(x, y, z) => {
            return Some(ColliderBuilder::new(SharedShape::cuboid(*x, *y, *z)).build());
        }
        Shape3D::Sphere(radius) => {
            return Some(ColliderBuilder::new(SharedShape::ball(*radius)).build());
        }
        Shape3D::CapsuleX(length, radius) => {
            return Some(
                ColliderBuilder::new(SharedShape::capsule_x(length / 2.0, *radius)).build(),
            );
        }
        Shape3D::CapsuleY(length, radius) => {
            return Some(
                ColliderBuilder::new(SharedShape::capsule_y(length / 2.0, *radius)).build(),
            );
        }
        Shape3D::Trimesh(mesh) => {
            let indices = Vec::new();
            let mut counter = 0;
            let mut accum = [0, 0, 0];
            for i in &mesh.indices {
                accum[counter] = *i;
                if counter > 2 {
                    counter = 0;
                }
                counter += 1;
            }
            let shape = SharedShape::trimesh(
                mesh.clone().positions
                    .into_iter()
                    .map(glam::Vec3::from_array)
                    .collect(),
                indices,
            );
            match shape {
                Ok(val) => {
                    return Some(ColliderBuilder::new(val).build());
                }
                Err(err) => {
                    println!("Error creating trimesh: {}", err);
                    return None;
                }
            }
        }
        Shape3D::Voxels(voxel_size, grid_coords) => {
            return Some(
                ColliderBuilder::new(SharedShape::voxels(*voxel_size, &grid_coords)).build(),
            );
        }
    }
}

pub fn shape_to_serialized_model(shape: &Shape3D) -> Option<SerializedModel3D> {
    match shape {
        Shape3D::Cube(scale) => Some(get_serialized_model(&cube(*scale as f64))),
        Shape3D::Cuboid(x, y, z) => Some(get_serialized_model(&cuboid(*x as f64, *y as f64, *z as f64))),
        Shape3D::Sphere(radius) => Some(get_serialized_model(&sphere(*radius as f64))),
        Shape3D::CapsuleX(_length, _radius) => None,
        Shape3D::CapsuleY(length, radius) => Some(get_serialized_model(&capsule_y(
            *length as f64,
            *radius as f64,
        ))),
        Shape3D::Trimesh(mesh) => {
            let mut results = SerializedModel3D::new();
            results.meshes.push(mesh.clone());
            Some(results)
        }
        Shape3D::Voxels(voxel_size, grid_coords) => {
            // We first find out the smallest values, in order to offset into the voxels array.
            let mut x_min = 0;
            let mut y_min = 0;
            let mut z_min = 0;
            for c in grid_coords {
                x_min = x_min.min(c.x);
                y_min = y_min.min(c.y);
                z_min = z_min.min(c.z);
            }
            let is_x_negative;
            if x_min < 0 {
                is_x_negative = true;
            } else {
                is_x_negative = false;
            }
            let is_y_negative;
            if y_min < 0 {
                is_y_negative = true;
            } else {
                is_y_negative = false;
            }
            let is_z_negative;
            if z_min < 0 {
                is_z_negative = true;
            } else {
                is_z_negative = false;
            }

            let x_offset;
            if is_x_negative {
                x_offset = x_min.abs();
            } else {
                x_offset = 0;
            }

            let y_offset;
            if is_y_negative {
                y_offset = y_min.abs();
            } else {
                y_offset = 0;
            }
            let z_offset;
            if is_z_negative {
                z_offset = z_min.abs();
            } else {
                z_offset = 0
            }
            const VOXELS_EXTENTS: usize = 64;
            let mut voxels = [0u16; bgm::Mesher::<VOXELS_EXTENTS>::CS_P3];
            for c in grid_coords {
                voxels[bgm::pad_linearize::<VOXELS_EXTENTS>(
                    (c.x + x_offset) as usize,
                    (c.y + y_offset) as usize,
                    (c.z + z_offset) as usize,
                )] = 1;
            }
            let mut mesher = bgm::Mesher::<VOXELS_EXTENTS>::new();

            mesher.mesh(&voxels, &BTreeSet::new());
            let mut results = SerializedModel3D::new();
            let mut mesh = SerializedMesh3D::new();
            for (face_n, quads) in mesher.quads.iter().enumerate() {
                let face: bgm::Face = (face_n as u8).into();
                //let n = face.n().map(|v| v as f32);
                for q in quads {
                    let vertices_packed = face.vertices_packed(*q);
                    for vertex in vertices_packed.iter() {
                        let [x, y, z] = vertex.xyz();
                        mesh.positions.push([x as f32, y as f32, z as f32]);
                        //mesh.normals.push(n.clone());
                    }
                }
            }
            let mut min_x = 0.0;
            let mut min_y = 0.0;
            let mut min_z = 0.0;
            let mut max_x = 0.0;
            let mut max_y = 0.0;
            let mut max_z = 0.0;

            mesh.indices = bgm::indices(mesh.positions.len() / 4);

            for p in mesh.positions.iter_mut() {
                p[0] = (p[0] - x_offset as f32) * voxel_size.x;
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
                p[1] = (p[1] - y_offset as f32) * voxel_size.y;
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
                p[2] = (p[2] - z_offset as f32) * voxel_size.z;
                min_z = min_z.min(p[2]);
                max_z = max_z.max(p[2]);
            }
            mesh.min_extents = [min_x, min_y, min_z];
            mesh.max_extents = [max_x, max_y, max_z];

            mesh.recalculate_normals();

            results.meshes.push(mesh);
            
            Some(results)
        }
    }
}


pub fn mesh_to_recast_mesh(mesh: &SerializedMesh3D) -> rerecast::TriMesh {
        let mut result = rerecast::TriMesh::default();
    for indices in mesh.indices.chunks(3) {
        let face = glam::UVec3::new(indices[0] , indices[1], indices[2]);
        result.indices.push(face);
        result.area_types.push(AreaType::NOT_WALKABLE);
    }

    for p in &mesh.positions {
        result.vertices.push(glam::Vec3A::from_array([p[0], p[1], p[2]]));
    }
    result
}