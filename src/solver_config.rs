#[derive(Debug)]
pub struct SolverConfig {
    beam_depth: usize,
    beam_width: usize,
    pub fire_max_chain_count: u8,
}
pub const DEFAULT_BEAM_DEPTH: usize = 15;
pub const DEFAULT_BEAM_WIDTH: usize = 150;
pub const DEFAULT_FIRE_MAX_CHAIN_COUNT: u8 = 11;
pub const SOLVER_VERSION: &str = "2.2";
impl SolverConfig {
    pub fn new(beam_depth: usize, beam_width: usize, fire_max_chain_count: u8) -> SolverConfig {
        SolverConfig {beam_depth, beam_width, fire_max_chain_count}
    }
    pub fn beam(&self) -> (usize, usize) {
        (self.beam_depth, self.beam_width)
    }
}
