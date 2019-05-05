use crate::board::Board;

#[derive(Debug, Clone)]
pub struct GameStatus {
    rest_time_milliseconds: u32,
    obstacle_block_count: u32,
    skill_point: u32,
    cumulative_game_score: u32,
    board: Board,
}

impl GameStatus {
    pub fn default() -> GameStatus {
        GameStatus { rest_time_milliseconds: 0, obstacle_block_count: 0, skill_point: 0, cumulative_game_score: 0, board: Board::default() }
    }
    pub fn rest_time_milliseconds(&self) -> u32 {
        self.rest_time_milliseconds
    }
    pub fn with_rest_time_milliseconds(mut self, rest_time_milliseconds: u32) -> GameStatus {
        self.rest_time_milliseconds = rest_time_milliseconds;
        self
    }
    pub fn obstacle_block_count(&self) -> u32 {
        self.obstacle_block_count
    }
    pub fn with_obstacle_block_count(mut self, obstacle_block_count: u32) -> GameStatus {
        self.obstacle_block_count = obstacle_block_count;
        self
    }
    pub fn skill_point(&self) -> u32 {
        self.skill_point
    }
    pub fn with_skill_point(mut self, skill_point: u32) -> GameStatus {
        self.skill_point = skill_point;
        self
    }
    pub fn cumulative_game_score(&self) -> u32 {
        self.cumulative_game_score
    }
    pub fn with_cumulative_game_score(mut self, cumulative_game_score: u32) -> GameStatus {
        self.cumulative_game_score = cumulative_game_score;
        self
    }
    pub fn board(&self) -> Board {
        self.board
    }
    pub fn with_board(mut self, board: Board) -> GameStatus {
        self.board = board;
        self
    }
}



#[test]
fn test_game_status() {
    let board = [
        [0, 0, 7, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 8, 0, 0, 0, 0, 0, 0, 0],
        [0, 6, 3, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 1, 2, 0, 11, 0, 0, 0, 0],
        [0, 3, 5, 3, 2, 1, 0, 0, 0, 0],
        [0, 6, 1, 11, 3, 11, 0, 0, 0, 0],
        [7, 5, 7, 11, 11, 8, 0, 0, 0, 0],
        [4, 7, 2, 11, 11, 7, 0, 0, 0, 0],
        [11, 11, 11, 7, 2, 11, 0, 0, 0, 0],
        [11, 11, 11, 2, 11, 11, 11, 0, 3, 8],
        [11, 11, 11, 2, 3, 3, 1, 0, 8, 3],
        [3, 3, 3, 2, 11, 11, 6, 11, 11, 11],
        [3, 9, 11, 3, 3, 9, 11, 11, 11, 11],
        [4, 11, 11, 11, 11, 9, 11, 11, 11, 11],
        [11, 11, 9, 11, 5, 9, 11, 11, 11, 11],
        [11, 6, 8, 7, 9, 2, 11, 11, 11, 11]
    ];
    let game_status = GameStatus::default()
        .with_rest_time_milliseconds(1126757)
        .with_obstacle_block_count(46)
        .with_skill_point(0)
        .with_cumulative_game_score(187)
        .with_board(Board::new(board));

    assert_eq!(game_status.rest_time_milliseconds(), 1126757);
    assert_eq!(game_status.obstacle_block_count(), 46);
    assert_eq!(game_status.skill_point(), 0);
    assert_eq!(game_status.cumulative_game_score(), 187);
    assert_eq!(game_status.board(), Board::new(board));
}