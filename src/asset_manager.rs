// This code's job is to ensure we can recreate scenes by holding onto descriptions of the models and textures that are loaded into the game.
// It'll ensure that no 
use std::{collections::*};

use crate::{texture::Texture};

pub struct AssetManager {
    pub models_by_name: HashMap<String, usize>,
    pub skinned_models_by_name: HashMap<String, usize>,
    pub textures_by_name: HashMap<String, Texture>,

    // These are especially useful during serialization/deserialization
    pub model_names: HashMap<usize, String>,
    pub skinned_model_names: HashMap<usize, String>,
    pub texture_names: HashMap<usize, String>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            models_by_name: HashMap::new(),
            skinned_models_by_name: HashMap::new(),
            textures_by_name: HashMap::new(),
            model_names: HashMap::new(),
            skinned_model_names: HashMap::new(),
            texture_names: HashMap::new(),
        }
    }
}