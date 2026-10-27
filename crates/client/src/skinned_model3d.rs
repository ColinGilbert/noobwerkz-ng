use noobwerkz_skeletal_animation::animation::Animation;
use noobwerkz_skeletal_animation::animation_graph::AnimationIndex;
use noobwerkz_skeletal_animation::animation_graph::Animations;
use noobwerkz_shared::{skeleton::*};
use crate::index_types::*;
use crate::model_material::*;
use crate::model3d::*;
use crate::vertex::*;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

#[repr(C)]
pub struct SkinnedModel3D {
    pub meshes: SkinnedTexturedMeshes3D<SkinnedTexturedMesh3D>,
    pub materials: ModelMaterials<ModelMaterial>,
    pub name: String,
    pub skeleton: Arc<Skeleton>,
    pub animations_idx_by_name: HashMap<String, AnimationIndex>,
    pub animation_names: HashMap<AnimationIndex, String>,
    pub animations: Animations<Animation>,
}

impl SkinnedModel3D {
    pub fn new() -> Self {
        Self {
            meshes: SkinnedTexturedMeshes3D::new(),
            materials: ModelMaterials::new(),
            name: "".to_owned(),
            skeleton: Arc::new(Skeleton::new()),
            animations_idx_by_name: HashMap::new(),
            animation_names: HashMap::new(),
            animations: Animations::new(),
        }
    }
}

#[repr(C)]
pub struct SkinnedTexturedMesh3D {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: ModelMaterialIndex,
    pub translation: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,
    pub dimensions: glam::Vec3,
    //pub matrices_texture: Option<wgpu::Texture>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SkinnedModelVertex3D {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
    pub bone_indices: [u32; 4],
    pub bone_weights: [f32; 4],
}

impl SkinnedModelVertex3D {
    pub fn new() -> Self {
        Self {
            position: [0.0; 3],
            tex_coords: [0.0; 2],
            normal: [0.0; 3],
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
            bone_indices: [0; 4],
            bone_weights: [0.0; 4],
        }
    }
    pub fn from_vert(model_vert: &Model3DVertex) -> Self {
        Self {
            position: model_vert.position,
            tex_coords: model_vert.tex_coords,
            normal: model_vert.normal,
            tangent: model_vert.tangent,
            bitangent: model_vert.bitangent,
            bone_indices: [0; 4],
            bone_weights: [0.0; 4],
        }
    }
}

impl Vertex for SkinnedModelVertex3D {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SkinnedModelVertex3D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Positions
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // UV
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Normals
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Tangent
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Bitangent
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 11]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Bone indices
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 14]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Uint32x4,
                },
                // Bone weights
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub trait DrawSkinnedModel<'a> {
    #[allow(unused)]
    fn draw_skinned_mesh(
        &mut self,
        mesh: &'a SkinnedTexturedMesh3D,
        material: &'a ModelMaterial,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        bone_matrices_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_skinned_mesh_instanced(
        &mut self,
        mesh: &'a SkinnedTexturedMesh3D,
        material: &'a ModelMaterial,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        bone_matrices_bind_group: &'a wgpu::BindGroup,
    );
    #[allow(unused)]
    fn draw_skinned_model(
        &mut self,
        model: &'a SkinnedModel3D,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        bone_matrices_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_skinned_model_instanced(
        &mut self,
        model: &'a SkinnedModel3D,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        bone_matrices_bind_group: &'a wgpu::BindGroup,
    );
    #[allow(unused)]
    fn draw_skinned_model_instanced_with_material(
        &mut self,
        model: &'a SkinnedModel3D,
        material: &'a ModelMaterial,
        instances: Range<u32>,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
        bone_matrices_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawSkinnedModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_skinned_mesh(
        &mut self,
        mesh: &'b SkinnedTexturedMesh3D,
        material: &'b ModelMaterial,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
        bone_matrices_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_skinned_mesh_instanced(
            mesh,
            material,
            0..1,
            camera_bind_group,
            light_bind_group,
            bone_matrices_bind_group,
        );
    }

    fn draw_skinned_mesh_instanced(
        &mut self,
        mesh: &'b SkinnedTexturedMesh3D,
        material: &'b ModelMaterial,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
        bone_matrices_bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.set_bind_group(2, light_bind_group, &[]);
        self.set_bind_group(3, bone_matrices_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_skinned_model(
        &mut self,
        model: &'b SkinnedModel3D,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
        bone_matrices_bind_group: &'b wgpu::BindGroup,
    ) {
        self.draw_skinned_model_instanced(
            model,
            0..1,
            camera_bind_group,
            light_bind_group,
            bone_matrices_bind_group,
        );
    }

    fn draw_skinned_model_instanced(
        &mut self,
        model: &'b SkinnedModel3D,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
        bone_matrices_bind_group: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_skinned_mesh_instanced(
                mesh,
                material,
                instances.clone(),
                camera_bind_group,
                light_bind_group,
                bone_matrices_bind_group,
            );
        }
    }

    fn draw_skinned_model_instanced_with_material(
        &mut self,
        model: &'b SkinnedModel3D,
        material: &'b ModelMaterial,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup,
        light_bind_group: &'b wgpu::BindGroup,
        bone_matrices_bind_group: &'b wgpu::BindGroup,
    ) {
        for mesh in &model.meshes {
            self.draw_skinned_mesh_instanced(
                mesh,
                material,
                instances.clone(),
                camera_bind_group,
                light_bind_group,
                bone_matrices_bind_group,
            );
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BoneMatrix {
    pub data: [[f32; 4]; 4],
}
