pub struct LocalToLocalResults {
    pub translations: Vec<glam::Vec3>,
    pub rotations: Vec<glam::Quat>,
    pub scales: Vec<glam::Vec3>
}

impl LocalToLocalResults {
    pub fn new() -> Self {
        Self {
            translations: Vec::new(),
            rotations: Vec::new(),
            scales: Vec::new()
        }
    }
}

pub struct LocalToModelResults {
    pub model_matrices: Vec<glam::Mat4>,
}

impl LocalToModelResults {
    pub fn new() -> Self {
        Self {
            model_matrices: Vec::new()
        }
    }
}