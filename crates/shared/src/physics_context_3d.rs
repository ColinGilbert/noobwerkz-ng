use rapier3d::pipeline::PhysicsWorld;


pub struct Physics3DContext {
    pub rigid_world: PhysicsWorld,
}

impl Physics3DContext {
    pub fn new(gravity: &glam::Vec3) -> Self {
        let mut rigid_world = PhysicsWorld::new();
        rigid_world.gravity = rapier3d::math::Vector3{ x: gravity.x, y: gravity.y, z: gravity.z };
        Self {
            rigid_world,
        }
    }
}