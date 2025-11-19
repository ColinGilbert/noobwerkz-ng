pub struct Character {
    pub skinned_model_idx: usize,
    pub instance_idx: usize,
    pub animation_player_idx: usize,
    pub position: glam::Vec3,
    pub orientation: glam::Quat,
    // TODO:
    // Colliders, animation blending, IK, pathfinding, AI
}