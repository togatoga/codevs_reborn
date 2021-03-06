pub const FIELD_WIDTH: usize = 10;
pub const INPUT_FIELD_HEIGHT: usize = 16;
pub const FIELD_HEIGHT: usize = 19;
pub const DANGER_LINE_HEIGHT: usize = INPUT_FIELD_HEIGHT + 1;
pub const EMPTY_BLOCK: u8 = 0;
pub const OBSTACLE_BLOCK: u8 = 11;
pub const ERASING_SUM: u8 = 10;


use crate::bit_board::BitBoard;
use crate::zobrist_hash_table::ZobristHash;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Board {
    board: BitBoard,
    //y starts from under left point
    pub heights: [usize; FIELD_WIDTH],
}


impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.board.zobrist_hash() == other.board.zobrist_hash() &&
        self.board == other.board
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = vec![];
        for y in 0..FIELD_HEIGHT {
            let mut tmp = Vec::new();
            for x in 0..FIELD_WIDTH {
                tmp.push(format!("{:?}", self.board.get(y, x)));
            }
            debug_assert_eq!(tmp.len(), FIELD_WIDTH);
            board.push(tmp.join(","));
        }
        write!(
            f,
            "Board {{ board: [{}], heights: [{:?}] }}",
            board.join(",\n"),
            self.heights
        )
    }
}


impl Board {
    pub fn default() -> Board {
        Board {
            board: BitBoard::default(),
            heights: [0; FIELD_WIDTH],
        }
    }
    pub fn new(input_board: [[u8; FIELD_WIDTH]; INPUT_FIELD_HEIGHT]) -> Board {
        let mut heights: [usize; FIELD_WIDTH] = [0; FIELD_WIDTH];
        for x in 0..FIELD_WIDTH {
            let mut y = 0;
            while y < INPUT_FIELD_HEIGHT
                && input_board[INPUT_FIELD_HEIGHT - 1 - y][x] != EMPTY_BLOCK
            {
                y += 1;
            }
            heights[x] = y;
        }

        let board = BitBoard::new(input_board);
        Board { board, heights }
    }
    #[inline]
    pub fn get(&self, y: usize, x: usize) -> u8 {
        debug_assert!(y < FIELD_HEIGHT);
        debug_assert!(x < FIELD_WIDTH);
        debug_assert!(self.board.get(y, x) <= 11);
        self.board.get(y, x)
    }
    #[inline]
    pub fn set(&mut self, y: usize, x: usize, value: u8) {
        debug_assert!(y < FIELD_HEIGHT);
        debug_assert!(x < FIELD_WIDTH);
        debug_assert!(value <= 11);
        self.board.set(y, x, value)
    }
    #[inline]
    pub fn zobrist_hash(&self) -> ZobristHash {
        self.board.zobrist_hash()
    }

    pub fn drop_obstacles(&mut self) {
        for x in 0..FIELD_WIDTH {
            debug_assert!(self.heights[x] < FIELD_HEIGHT);
            self.board.set(self.heights[x], x, OBSTACLE_BLOCK);
            self.heights[x] += 1;
        }
    }
    pub fn count_blocks(&self) -> (u8, u8) {
        let mut live_count = 0;
        let mut obstacle_count = 0;
        for x in 0..FIELD_WIDTH {
            for y in 0..self.heights[x] {
                let block = self.get(y, x);
                debug_assert!(block != EMPTY_BLOCK);
                if block != OBSTACLE_BLOCK {
                    live_count += 1;
                } else {
                    obstacle_count += 1;
                }
            }
        }
        (live_count, obstacle_count)
    }
    pub fn is_game_over(&self) -> bool {
        for &x in self.heights.iter() {
            if x >= DANGER_LINE_HEIGHT {
                return true;
            }
        }
        false
    }
}


#[test]
fn test_is_game_over() {
    let board = [
        [0, 0, 9, 11, 11, 4, 8, 8, 0, 0],
        [0, 2, 3, 11, 11, 11, 7, 9, 0, 0],
        [0, 2, 1, 1, 11, 11, 11, 11, 11, 0],
        [0, 3, 11, 6, 6, 11, 11, 11, 1, 0],
        [0, 8, 11, 11, 6, 6, 11, 11, 11, 0],
        [8, 7, 9, 2, 9, 7, 7, 7, 11, 0],
        [9, 5, 11, 4, 11, 9, 2, 9, 5, 11],
        [3, 8, 7, 8, 11, 11, 3, 9, 7, 9],
        [5, 9, 7, 11, 11, 11, 8, 11, 5, 6],
        [11, 11, 11, 11, 4, 3, 11, 9, 3, 11],
        [11, 11, 11, 11, 5, 11, 11, 5, 11, 11],
        [11, 11, 11, 2, 11, 11, 8, 11, 1, 7],
        [11, 11, 11, 4, 7, 4, 7, 11, 8, 11],
        [11, 11, 3, 11, 8, 7, 11, 11, 11, 11],
        [11, 11, 2, 5, 9, 5, 11, 5, 11, 11],
        [11, 11, 2, 3, 3, 2, 7, 1, 11, 11],
    ];
    let mut board = Board::new(board);
    debug_assert!(!board.is_game_over());
    board.drop_obstacles();
    debug_assert!(board.is_game_over());
}

#[test]
fn test_count_live_blocks() {
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 7, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 4, 4, 8, 0, 0, 0, 0],
        [0, 0, 0, 9, 8, 4, 0, 0, 0, 0],
        [0, 0, 0, 3, 4, 8, 9, 0, 0, 0],
        [0, 0, 0, 5, 9, 4, 8, 0, 0, 0],
        [0, 0, 1, 6, 3, 4, 1, 0, 11, 0],
        [0, 0, 6, 5, 1, 2, 3, 4, 11, 0],
        [0, 0, 1, 3, 6, 2, 2, 1, 11, 0],
    ];
    let board = Board::new(board);
    debug_assert_eq!(board.count_blocks(), (35, 3));
}

#[test]
fn test_heights() {
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let board = Board::new(board);
    debug_assert_eq!(board.heights, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn drop_obstacles() {
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let mut board = Board::new(board);
    //drop
    board.drop_obstacles();
    //
    let dropped_obstacles_board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 11],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 1],
        [0, 0, 0, 0, 0, 0, 0, 11, 1, 1],
        [0, 0, 0, 0, 0, 0, 11, 1, 1, 1],
        [0, 0, 0, 0, 0, 11, 1, 1, 1, 1],
        [0, 0, 0, 0, 11, 1, 1, 1, 1, 1],
        [0, 0, 0, 11, 1, 1, 1, 1, 1, 1],
        [0, 0, 11, 1, 1, 1, 1, 1, 1, 1],
        [0, 11, 1, 1, 1, 1, 1, 1, 1, 1],
        [11, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let dropped_obstacles_board = Board::new(dropped_obstacles_board);
    debug_assert_eq!(board, dropped_obstacles_board);
}