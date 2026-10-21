pub const MAX_BONES: usize = 64;

pub mod skeleton;
pub mod bone;
pub mod keyframe;
pub mod animation_channel;
pub mod animation_graph;
pub mod animation_graph_definition;
pub mod blend_additive;
pub mod blend_multiplicative;
pub mod playback;
pub mod operation_result;