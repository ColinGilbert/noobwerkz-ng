/// This code is adapted from simple-particles, found https://gitlab.com/porky11/simple-particles, by Fabio Krapohl

pub struct Prng {
    state: u32,
}

impl Prng {
    pub const fn new(seed: u32) -> Self {
        Self { state: seed | 1 }
    }

    pub const fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    pub fn unit(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / 16_777_216.0
    }

    pub fn range(&mut self, bounds: [f32; 2]) -> f32 {
        let [min, max] = bounds;
        (max - min).mul_add(self.unit(), min)
    }

    pub fn direction(&mut self) -> glam::Vec3 {
        let z = self.unit().mul_add(2.0, -1.0);
        let azimuth = self.unit() * std::f32::consts::TAU;
        let radius = z.mul_add(-z, 1.0).max(0.0).sqrt();
        glam::Vec3::from_array([radius * azimuth.cos(), z, radius * azimuth.sin()])
    }
}