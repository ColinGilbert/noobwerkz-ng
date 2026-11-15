use noobwerkz_shared::grid2d::Grid2D;

pub struct SimulationState {
    pub water: Grid2D<f32>,
    pub terrain: Grid2D<f32>,
    pub suspended_sediment: Grid2D<f32>,
    pub surface_normals: Grid2D<glam::Vec3>,
}