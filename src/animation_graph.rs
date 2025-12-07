use std::sync::*;

use mapgraph::{
    aliases::SlotMapGraph,
    map::slotmap::{EdgeIndex, NodeIndex},
};
use ozz_animation_rs::*;
pub struct AnimGraph {
    pub graph: SlotMapGraph<AnimNode, AnimEdge>,
    pub root: Option<NodeIndex>,
    pub on_node: bool,
    pub current_node: Option<NodeIndex>,
    pub current_edge: Option<EdgeIndex>,
    pub get_bool_param: Option<fn(usize) -> bool>,
    pub get_float_param: Option<fn(usize) -> f32>,
    pub get_int_param: Option<fn(usize) -> i64>,
    pub get_uint_param: Option<fn(usize) -> usize>,
    pub get_vec3f_param: Option<fn(usize) -> [f32; 3]>,
}

impl AnimGraph {
    pub fn new() -> Self {
        Self {
            graph: SlotMapGraph::<AnimNode, AnimEdge>::default(),
            root: None,
            on_node: true,
            current_node: None,
            current_edge: None,
            get_bool_param: None,
            get_float_param: None,
            get_int_param: None,
            get_uint_param: None,
            get_vec3f_param: None,
        }
    }
}
pub enum AnimNode {
    Blend(BlendNode),
    LocalToModel(LocalToModelNode),
    Sample(SampleNode),
    StateMachine(StateMachineNode),
}

pub struct BlendNode {
    pub blend_job: BlendingJobArc,
}

impl BlendNode {
    pub fn new(skeleton: Arc<Skeleton>) -> Self {
        let mut blend_job = BlendingJobArc::default();
        blend_job.set_skeleton(skeleton.clone());

        Self { blend_job }
    }

    pub fn update(&mut self) {
        self.blend_job.run().unwrap();
    }

    // Returns the layer index
    pub fn set_input(&mut self, input: Arc<RwLock<Vec<SoaTransform>>>) -> usize {
        self.blend_job
            .layers_mut()
            .push(BlendingLayer::new(input.clone()));
        let i = self.blend_job.layers().len() - 1;
        self.blend_job.layers_mut()[i].weight = 1.0;

        i
    }

    pub fn set_output(&mut self, output: Arc<RwLock<Vec<SoaTransform>>>) {
        self.blend_job.set_output(output.clone());
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

pub struct SampleNode {
    pub sample_job: SamplingJobArc,
    pub seek: f32,
    pub speed: f32,
    pub looping: bool,
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
            looping: false,
            speed: 1.0,
        }
    }

    pub fn update(&mut self, dt: web_time::Duration) {
        let duration = self.sample_job.animation().unwrap().duration();
        self.seek += dt.as_secs_f32() * self.speed;
        if self.looping {
            self.seek %= duration;
        } else {
            if !(self.seek < duration) {
                self.seek = 0.0;
            }
        }
        let ratio = self.seek / duration;
        self.sample_job.set_ratio(ratio);
        self.sample_job.run().unwrap();
    }
}

// This is the most complex node type because it manages the current state, does callbacks, and assigns weights to blending jobs.
pub struct StateMachineNode {
    pub graph: SlotMapGraph<AnimNode, AnimEdge>,
    pub start: Option<NodeIndex>,
    pub end: Option<NodeIndex>,
    pub active_node: Option<NodeIndex>,
    pub active_edge: Option<EdgeIndex>,
    pub on_node: bool,
}

impl StateMachineNode {
    pub fn new() -> Self {
        Self {
            graph: SlotMapGraph::<AnimNode, AnimEdge>::default(),
            start: None,
            end: None,
            active_node: None,
            active_edge: None,
            on_node: true,
        }
    }

    pub fn add_sample_node(
        &mut self,
        skeleton: Arc<Skeleton>,
        animation: Arc<Animation>,
    ) -> NodeIndex {
        let val = AnimNode::Sample(SampleNode::new(skeleton, animation));
        let node_index: NodeIndex = self.graph.add_node(val);

        node_index
    }

    pub fn add_blend_node(&mut self, skeleton: Arc<Skeleton>) -> NodeIndex {
        let val = AnimNode::Blend(BlendNode::new(skeleton));
        let node_index: NodeIndex = self.graph.add_node(val);

        node_index
    }

    pub fn add_l2m_node(
        &mut self,
        skeleton: Arc<Skeleton>,
        locals: Arc<RwLock<Vec<SoaTransform>>>,
    ) -> NodeIndex {
        let val = AnimNode::LocalToModel(LocalToModelNode::new(skeleton, locals));
        let node_index: NodeIndex = self.graph.add_node(val);

        node_index
    }

    pub fn add_state_machine_node(&mut self) -> NodeIndex {
        let val = AnimNode::StateMachine(StateMachineNode::new());
        let node_index: NodeIndex = self.graph.add_node(val);

        node_index
    }
}

pub enum AnimEdge {
    Simple(SimpleEdge),
    Output(OutputEdge),
    Transition(TransitionEdge),
}

// This is used to connect between nodes that don't need any special processing
// IE: From your playback/blend to your l2m job or your state machines to your final output
pub enum SimpleEdge {}

// This is used mostly to pipe between playbacks and blend jobs.
// Can also be used to pass blend job output to another blend job, or even a state machine's output to a blend job
pub struct OutputEdge {
    pub weight: f32,
    pub seek: f32,
    pub speed: f32,
    pub layer: usize,
}

// This edge is used to do transitions between two state machines.
// It is the most complex edge type requiring a its own blend job and many parameters.
// In the future it'll receive information via callbacks and send events.
pub struct TransitionEdge {
    pub blend: BlendNode,
    pub seek1: f32,
    pub seek2: f32,
    pub weight1: f32,
    pub weight2: f32,
    pub speed1: f32,
    pub speed2: f32,
    pub duration: f32,
    pub elapsed: f32,
}

impl TransitionEdge {
    pub fn new(skeleton: Arc<Skeleton>) -> Self {
        Self {
            blend: BlendNode::new(skeleton),
            seek1: 0.0,
            seek2: 0.0,
            weight1: 1.0,
            weight2: 1.0,
            speed1: 1.0,
            speed2: 1.0,
            duration: 0.2,
            elapsed: 0.0,
        }
    }
}
