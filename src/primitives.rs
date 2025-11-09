use crate::serialized_model::*;
use glam::*;
use truck_meshalgo::{
    prelude::{Splitting, StructuringFilter},
    tessellation::*,
};
use truck_modeling::*;

pub fn cube(scale: f64) -> SerializedModel {
    let v = builder::vertex(Point3::new(-scale / 2.0, -scale / 2.0, -scale / 2.0));
    let e = builder::tsweep(&v, Vector3::new(scale, 0.0, 0.0));
    let f = builder::tsweep(&e, Vector3::new(0.0, scale, 0.0));
    let cube = builder::tsweep(&f, Vector3::new(0.0, 0.0, scale));
    let mut polygon_mesh = cube.triangulation(0.005).to_polygon();
    let triangulation = polygon_mesh.triangulate();
    let mut result = SerializedModel::new();
    result.meshes.push(SerializedMesh::new());

    for p in triangulation.positions() {
        result.meshes[0]
            .positions
            .push([p.x as f32, p.y as f32, p.z as f32]);
    }

    for n in triangulation.normals() {
        result.meshes[0]
            .normals
            .push([n.x as f32, n.y as f32, n.z as f32]);
    }

    for t in triangulation.uv_coords() {
        result.meshes[0].uvs.push([t.x as f32, t.y as f32])
    }

    let faces = triangulation.faces();

    let triangles = faces.tri_faces();

    for t in triangles {
        result.meshes[0].indices.push(t[0].pos as u32);
        result.meshes[0].indices.push(t[1].pos as u32);
        result.meshes[0].indices.push(t[2].pos as u32);
    }

    result.meshes[0].scale = [1.0, 1.0, 1.0];
    result.meshes[0].rotation = glam::Quat::IDENTITY.to_array();

    result

    //     let mut results = SerializedModel::new();
    //     let mut m = SerializedMesh::new();
    //     // Front face
    //     m.positions.push([-scale, -scale, scale]);
    //     m.normals.push([0.0, 0.0, 1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, -scale, scale]);
    //     m.normals.push([0.0, 0.0, -1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, scale, scale]);
    //     m.normals.push([1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([-scale, scale, scale]);
    //     m.normals.push([-1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     // Back face
    //     m.positions.push([-scale, -scale, -scale]);
    //     m.normals.push([0.0, 1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([-scale, scale, -scale]);
    //     m.normals.push([0.0, -1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, scale, -scale]);
    //     m.normals.push([0.0, 0.0, 1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, -scale, -scale]);
    //     m.normals.push([0.0, 0.0, -1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     // Top face
    //     m.positions.push([-scale, scale, -scale]);
    //     m.normals.push([1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([-scale, scale, scale]);
    //     m.normals.push([-1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, scale, scale]);
    //     m.normals.push([0.0, 1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, scale, -scale]);
    //     m.normals.push([0.0, -1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     // Bottom face
    //     m.positions.push([-scale, -scale, -scale]);
    //     m.normals.push([0.0, 0.0, 1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, -scale, -scale]);
    //     m.normals.push([0.0, 0.0, -1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, -scale, scale]);
    //     m.normals.push([1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([-scale, -scale, scale]);
    //     m.normals.push([-1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     // Right face
    //     m.positions.push([scale, -scale, -scale]);
    //     m.normals.push([0.0, 1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, scale, -scale]);
    //     m.normals.push([0.0, -1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, scale, scale]);
    //     m.normals.push([0.0, 0.0, 1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([scale, -scale, scale]);
    //     m.normals.push([0.0, 0.0, -1.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     // Left face
    //     m.positions.push([-scale, -scale, -scale]);
    //     m.normals.push([1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([-scale, -scale, scale]);
    //     m.normals.push([-1.0, 0.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([-scale, scale, scale]);
    //     m.normals.push([0.0, 1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.positions.push([-scale, scale, -scale]);
    //     m.normals.push([0.0, -1.0, 0.0]);
    //     m.uvs.push([0.0, 0.5]);

    //     m.indices = cube_indices();

    //     results.meshes.push(m);
    //     results
    // }

    // pub fn cube_indices() -> Vec<u32> {
    //     vec![
    //         0, 1, 2, 0, 2, 3, // front
    //         4, 5, 6, 4, 6, 7, // back
    //         8, 9, 10, 8, 10, 11, // top
    //         12, 13, 14, 12, 14, 15, // bottom
    //         16, 17, 18, 16, 18, 19, // right
    //         20, 21, 22, 20, 22, 23, // left
    //     ]
}
