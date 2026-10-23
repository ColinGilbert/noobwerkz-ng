use crate::{MAX_BONES, operation_result::LocalToLocalResults};
use std::iter::zip;

pub fn blend_multiplicative(first: &LocalToLocalResults, second : &LocalToLocalResults, weight: f32, mask: &Vec<bool>) -> LocalToLocalResults {
    let mut results = LocalToLocalResults::new();
    for (i, vals) in zip(first.translations, second.translations).enumerate() {
        if mask[i] {
            results.translations[i] = vals.0.lerp(vals.1, weight);
        }
    }

    for (i, vals) in zip(first.rotations, second.rotations).enumerate() {
        if mask[i] {
            results.rotations[i] = vals.0.slerp(vals.1, weight);
        }
    }

    for (i, vals) in zip(first.scales, second.scales).enumerate() {
        if mask[i] {
            results.scales[i] = vals.0.lerp(vals.1, weight);
        }
    }

    results
}