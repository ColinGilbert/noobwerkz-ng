use crate::{index_types::*, model_material::ModelMaterial, model3d::TexturedMesh3D, vertex::Vertex};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TerrainModel3DVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    _filler: [f32; 2],
    // pub tangent: [f32; 3],
    // pub bitangent: [f32; 3],
}

impl TerrainModel3DVertex {
    pub fn new() -> Self {
        Self {
            position: [0.0; 3],
            normal: [0.0; 3],
            _filler: [0.0; 2],
            // tangent: [0.0; 3],
            // bitangent: [0.0; 3],
        }
    }
}

impl Vertex for TerrainModel3DVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TerrainModel3DVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Positions
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normals
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}


pub struct TerrainModel3D {
pub meshes: TexturedMeshes<TexturedMesh3D>,
    pub materials: ModelMaterials<ModelMaterial>,
}

impl TerrainModel3D {
    pub fn new() -> Self {
        Self {
            meshes: TexturedMeshes::new(),
            materials: ModelMaterials::new(),
        }
    }
}