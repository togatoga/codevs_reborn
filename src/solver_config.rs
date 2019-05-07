#[derive(Debug)]
pub struct SolverConfig {
    beam_depth: usize,
    beam_width: usize,
    gaze_beam_depth: usize,
    gaze_beam_width: usize,
    pub fire_max_chain_count: u8,

}
pub const DEFAULT_BEAM_DEPTH: usize = 13;
pub const DEFAULT_BEAM_WIDTH: usize = 600;
pub const DEFAULT_FIRE_MAX_CHAIN_COUNT: u8 = 12;
pub const DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT: u8 = 15;
//parameters for gazing enemy
pub const DEFAULT_GAZE_BEAM_DEPTH: usize = 3;
pub const DEFAULT_GAZE_BEAM_WIDTH: usize = 50;


pub const SOLVER_VERSION: &str = "3.5";
impl SolverConfig {
    pub fn default() -> SolverConfig {
        SolverConfig {
            beam_depth: DEFAULT_BEAM_DEPTH,
            beam_width: DEFAULT_BEAM_WIDTH,
            gaze_beam_depth: DEFAULT_GAZE_BEAM_DEPTH,
            gaze_beam_width: DEFAULT_BEAM_WIDTH,
            fire_max_chain_count: DEFAULT_FIRE_MAX_CHAIN_COUNT}
    }
    pub fn new(beam_depth: usize, beam_width: usize, gaze_beam_depth: usize, gaze_beam_width: usize, fire_max_chain_count: u8) -> SolverConfig {
        SolverConfig {beam_depth, beam_width, gaze_beam_depth, gaze_beam_width, fire_max_chain_count}
    }
    pub fn with_beam(mut self, beam_depth: usize, beam_width: usize) -> SolverConfig {
        self.beam_depth = beam_depth;
        self.beam_width = beam_width;
        self
    }
    pub fn beam(&self) -> (usize, usize) {
        (self.beam_depth, self.beam_width)
    }
    pub fn gaze_beam(&self) -> (usize, usize) {
        (self.gaze_beam_depth, self.gaze_beam_width)
    }
}
