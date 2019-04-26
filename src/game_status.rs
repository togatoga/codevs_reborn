use crate::field::Field;
#[derive(Debug)]
pub struct GameStatus {
    pub rest_time_milliseconds: u32,
    pub obstacle_block_count: u32,
    pub skill_point: u32,
    pub cumulative_game_score: u32,
    pub field: Field,
}