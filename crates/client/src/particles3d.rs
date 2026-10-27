use std::collections::HashMap;

use crate::{texture::Texture, vertex::Vertex};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Particle3DVertex {
    pub position: [f32; 3],
    pub size: f32,
    pub color: [f32; 4],
    pub rotation: f32,
    pub life: f32,
    pub uv: [f32; 2],
}

impl Particle3DVertex {
    pub fn new() -> Self {
        Self {
            position: [0.0; 3],
            size: 1.0,
            color: [1.0; 4],
            rotation: 0.0,
            life: 10.0,
            uv: [0.0; 2],
        }
    }
}

impl Vertex for Particle3DVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Particle3DVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Positions
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Size
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                //Rotation
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
                // Life
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                // UV
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct Particle3DContext {
    pub textures: HashMap<String, Texture>,
    pub instance_buffer: wgpu::Buffer,
    pub instance_capacity: u64,
    pub batches: Vec<ParticlesBatch>,
}

impl Particle3DContext {
    pub fn new(instance_buffer: wgpu::Buffer, instance_capacity: u64) -> Self {
        Self {
            textures: HashMap::new(),
            instance_buffer,
            instance_capacity,
            batches: Vec::new(),
        }
    }

    pub fn register_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        name: &str,
        rgba: &[u8],
        // width: u32,
        // height: u32,
        texture_layout: &wgpu::BindGroupLayout,
    ) -> bool {
        let texture = Texture::from_bytes(device, queue, rgba, name, false, texture_layout);
        if texture.is_ok() {
            self.textures.insert(name.to_owned(), texture.unwrap());
            return true;
        }
        return false;
    }
}

pub struct ParticlesBatch {
    pub additive: bool,
    pub texture: String,
    pub start: u32,
    pub count: u32,
}
