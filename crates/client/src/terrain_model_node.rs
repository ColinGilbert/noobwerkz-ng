use crate::{index_types::*, instance3d::*};

pub struct TerrainModel3DNode {
    pub model_idx: TerrainModel3DIndex,
    pub instance: Instance3D,
    // pub visible: bool,
}

impl TerrainModel3DNode {
    pub fn new(model_idx: TerrainModel3DIndex, instance: Instance3D) -> Self {
        Self {
            model_idx,
            instance,
            // visible: true,
        }
    }
}
