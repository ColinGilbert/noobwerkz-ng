use quick_noise::simd::{Simd, dispatch_simd};
use quick_noise::{BatchNoise, Fbm, Grid, Perlin};

#[dispatch_simd(A)]
pub fn make_perlin_noise() {
    // Dispatch simd here to avoid repeated dispatching
    // in every iteration.
}