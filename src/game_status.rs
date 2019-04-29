use crate::board::Board;
#[derive(Debug)]
pub struct GameStatus {
    pub rest_time_milliseconds: u32,
    pub obstacle_block_count: u32,
    pub skill_point: u32,
    pub cumulative_game_score: u32,
    pub board: Board,
}

impl GameStatus {
    pub fn default() -> GameStatus {
        GameStatus {rest_time_milliseconds: 0, obstacle_block_count: 0, skill_point: 0, cumulative_game_score: 0, board: Board::default()}
    }
}