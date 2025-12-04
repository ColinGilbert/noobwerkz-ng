use std::sync::*;

use mapgraph::{aliases::SlotMapGraph, map::slotmap::NodeIndex};
use ozz_animation_rs::*;

pub struct AnimationStateMachine {
    pub skeleton: Arc<Skeleton>,
    pub graph: SlotMapGraph<String, AnimNode>,
    pub current: Option<NodeIndex>,
}

impl AnimationStateMachine {
    pub fn new(skeleton: Arc<Skeleton>) -> Self {
        Self {
            skeleton: skeleton.clone(),
            graph: SlotMapGraph::<String, AnimNode>::default(),
            current: None,
        }
    }
}

pub enum AnimNode {
    Sample(SampleNode),
    Blend(BlendNode),
    LocalToModel(LocalToModelNode),
}

pub struct SampleNode {
    pub sample_job: SamplingJobArc,
    pub seek: f32,
}

impl SampleNode {
    pub fn new(skeleton: Arc<Skeleton>, animation: Arc<Animation>) -> Self {
        let mut sample_job = SamplingJobArc::default();

        sample_job.set_animation(animation.clone());

        sample_job.set_context(SamplingContext::new(animation.num_tracks()));

        let sample_out = Arc::new(RwLock::new(vec![
            SoaTransform::default();
            skeleton.num_soa_joints()
        ]));

        sample_job.set_output(sample_out.clone());

        Self {
            sample_job,
            seek: 0.0,
        }
    }

    pub fn update(&mut self, dt: web_time::Duration, speed: f32) {
        let duration = self.sample_job.animation().unwrap().duration();
        self.seek += dt.as_secs_f32() * speed;
        self.seek %= duration;
        let ratio = self.seek / duration;
        self.sample_job.set_ratio(ratio);
        self.sample_job.run().unwrap();
    }
}

pub struct BlendNode {
    pub blend_job: BlendingJobArc,
}

impl BlendNode {
    pub fn new(skeleton: Arc<Skeleton>) -> Self {
        let mut blend_job = BlendingJobArc::default();
        blend_job.set_skeleton(skeleton.clone());
        let blending_out = Arc::new(RwLock::new(vec![
            SoaTransform::default();
            skeleton.num_soa_joints()
        ]));

        blend_job.set_output(blending_out.clone());


        Self { blend_job }
    }

    pub fn update(&mut self) {
        self.blend_job.run().unwrap();
    }

    // Returns the layer index
    pub fn set_input(&mut self, input: Arc<RwLock<Vec<SoaTransform>>>) -> usize {
        self.blend_job.layers_mut().push(BlendingLayer::new(input.clone()));
        let i = self.blend_job.layers().len() - 1;
        self.blend_job.layers_mut()[i].weight = 1.0;
        
        i
    }
}

pub struct LocalToModelNode {
    pub l2m_job: LocalToModelJobArc,
    pub models: Arc<RwLock<Vec<glam::Mat4>>>,
}

impl LocalToModelNode {
    pub fn new(skeleton: Arc<Skeleton>, locals: Arc<RwLock<Vec<SoaTransform>>>) -> Self {
        
        let mut o = Self {
            l2m_job: LocalToModelJob::default(),
            models: Arc::new(RwLock::new(vec![
                glam::Mat4::default();
                skeleton.num_joints()
            ])),
        };

        o.l2m_job.set_skeleton(skeleton.clone());
        o.l2m_job.set_input(locals.clone());
        o.l2m_job.set_output(o.models.clone());

        o
    }
}
