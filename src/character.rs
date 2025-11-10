pub struct Character {
    pub skinned_model_idx: usize,
    pub gfx_instance_idx: usize,
    pub skeletal_idx: usize,
    pub animation_idx: usize,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    // TODO:
    // Colliders, animations, pathfinding, AI
}