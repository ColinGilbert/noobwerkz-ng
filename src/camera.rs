use core::f32::consts::FRAC_PI_2;

// pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols(
//     glam::Vec4::from_array([1.0, 0.0, 0.0, 0.0]),
//     glam::Vec4::from_array([0.0, 1.0, 0.0, 0.0]),
//     glam::Vec4::from_array([0.0, 0.0, 0.5, 0.0]),
//     glam::Vec4::from_array([0.0, 0.0, 0.5, 1.0]),
// );

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

pub struct Camera {
    pub position: glam::Vec3,
    pub yaw_rad: f32,
    pub pitch_rad: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: glam::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            yaw_rad: 0.0,
            pitch_rad: 0.0,
        }
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        let (sin_pitch, cos_pitch) = libm::sincosf(self.pitch_rad);
        let (sin_yaw, cos_yaw) = libm::sincosf(self.yaw_rad);
        let focal_point = glam::Vec3 {
            x: cos_pitch * cos_yaw,
            y: sin_pitch,
            z: cos_pitch * sin_yaw,
        };
        let results = glam::Mat4::look_at_lh(self.position, focal_point.normalize(), glam::Vec3::Y);
        results
    }
}

pub struct Projection {
    pub aspect_ratio: f32,
    pub fovy_rad: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Projection {
    pub fn new(height: u32, width: u32) -> Self {
        Projection {
            aspect_ratio: width as f32 / height as f32,
            fovy_rad: 0.0,
            znear: 0.0001,
            zfar: 1000.0,
        }
    }

    pub fn resize(&mut self, height: u32, width: u32) -> () {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        //OPENGL_TO_WGPU_MATRIX * glam::Mat4::perspective_rh(self.fovy_rad, self.aspect_ratio, self.znear, self.zfar);
        let results =
            glam::Mat4::perspective_lh(self.fovy_rad, self.aspect_ratio, self.znear, self.zfar);
        results
    }
}

// This represents the camera uniform that lives on the GPU
pub struct CameraUniform {
    pub view_position: glam::Vec4,
    pub view_projection: glam::Mat4,
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: glam::Vec4::from_array([0.0, 0.0, 0.0, 0.0]),
            view_projection: glam::Mat4::IDENTITY,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = glam::Vec4::from_array([camera.position.x, camera.position.y, camera.position.z, 1.0]);
        self.view_projection = (projection.calc_matrix() * camera.calc_matrix()).into()
    }
}
