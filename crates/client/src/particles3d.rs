use std::{
    collections::{HashMap, VecDeque},
    mem,
};

use crate::{index_types::ParticleSystemIndex, texture::Texture, vertex::Vertex};

pub struct Particle3D {
    pub position: glam::Vec3,
    pub velocity: glam::Vec3,
    pub acceleration: glam::Vec3,
    pub alpha: f32,
    pub rotation: f32,
    pub angular_acceleration: f32,
    pub life_left: web_time::Duration,
    pub alive: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
pub struct Particle3DRaw {
    pub position: [f32; 3],
    pub size: f32,
    pub color: [f32; 4],
    pub rotation: f32,
    pub life: f32,
    pub sheet: [f32; 2],
}

impl Particle3D {
    pub fn new(position: glam::Vec3, alpha: f32, lifespan: web_time::Duration) -> Self {
        Self {
            position,
            velocity: glam::Vec3::ZERO,
            acceleration: glam::Vec3::ZERO,
            alpha,
            rotation: 0.0,
            angular_acceleration: 0.0,
            life_left: lifespan,
            alive: true,
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.rotation += self.angular_acceleration;
        self.acceleration = glam::Vec3::ZERO;
        self.angular_acceleration = 0.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Additive,
    Alpha,
}

pub struct EffectDraw {
    pub texture: String,
    pub sheet: [u32; 2],
    pub blend: BlendMode,
    pub instances: Vec<Particle3DRaw>,
}

impl Vertex for Particle3DRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Particle3DRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // Model matrix
                wgpu::VertexAttribute {
                    offset: 0,
                    // Our vertex shader uses slots 0-4. We start at 5.
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct Attractor {
    pub to: glam::Vec3,
    pub force: f32,
}

pub struct Repellor {
    pub from: glam::Vec3,
    pub force: f32,
}

// pub struct Emitter3D {}
pub struct ParticleSystem3D {
    pub visible: bool,
    pub gravity: glam::Vec3,
    pub max_particles: usize,
    // pub particle_speed: f32,
    pub particle_drag: f32, // From 0.0 (no drag) to 1.0 (no movement)
    pub particle_spin: f32,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    pub attractor: Option<Attractor>,
    pub repellor: Option<Repellor>,
    pub particle_initial_size: f32,
    pub particle_size_curve: Curve,
    pub particle_initial_alpha: f32,
    pub particle_alpha_curve: Curve,
    pub particle_lifespan: web_time::Duration,

    particles: VecDeque<Particle3D>,
    texture: Texture,
    particle_vertices_ccw: [glam::Vec2; 4],
}

impl ParticleSystem3D {
    pub fn new(texture: Texture, particle_vertices_ccw: [glam::Vec2; 4]) -> Self {
        Self {
            visible: true,
            gravity: glam::Vec3::ZERO,
            max_particles: 50,
            // particle_speed: 1.0,
            particle_drag: 0.1,
            particle_spin: 0.0,
            position: glam::Vec3::ZERO,
            orientation: glam::Quat::IDENTITY,
            attractor: None,
            repellor: None,
            particle_initial_size: 1.0,
            particle_size_curve: Curve::constant(1.0),
            particle_initial_alpha: 1.0,
            particle_alpha_curve: Curve::constant(1.0),
            particle_lifespan: web_time::Duration::from_secs_f32(2.0),
            particles: VecDeque::new(),
            texture,
            particle_vertices_ccw,
        }
    }

    pub fn spawn_particle(&mut self) {
        let mut p = Particle3D::new(
            self.position,
            self.particle_initial_alpha,
            self.particle_lifespan,
        );
    }

    // TODO
    pub fn update(&mut self, dt: web_time::Duration) {
        if self.visible {
            for p in self.particles.iter_mut() {
                p.acceleration += self.gravity * dt.as_secs_f32();
                p.angular_acceleration += self.particle_spin * dt.as_secs_f32();
                if self.attractor.is_some() {
                    let attractor = self.attractor.as_ref().unwrap();
                    p.acceleration += attractor.to * attractor.force * dt.as_secs_f32();
                }
                if self.repellor.is_some() {
                    let repellor = self.repellor.as_ref().unwrap();
                    p.acceleration += repellor.from * repellor.force * dt.as_secs_f32();
                }
                p.acceleration *= self.particle_drag;
                p.life_left -= dt;
                let time_left_f32 = p.life_left.as_secs_f32();
                let lifespan_f32 = self.particle_lifespan.as_secs_f32();
                let time_left_ratio = time_left_f32 / lifespan_f32;
                p.alpha = self.particle_alpha_curve.sample(time_left_ratio);
                p.update();
                if time_left_f32 - dt.as_secs_f32() <= 0.0 {
                    p.alive = false;
                }
            }
            let mut still_checking = true;
            while still_checking {
                let p = self.particles.front(); // The particles are guaranteed to be in sorted order by time left
                if p.is_some() {
                    if p.unwrap().alive == false {
                        self.particles.pop_front();
                    } else {
                        still_checking = false;
                    }
                } else {
                    // If no particles in container
                    still_checking = false;
                }
            }
        }
    }
}

pub struct ParticleSystem3DManager {
    systems: HashMap<ParticleSystemIndex, ParticleSystem3D>,
    counter: usize,
}

impl ParticleSystem3DManager {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_system(&mut self, sys: ParticleSystem3D) -> ParticleSystemIndex {
        self.systems
            .insert(ParticleSystemIndex::new(self.counter), sys);
        self.counter += 1;
        return ParticleSystemIndex::new(self.counter);
    }

    pub fn remove_system(&mut self, sys: ParticleSystemIndex) -> bool {
        if self.systems.contains_key(&sys) {
            self.systems.remove(&sys);
            return true;
        }
        return false;
    }

    pub fn set_system_visibility(&mut self, sys: ParticleSystemIndex, visible: bool) -> bool {
        if self.systems.contains_key(&sys) {
            self.systems.get_mut(&sys).unwrap().visible = visible;
            return true;
        }
        return false;
    }

    pub fn update(&mut self, dt: web_time::Duration) {
        for ps in self.systems.iter_mut() {
            ps.1.update(dt);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Curve {
    keys: Vec<[f32; 2]>,
}

pub struct Rng {
    state: u32,
}

impl Rng {
    pub const fn new(seed: u32) -> Self {
        Self { state: seed | 1 }
    }

    const fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn unit(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / 16_777_216.0
    }

    pub fn range(&mut self, bounds: [f32; 2]) -> f32 {
        let [min, max] = bounds;
        (max - min).mul_add(self.unit(), min)
    }

    pub fn direction(&mut self) -> glam::Vec3 {
        let z = self.unit().mul_add(2.0, -1.0);
        let azimuth = self.unit() * std::f32::consts::TAU;
        let radius = (1.0 - z * z).max(0.0).sqrt();
        glam::Vec3::from([radius * azimuth.cos(), z, radius * azimuth.sin()])
    }
}

impl Curve {
    pub fn constant(value: f32) -> Self {
        Self {
            keys: vec![[0.0, value]],
        }
    }

    pub fn linear(start: f32, end: f32) -> Self {
        Self {
            keys: vec![[0.0, start], [1.0, end]],
        }
    }

    pub fn from_keys(mut keys: Vec<[f32; 2]>) -> Self {
        keys.sort_by(|a, b| a[0].total_cmp(&b[0]));
        if keys.is_empty() {
            Self::constant(1.0)
        } else {
            Self { keys }
        }
    }

    pub fn keys(&self) -> &[[f32; 2]] {
        &self.keys
    }

    pub fn sample(&self, time: f32) -> f32 {
        let (Some(&[first_time, first_value]), Some(&[last_time, last_value])) =
            (self.keys.first(), self.keys.last())
        else {
            return 1.0;
        };
        if time <= first_time {
            return first_value;
        }
        if time >= last_time {
            return last_value;
        }
        for window in self.keys.windows(2) {
            let &[[left_time, left_value], [right_time, right_value]] = window else {
                continue;
            };
            if time <= right_time {
                let span = right_time - left_time;
                let blend = if span > 1e-6 {
                    (time - left_time) / span
                } else {
                    0.0
                };
                return (right_value - left_value).mul_add(blend, left_value);
            }
        }
        last_value
    }
}
