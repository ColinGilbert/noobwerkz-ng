pub const MAX_BONES: usize = 64;
pub const INVALID_BONE: usize = usize::MAX;

pub mod skeleton;
pub mod bone;
pub mod keyframe;
pub mod animation;
pub mod animation_graph;
pub mod animgraph_definition;
pub mod blend_additive;
pub mod blend_multiplicative;
pub mod playback;
pub mod operation_result;