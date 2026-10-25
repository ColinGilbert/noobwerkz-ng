use std::collections::HashMap;

/// This code is adapted from simple-particles, found at https://gitlab.com/porky11/simple-particles, by Fabio Krapohl.
use noobwerkz_shared::prng::*;

fn drain(accumulator: &mut f32, limit: usize) -> u32 {
    let owed = accumulator.floor();
    *accumulator -= owed;
    owed.clamp(0.0, limit as f32) as u32
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Additive,
    Alpha,
}

#[derive(Debug, Clone, Copy)]
pub enum EmitterShape {
    Point,
    Sphere { radius: f32 },
    Hemisphere { radius: f32 },
    Cone { radius: f32, angle: f32 },
    Box { size: [f32; 3] },
    Disc { radius: f32 },
    Ring { radius: f32 },
    Line { length: f32 },
    Rect { size: [f32; 2] },
}

pub struct Particle {
    pub position: glam::Vec3,
    pub velocity: glam::Vec3,
    pub age: f32,
    pub lifetime: f32,
    pub rotation: f32,
    pub spin: f32,
}

pub struct ParticleRender {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub size: f32,
    pub rotation: f32,
    pub life: f32,
}

pub struct Emitter {
    particles: Vec<Particle>,
    spawn_accumulator: f32,
    distance_accumulator: f32,
    age: f32,
    emitting: bool,
    tint: [f32; 4],
    rng: Prng,
}

#[derive(Clone)]
pub struct ParticleEffect {
    pub texture: String,
    pub sheet: [u32; 2],
    pub blend: BlendMode,
    pub max_particles: usize,
    pub looping: bool,
    pub duration: f32,
    pub rate: f32,
    pub rate_distance: f32,
    pub burst: u32,
    pub shape: EmitterShape,
    pub lifetime: [f32; 2],
    pub speed: [f32; 2],
    pub gravity: glam::Vec3,
    pub drag: f32,
    pub size: Curve,
    pub color: Gradient,
    pub spin: [f32; 2],
}


pub struct ParticleSystem {
    effects: HashMap<String, ParticleEffect>,
    emitters: Vec<ActiveEmitter>,
    next_seed: u32,
    next_id: u64,
    wind: glam::Vec3,
}

pub struct EffectDraw {
    pub texture: String,
    pub sheet: [u32; 2],
    pub blend: BlendMode,
    pub instances: Vec<ParticleRender>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EmitterId(u64);

struct ActiveEmitter {
    id: EmitterId,
    effect: String,
    emitter: Emitter,
    origin: glam::Vec3,
    velocity: glam::Vec3,
}

#[derive(Clone)]
pub struct Curve {
    keys: Vec<[f32; 2]>,
}

impl Curve {
    fn constant(value: f32) -> Self {
        Self {
            keys: vec![[0.0, value]],
        }
    }

    #[must_use]
    pub fn linear(start: f32, end: f32) -> Self {
        Self {
            keys: vec![[0.0, start], [1.0, end]],
        }
    }

    #[must_use]
    pub fn from_keys(mut keys: Vec<[f32; 2]>) -> Self {
        keys.sort_by(|a, b| a[0].total_cmp(&b[0]));
        if keys.is_empty() {
            Self::constant(1.0)
        } else {
            Self { keys }
        }
    }

    #[must_use]
    pub fn keys(&self) -> &[[f32; 2]] {
        &self.keys
    }

    #[must_use]
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


#[derive(Debug, Clone)]
pub struct Gradient {
    keys: Vec<(f32, [f32; 4])>,
}

impl Gradient {
    #[must_use]
    pub fn linear(start: [f32; 4], end: [f32; 4]) -> Self {
        Self {
            keys: vec![(0.0, start), (1.0, end)],
        }
    }

    #[must_use]
    pub fn from_stops(mut keys: Vec<(f32, [f32; 4])>) -> Self {
        keys.sort_by(|a, b| a.0.total_cmp(&b.0));
        if keys.is_empty() {
            Self::constant([1.0, 1.0, 1.0, 1.0])
        } else {
            Self { keys }
        }
    }

    #[must_use]
    pub fn stops(&self) -> &[(f32, [f32; 4])] {
        &self.keys
    }

    fn constant(color: [f32; 4]) -> Self {
        Self {
            keys: vec![(0.0, color)],
        }
    }

    #[must_use]
    pub fn sample(&self, time: f32) -> [f32; 4] {
        let (Some(&(first_time, first_color)), Some(&(last_time, last_color))) =
            (self.keys.first(), self.keys.last())
        else {
            return [1.0; 4];
        };
        if time <= first_time {
            return first_color;
        }
        if time >= last_time {
            return last_color;
        }
        for window in self.keys.windows(2) {
            let &[(left_time, left_color), (right_time, right_color)] = window else {
                continue;
            };
            if time <= right_time {
                let span = right_time - left_time;
                let blend = if span > 1e-6 {
                    (time - left_time) / span
                } else {
                    0.0
                };
                let mut result = [0.0; 4];
                let channels = left_color.into_iter().zip(right_color);
                for (slot, (left, right)) in result.iter_mut().zip(channels) {
                    *slot = (right - left).mul_add(blend, left);
                }
                return result;
            }
        }
        last_color
    }
}

pub const fn set_shape_radius(shape: &mut EmitterShape, value: f32) {
    match shape {
        EmitterShape::Sphere { radius }
        | EmitterShape::Hemisphere { radius }
        | EmitterShape::Cone { radius, .. }
        | EmitterShape::Disc { radius }
        | EmitterShape::Ring { radius } => *radius = value,
        _ => {}
    }
}

pub const fn set_shape_angle(shape: &mut EmitterShape, value: f32) {
    if let EmitterShape::Cone { angle, .. } = shape {
        *angle = value;
    }
}

impl Emitter {
    #[must_use]
    pub const fn new(seed: u32) -> Self {
        Self {
            particles: Vec::new(),
            spawn_accumulator: 0.0,
            distance_accumulator: 0.0,
            age: 0.0,
            emitting: true,
            tint: [1.0; 4],
            rng: Prng::new(seed),
        }
    }

    pub const fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
    }

    #[must_use]
    pub const fn tint(&self) -> [f32; 4] {
        self.tint
    }

    #[must_use]
    pub const fn finished(&self) -> bool {
        !self.emitting && self.particles.is_empty()
    }

    pub const fn stop(&mut self) {
        self.emitting = false;
    }

    pub fn update(
        &mut self,
        effect: &ParticleEffect,
        delta_time: f32,
        origin: glam::Vec3,
        velocity: glam::Vec3,
        wind: glam::Vec3,
    ) {
        let was_fresh = self.age == 0.0;
        self.age += delta_time;

        if self.emitting {
            if was_fresh {
                for _ in 0..effect.burst {
                    self.spawn(effect, origin);
                }
            }
            if effect.rate > 0.0 {
                self.spawn_accumulator = effect.rate.mul_add(delta_time, self.spawn_accumulator);
                for _ in 0..drain(&mut self.spawn_accumulator, effect.max_particles) {
                    self.spawn(effect, origin);
                }
            }
            if effect.rate_distance > 0.0 {
                self.distance_accumulator = (effect.rate_distance * velocity.length())
                    .mul_add(delta_time, self.distance_accumulator);
                for _ in 0..drain(&mut self.distance_accumulator, effect.max_particles) {
                    self.spawn(effect, origin);
                }
            }
            if !effect.looping && self.age >= effect.duration {
                self.emitting = false;
            }
        }

        let drag = (effect.drag.mul_add(-delta_time, 1.0)).max(0.0);
        let mut index = 0;
        while let Some(particle) = self.particles.get_mut(index) {
            particle.age += delta_time;
            if particle.age >= particle.lifetime {
                self.particles.swap_remove(index);
                continue;
            }
            particle.velocity += (effect.gravity + wind) * delta_time;
            particle.velocity *= drag;
            particle.position += particle.velocity * delta_time;
            particle.rotation = particle.spin.mul_add(delta_time, particle.rotation);
            index += 1;
        }
    }

    fn spawn(&mut self, effect: &ParticleEffect, origin: glam::Vec3) {
        if self.particles.len() >= effect.max_particles {
            return;
        }
        let (offset, direction) = self.sample_shape(&effect.shape);
        let speed = self.rng.range(effect.speed);
        self.particles.push(Particle {
            position: origin + offset,
            velocity: direction * speed,
            age: 0.0,
            lifetime: self.rng.range(effect.lifetime).max(0.01),
            rotation: 0.0,
            spin: self.rng.range(effect.spin),
        });
    }

    fn sample_shape(&mut self, shape: &EmitterShape) -> (glam::Vec3, glam::Vec3) {
        let up = glam::Vec3::Y;
        match *shape {
            EmitterShape::Point => (glam::Vec3::ZERO, up),
            EmitterShape::Sphere { radius } => {
                let direction = self.rng.direction();
                (direction * (radius * self.rng.unit()), direction)
            }
            EmitterShape::Cone { radius, angle } => {
                let half = angle.to_radians();
                let polar = half * self.rng.unit().sqrt();
                let azimuth = self.rng.unit() * std::f32::consts::TAU;
                let direction = glam::Vec3::from_array([
                    polar.sin() * azimuth.cos(),
                    polar.cos(),
                    polar.sin() * azimuth.sin(),
                ]).normalize();
                let disk = radius * self.rng.unit().sqrt();
                let offset = glam::Vec3::from_array([azimuth.cos() * disk, 0.0, azimuth.sin() * disk]);
                (offset, direction)
            }
            EmitterShape::Box { size } => {
                let [size_x, size_y, size_z] = size;
                let offset = glam::Vec3::from_array([
                    (self.rng.unit() - 0.5) * size_x,
                    (self.rng.unit() - 0.5) * size_y,
                    (self.rng.unit() - 0.5) * size_z,
                ]);
                (offset, up)
            }
            EmitterShape::Hemisphere { radius } => {
                let mut direction = self.rng.direction();
                direction[1] = direction[1].abs();
                (direction * (radius * self.rng.unit()), direction)
            }
            EmitterShape::Disc { radius } => {
                let azimuth = self.rng.unit() * std::f32::consts::TAU;
                let distance = radius * self.rng.unit().sqrt();
                let offset = glam::Vec3::from_array([azimuth.cos() * distance, 0.0, azimuth.sin() * distance]);
                (offset, up)
            }
            EmitterShape::Ring { radius } => {
                let azimuth = self.rng.unit() * std::f32::consts::TAU;
                let offset = glam::Vec3::from_array([azimuth.cos() * radius, 0.0, azimuth.sin() * radius]);
                (offset, up)
            }
            EmitterShape::Line { length } => {
                let offset = glam::Vec3::from_array([(self.rng.unit() - 0.5) * length, 0.0, 0.0]);
                (offset, up)
            }
            EmitterShape::Rect {
                size: [size_x, size_z],
            } => {
                let offset = glam::Vec3::from_array([
                    (self.rng.unit() - 0.5) * size_x,
                    0.0,
                    (self.rng.unit() - 0.5) * size_z,
                ]);
                (offset, up)
            }
        }
    }

    pub fn render_into(&self, effect: &ParticleEffect, out: &mut Vec<ParticleRender>) {
        for particle in &self.particles {
            let life = (particle.age / particle.lifetime).clamp(0.0, 1.0);
            let mut color = effect.color.sample(life);
            for (channel, factor) in color.iter_mut().zip(self.tint) {
                *channel *= factor;
            }
            out.push(ParticleRender {
                position: particle.position.into(),
                color,
                size: effect.size.sample(life),
                rotation: particle.rotation,
                life,
            });
        }
    }
}

impl ParticleSystem {
    #[must_use]
    pub fn new() -> Self {
        Self {
            effects: HashMap::new(),
            emitters: Vec::new(),
            next_seed: 1,
            next_id: 1,
            wind: glam::Vec3::ZERO,
        }
    }

    pub fn set_wind(&mut self, wind: glam::Vec3) {
        self.wind = wind;
    }

    pub fn register(&mut self, name: &str, effect: ParticleEffect) {
        self.effects.insert(name.to_string(), effect);
    }

    pub fn spawn(&mut self, name: &str, origin: glam::Vec3) -> EmitterId {
        self.spawn_tinted(name, origin, [1.0; 4])
    }

    pub fn spawn_tinted(&mut self, name: &str, origin: glam::Vec3, tint: [f32; 4]) -> EmitterId {
        let id = EmitterId(self.next_id);
        self.next_id += 1;
        if !self.effects.contains_key(name) {
            return id;
        }
        self.next_seed = self
            .next_seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let mut emitter = Emitter::new(self.next_seed);
        emitter.set_tint(tint);
        self.emitters.push(ActiveEmitter {
            id,
            effect: name.to_string(),
            emitter,
            origin,
            velocity: glam::Vec3::ZERO,
        });
        id
    }

    pub fn set_emitter_motion(&mut self, id: EmitterId, origin: glam::Vec3, velocity: glam::Vec3) {
        if let Some(active) = self.emitters.iter_mut().find(|active| active.id == id) {
            active.origin = origin;
            active.velocity = velocity;
        }
    }

    pub fn set_emitter_tint(&mut self, id: EmitterId, tint: [f32; 4]) {
        if let Some(active) = self.emitters.iter_mut().find(|active| active.id == id) {
            active.emitter.set_tint(tint);
        }
    }

    pub fn stop_emitter(&mut self, id: EmitterId) {
        if let Some(active) = self.emitters.iter_mut().find(|active| active.id == id) {
            active.emitter.stop();
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let effects = &self.effects;
        let wind = self.wind;
        self.emitters.retain_mut(|active| {
            let Some(effect) = effects.get(&active.effect) else {
                return false;
            };
            active
                .emitter
                .update(effect, delta_time, active.origin, active.velocity, wind);
            !active.emitter.finished()
        });
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.emitters.is_empty()
    }

    #[must_use]
    pub fn collect(&self) -> Vec<EffectDraw> {
        let mut by_effect: HashMap<&str, Vec<ParticleRender>> = HashMap::new();
        for active in &self.emitters {
            let Some(effect) = self.effects.get(&active.effect) else {
                continue;
            };
            let instances = by_effect.entry(active.effect.as_str()).or_default();
            active.emitter.render_into(effect, instances);
        }
        by_effect
            .into_iter()
            .filter_map(|(name, instances)| {
                let effect = self.effects.get(name)?;
                Some(EffectDraw {
                    texture: effect.texture.clone(),
                    sheet: effect.sheet,
                    blend: effect.blend,
                    instances,
                })
            })
            .collect()
    }
}
