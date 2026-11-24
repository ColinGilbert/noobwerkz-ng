use std::sync::Arc;

use rapier3d::geometry::SharedShape;

pub struct Prop3D {
    pub shape: Arc<SharedShape>,
}