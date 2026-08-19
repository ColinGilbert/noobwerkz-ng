use rapier3d::pipeline::PhysicsWorld;
use salva3d::LiquidWorld;

static LIQUIDS_TIMESTEP: f32 = 1.0 / 200.0;
static NO_LIQUIDS_TIMESTEP: f32 = 1.0 / 60.0;
pub struct PhysicsContext {
    pub rigid_world: PhysicsWorld,
    pub liquid_world: Option<LiquidWorld>,
}

pub struct LiquidWorldProperties {
    pub particle_radius: f32,
    pub smoothing_factor: f32,
    pub boundary_force_coefficient: f32,
}

impl PhysicsContext {
    pub fn new(gravity: &glam::Vec3, has_liquids: bool) -> Self {
        let mut rigid_world = PhysicsWorld::new();
        rigid_world.gravity = rapier3d::math::Vector3{ x: gravity.x, y: gravity.y, z: gravity.z };
        if has_liquids {
            rigid_world.integration_parameters.dt = LIQUIDS_TIMESTEP;
        } else {
            rigid_world.integration_parameters.dt = NO_LIQUIDS_TIMESTEP;
        }
        Self {
            rigid_world,
            liquid_world: None,
        }
    }
}