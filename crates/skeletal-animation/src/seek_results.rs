#[derive(Clone)]
pub struct SeekResults {
    pub end: bool,
    pub begin: bool,
    pub interpolation_needed: bool,
    pub t_between_frames: f32,
    pub index: usize,
}