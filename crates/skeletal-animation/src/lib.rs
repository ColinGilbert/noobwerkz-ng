pub const MAX_BONES: usize = 64;

pub mod skeleton;
pub mod bone;
pub mod animation_channel;
pub mod animation;
pub mod animation_graph;
pub mod animation_graph_definition;
pub mod blend_additive;
pub mod blend_multiplicative;
pub mod operation_result;
pub mod seek_results;
pub mod local_to_model;