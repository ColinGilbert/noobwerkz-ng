use wgpu::util::DeviceExt;

use crate::texture;
#[repr(C)]
#[derive(Clone)]
pub struct ModelMaterial {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub normal_texture: texture::Texture,
    pub texture_bind_group: wgpu::BindGroup,
    pub uniform: ModelMaterialPBRUniform,
}

#[repr(C)]
// Derive the required traits for safe casting.
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelMaterialPBRUniform {
    pub color: [f32;4],
    pub metallic: f32,
    pub roughness: f32,
}

impl ModelMaterial {
    pub fn new(
        device: &wgpu::Device,
        name: &str,
        diffuse_texture: texture::Texture,
        normal_texture: texture::Texture,
        texture_layout: &wgpu::BindGroupLayout,
        pbr_layout: &wgpu::BindGroupLayout,
        color: [f32; 4],
        metallic: f32,
        roughness: f32,
    ) -> Self {

        let uniform = ModelMaterialPBRUniform{ color, metallic, roughness};

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("PBR Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });





        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_texture.sampler),
                },
            ],
            label: Some(name),
        });

        Self {
            name: String::from(name),
            diffuse_texture,
            normal_texture,
            texture_bind_group,
            uniform,
        }
    }
}
