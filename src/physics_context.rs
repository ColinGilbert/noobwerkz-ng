use rapier3d::pipeline::PhysicsWorld;
use salva3d::integrations::rapier::FluidsPipeline;


const LIQUIDS_HZ: u32 = 240;
const NO_LIQUIDS_HZ: u32 = 60;

const LIQUIDS_TIMESTEP: f32 = 1.0 / 240.0;
const NO_LIQUIDS_TIMESTEP: f32 = 1.0 / 60.0;

pub struct PhysicsContext {
    pub rigid_world: PhysicsWorld,
    pub fluids_pipeline: Option<FluidsPipeline>,
}

pub struct LiquidWorldProperties {
    pub has_liquid_world: bool,
    pub particle_radius: f32,
    pub smoothing_factor: f32,
    // pub boundary_force_coefficient: f32,
}

impl PhysicsContext {
    pub fn new(gravity: &glam::Vec3, liquid_world_properties: LiquidWorldProperties) -> Self {
        let mut rigid_world = PhysicsWorld::new();
        rigid_world.gravity = rapier3d::math::Vector3{ x: gravity.x, y: gravity.y, z: gravity.z };
        let mut fluids_pipeline: Option<FluidsPipeline> = None;
        if liquid_world_properties.has_liquid_world {
            rigid_world.integration_parameters.dt = LIQUIDS_TIMESTEP;
            fluids_pipeline = Some(FluidsPipeline::new(liquid_world_properties.particle_radius, liquid_world_properties.smoothing_factor));
        } else {
            rigid_world.integration_parameters.dt = NO_LIQUIDS_TIMESTEP;
        }
        Self {
            rigid_world,
            fluids_pipeline,
        }
    }
}