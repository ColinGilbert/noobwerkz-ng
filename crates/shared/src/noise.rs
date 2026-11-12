use quick_noise::simd::{Simd, dispatch_simd};
use quick_noise::*;
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]

pub struct Noise2DParameters {
    pub seed: i64,           // Acts a second seed in combination with the grid's seed.
    pub octaves: usize,      // Number of noise passes.
    pub frequency: f32,      // How fast the noise changes across sample space.
    pub lacunarity: f32,     // Multiplier to the frequency after each subsequent octave.
    pub persistence: f32,    // Multiplier to the weight after each subsequent octave.
    pub amplitude: f32,      // The maximum absolute value the noise can produce (For Fbm).
    pub normalization: bool, // Whether or not to normalize the output.
    pub scaling_x: f32,      // How much is the noise stretched in each dimension.
    pub scaling_y: f32,
}

impl Noise2DParameters {
    pub fn from_json(json_string: String) -> Self {
        let deserialized_config: Self = serde_json::from_str(json_string.as_str()).unwrap();

        deserialized_config
    }
}

#[dispatch_simd(A)]
pub fn make_fbm_perlin_noise_2d(
    params: &Noise2DParameters,
    x: f64,
    y: f64,
    heightfield_size: usize,
    grid_seed: i64,
    results: &mut Vec<f32>,
) {
    let grid_2d = Grid::<2>::new(heightfield_size, heightfield_size)
        .grid_position(0.0, 0.0) // Position of the grid region in sample space.
        .sample_position(x, y)
        .seed(grid_seed); // Seed for deterministic random results.

    results.resize(heightfield_size * heightfield_size, 0.0);

    // // FBM Grid noise with all parameters.
    let _noise = grid_2d
        .builder::<Fbm, Perlin>()
        .seed(params.seed) // Acts a second seed in combination with the grid's seed.
        .octaves(params.octaves) // Number of noise passes.
        .frequency(params.frequency) // How fast the noise changes across sample space.
        .lacunarity(params.lacunarity) // Multiplier to the frequency after each subsequent octave.
        .persistence(params.persistence) // Multiplier to the weight after each subsequent octave.
        .amplitude(params.amplitude) // The maximum absolute value the noise can produce (For Fbm).
        .normalization(params.normalization) // Whether or not to normalize the output.
        .scaling(params.scaling_x, params.scaling_y) // How much is the noise stretched in each dimension.
        .initialize(true) // Setting to false adds noise to current values.
        .finalize(true) // Some combiners have a finalization stage.
        .fill(results.as_mut_slice());
}

#[dispatch_simd(A)]
pub fn make_fbm_simplex_noise_2d(
    params: &Noise2DParameters,
    x: f64,
    y: f64,
    heightfield_size: usize,
    grid_seed: i64,
    results: &mut Vec<f32>,
) {
    let grid_2d = Grid::<2>::new(heightfield_size, heightfield_size)
        .grid_position(0.0, 0.0) // Position of the grid region in sample space.
        .sample_position(x, y)
        .seed(grid_seed); // Seed for deterministic random results.

    results.resize(heightfield_size * heightfield_size, 0.0);

    // // FBM Grid noise with all parameters.
    BatchNoise::<2, Fbm, Simplex>::builder(grid_2d.x_iter(), grid_2d.y_iter())
        .seed(params.seed) // Acts a second seed in combination with the grid's seed.
        .octaves(params.octaves) // Number of noise passes.
        .frequency(params.frequency) // How fast the noise changes across sample space.
        .lacunarity(params.lacunarity) // Multiplier to the frequency after each subsequent octave.
        .persistence(params.persistence) // Multiplier to the weight after each subsequent octave.
        .amplitude(params.amplitude) // The maximum absolute value the noise can produce (For Fbm).
        .normalization(params.normalization) // Whether or not to normalize the output.
        .scaling(params.scaling_x, params.scaling_y) // How much is the noise stretched in each dimension.
        .initialize(true) // Setting to false adds noise to current values.
        .finalize(true) // Some combiners have a finalization stage.
        .fill(results.as_mut_slice());
}

pub fn make_noise_zero_or_above(noise: &mut Vec<f32>) {
    for n in noise.iter_mut() {
        *n = (*n + 1.0) * 0.5;
    }
}

pub fn make_noise_terraces(noise: &mut Vec<f32>, step: f32) {
    let step = step.round();
    for n in noise.iter_mut() {
        *n = (*n * step).round() / step;
    }
}
