use crate::{resource::*, model::*, serialized_model::SerializedMesh};


pub fn cube_serialized_mesh(scale: f32) {
    let mut m = SerializedMesh::new();
    // Front face
    m.positions.push([-scale, -scale, scale]);
    m.normals.push([0.0, 0.0, 1.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, -scale, scale]);
    m.normals.push([0.0, 0.0, -1.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, scale, scale]);
    m.normals.push([1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([-scale, scale, scale]);
    m.normals.push([-1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    // Back face
    m.positions.push([-scale, -scale, -scale]);
    m.normals.push([0.0, 1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([-scale, scale, -scale]);
    m.normals.push([0.0, -1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, scale, -scale]);
    m.normals.push([0.0, 0.0, 1.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, -scale, -scale]);
    m.normals.push([0.0, 0.0, -1.0]);
    m.uvs.push([0.0, 0.5]);

    // Top face
    m.positions.push([-scale, scale, -scale]);
    m.normals.push([1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([-scale, scale, scale]);
    m.normals.push([-1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, scale, scale]);
    m.normals.push([0.0, 1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, scale, -scale]);
    m.normals.push([0.0, -1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    // Bottom face
    m.positions.push([-scale, -scale, -scale]);
    m.normals.push([0.0, 0.0, 1.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, -scale, -scale]);
    m.normals.push([0.0, 0.0, -1.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, -scale, scale]);
    m.normals.push([1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([-scale, -scale, scale]);
    m.normals.push([-1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    // Right face
    m.positions.push([scale, -scale, -scale]);
    m.normals.push([0.0, 1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, scale, -scale]);
    m.normals.push([0.0, -1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, scale, scale]);
    m.normals.push([0.0, 0.0, 1.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([scale, -scale, scale]);
    m.normals.push([0.0, 0.0, -1.0]);
    m.uvs.push([0.0, 0.5]);

    
    // Left face
    m.positions.push([-scale, -scale, -scale]);
    m.normals.push([1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);
    
    m.positions.push([-scale, -scale, scale]);
    m.normals.push([-1.0, 0.0, 0.0]);
    m.uvs.push([0.0, 0.5]);

    m.positions.push([-scale, scale, scale]);
    m.normals.push([0.0, 1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);
    
    m.positions.push([-scale, scale, -scale]);
    m.normals.push([0.0, -1.0, 0.0]);
    m.uvs.push([0.0, 0.5]);
    
    
    m.indices = cube_indices();
}

pub fn cube_indices() -> Vec<u32> {
    vec![
        0, 1, 2, 0, 2, 3, // front
        4, 5, 6, 4, 6, 7, // back
        8, 9, 10, 8, 10, 11, // top
        12, 13, 14, 12, 14, 15, // bottom
        16, 17, 18, 16, 18, 19, // right
        20, 21, 22, 20, 22, 23, // left
    ]
}
