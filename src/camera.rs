pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols(
    glam::Vec4::from_array([1.0, 0.0, 0.0, 0.0]),
    glam::Vec4::from_array([0.0, 1.0, 0.0, 0.0]),
    glam::Vec4::from_array([0.0, 0.0, 0.5, 0.0]),
    glam::Vec4::from_array([0.0, 0.0, 0.5, 1.0]),
);

pub struct Camera {
    pub position: glam::Vec3,
    pub yaw: glam::Vec3,
    pub pitch: glam::Vec3,
}

impl Camera {
    pub fn new() -> Self {
      Camera {
        position: glam::Vec3{x: 0.0, y: 0.0, z:0.0},
        yaw: glam::Vec3{x: 0.0, y: 0.0, z: 0.0},
        pitch: glam::Vec3{x: 0.0, y: 0.0, z: 0.0}
      }
    }
}

pub struct CameraProjection {
    pub aspect: f32,
    pub fovy: glam::Vec3,
    pub znear: f32,
    pub zfar: f32,
}

impl CameraProjection {
    pub fn new(height: f32, width: f32) -> Self {
        CameraProjection {
           aspect: height/width,
           fovy: glam::Vec3{x: 0.0, y: 1.0, z: 0.0},
           znear: 0.0001,
           zfar: 1000.0
        }
    }
}

pub struct CameraUniform {
    pub view_position: glam::Vec4,
    pub view_projection: glam::Mat4
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: glam::Vec4::from_array([0.0, 0.0, 0.0, 0.0]),
            view_projection: glam::Mat4::IDENTITY,
        }
    }
}