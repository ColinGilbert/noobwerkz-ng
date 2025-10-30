use glam::Vec4Swizzles;
use ozz_animation_rs::OzzBuf;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy)]
pub struct OzzTransform {
    pub scale: f32,
    pub rotation: glam::Quat,
    pub position: glam::Vec3,
}

pub trait OzzTrait
where
    Self: Send + Sync,
{
    fn update(&mut self, time: web_time::Duration);
    fn root(&self) -> glam::Mat4;
    fn bone_trans(&self) -> &[OzzTransform];
    fn spine_trans(&self) -> &[OzzTransform];
}

pub struct OzzPlayback {
    seek: f32,
    skeleton: Arc<ozz_animation_rs::Skeleton>,
    sample_job: ozz_animation_rs::SamplingJobArc,
    l2m_job: ozz_animation_rs::LocalToModelJobArc,
    models: Arc<RwLock<Vec<glam::Mat4>>>,
    bone_trans: Vec<OzzTransform>,
    spine_trans: Vec<OzzTransform>,
}

impl OzzPlayback {
    pub async fn new(
        skeleton: &Arc<ozz_animation_rs::Skeleton>,
        animation: &Arc<ozz_animation_rs::Animation>,
    ) -> Self {
        let mut o = OzzPlayback {
            seek: 0.0,
            skeleton: skeleton.clone(),
            sample_job: ozz_animation_rs::SamplingJob::default(),
            l2m_job: ozz_animation_rs::LocalToModelJob::default(),
            models: Arc::new(RwLock::new(vec![
                glam::Mat4::default();
                skeleton.num_joints()
            ])),
            bone_trans: Vec::new(),
            spine_trans: Vec::new(),
        };

        o.sample_job.set_animation(animation.clone());
        o.sample_job
            .set_context(ozz_animation_rs::SamplingContext::new(
                animation.num_tracks(),
            ));
        let sample_out = Arc::new(RwLock::new(vec![
            ozz_animation_rs::SoaTransform::default();
            skeleton.num_soa_joints()
        ]));
        o.sample_job.set_output(sample_out.clone());
        o.l2m_job.set_skeleton(skeleton.clone());
        o.l2m_job.set_input(sample_out.clone());
        o.l2m_job.set_output(o.models.clone());

        let mut bone_count = 0;
        let mut spine_count = 0;
        for i in 0..skeleton.num_joints() {
            let parent_id = skeleton.joint_parent(i);
            if parent_id as i32 == ozz_animation_rs::SKELETON_NO_PARENT {
                continue;
            }
            bone_count += 1;
            spine_count += 1;
            if skeleton.is_leaf(i as i16) {
                spine_count += 1;
            }
        }

        o.bone_trans.reserve(bone_count);
        o.spine_trans.reserve(spine_count);
        o
    }
}

impl OzzTrait for OzzPlayback {
    fn root(&self) -> glam::Mat4 {
        self.models.buf().unwrap()[0]
    }

    fn bone_trans(&self) -> &[OzzTransform] {
        &self.bone_trans
    }

    fn spine_trans(&self) -> &[OzzTransform] {
        &self.spine_trans
    }

    fn update(&mut self, dt: web_time::Duration) {
        let duration = self.sample_job.animation().unwrap().duration();
        // println!("Duration {}, dt {}", duration, dt.as_secs_f32());
        self.seek += dt.as_secs_f32();//* 1_000.0;
        self.seek %= duration;
        let ratio = self.seek / duration;

        println!("ratio {}", ratio);
        self.sample_job.set_ratio(ratio);
        self.sample_job.run().unwrap();
        self.l2m_job.run().unwrap();

        //println!("{:?}", self.bone_trans);
        self.bone_trans.clear();
        self.spine_trans.clear();

        let modals = self.models.buf().unwrap();
        for (i, current) in modals.iter().enumerate() {
            let parent_id = self.skeleton.joint_parent(i);
            if parent_id as i32 == ozz_animation_rs::SKELETON_NO_PARENT {
                continue;
            }
            let parent = &modals[parent_id as usize];

            let current_pos = current.w_axis.xyz();
            let parent_pos = parent.w_axis.xyz();
            let scale: f32 = (current_pos - parent_pos).length();

            let bone_dir = (current_pos - parent_pos).normalize();
            let dot1 = glam::Vec3::dot(bone_dir, parent.x_axis.xyz());
            let dot2 = glam::Vec3::dot(bone_dir, parent.z_axis.xyz());
            let binormal = if dot1.abs() < dot2.abs() {
                parent.x_axis.xyz()
            } else {
                parent.z_axis.xyz()
            };

            let bone_rot_y = glam::Vec3::cross(binormal, bone_dir).normalize();
            let bone_rot_z = glam::Vec3::cross(bone_dir, bone_rot_y).normalize();
            let bone_rot =
                glam::Quat::from_mat3(&glam::Mat3::from_cols(bone_dir, bone_rot_y, bone_rot_z));

            self.bone_trans.push(OzzTransform {
                scale,
                rotation: bone_rot,
                position: parent_pos,
            });

            let parent_rot = glam::Quat::from_mat4(parent);
            self.spine_trans.push(OzzTransform {
                scale,
                rotation: parent_rot,
                position: parent_pos,
            });

            if self.skeleton.is_leaf(i as i16) {
                let current_rot = glam::Quat::from_mat4(current);
                self.spine_trans.push(OzzTransform {
                    scale,
                    rotation: current_rot,
                    position: current_pos,
                });
            }
        }
    }
}
