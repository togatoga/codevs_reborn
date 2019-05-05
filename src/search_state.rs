use crate::board::{Board, FIELD_WIDTH};
use crate::command::Command;
use std::cmp::Ordering;
use crate::zobrist_hash_table::ZobristHash;
use crate::xorshift::Xorshift;
use crate::search_result::SearchResult;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SearchState {
    board: Board,
    obstacle_block_count: u32,
    //obstacle block count what enemy spawned and attacked player
    spawn_obstacle_block_count: u32,
    //spawn obstacle block count what player spawned and attacked enemy
    skill_point: u32,
    cumulative_game_score: u32,
    command: Option<Command>,
    search_score: f64,
}

impl Eq for SearchState {}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &SearchState) -> Option<Ordering> {
        self.search_score.partial_cmp(&other.search_score)
    }
}

impl Ord for SearchState {
    fn cmp(&self, other: &SearchState) -> Ordering {
        self.search_score.partial_cmp(&other.search_score).unwrap()
    }
}


impl SearchState {
    pub fn default() -> SearchState {
        SearchState { board: Board::default(), obstacle_block_count: 0, spawn_obstacle_block_count: 0, skill_point: 0, cumulative_game_score: 0, command: None, search_score: 0.0 }
    }
    pub fn new(board: Board, obstacle_block_count: u32, spawn_obstacle_block_count: u32, skill_point: u32, cumulative_game_score: u32, command: Option<Command>, search_score: f64) -> SearchState {
        SearchState { board, obstacle_block_count, spawn_obstacle_block_count, skill_point, cumulative_game_score, command, search_score }
    }

    pub fn update_obstacle_block_and_drop(&mut self) {
        if self.spawn_obstacle_block_count >= self.obstacle_block_count {
            self.spawn_obstacle_block_count -= self.obstacle_block_count;
            self.obstacle_block_count = 0;
        } else {
            self.obstacle_block_count -= self.spawn_obstacle_block_count;
            self.spawn_obstacle_block_count = 0;
        }
        //Drop
        if self.obstacle_block_count >= FIELD_WIDTH as u32 {
            self.board.drop_obstacles();
            self.obstacle_block_count -= FIELD_WIDTH as u32;
        }
    }
    pub fn update_obstacle_block(&mut self) {
        if self.spawn_obstacle_block_count >= self.obstacle_block_count {
            self.spawn_obstacle_block_count -= self.obstacle_block_count;
            self.obstacle_block_count = 0;
        } else {
            self.obstacle_block_count -= self.spawn_obstacle_block_count;
            self.spawn_obstacle_block_count = 0;
        }
    }
    pub fn obstacle_block_count(&self) -> u32 {
        self.obstacle_block_count
    }
    pub fn with_obstacle_block_count(mut self, count: u32) -> Self
    {
        self.obstacle_block_count = count;
        self
    }
    pub fn spawn_obstacle_block_count(&self) -> u32 {
        self.spawn_obstacle_block_count
    }
    pub fn with_spawn_obstacle_block_count(mut self, count: u32) -> Self {
        self.spawn_obstacle_block_count += count;
        self
    }
    pub fn board(&self) -> Board {
        self.board
    }
    pub fn with_board(mut self, board: Board) -> Self
    {
        self.board = board;
        self
    }
    pub fn cumulative_game_score(&self) -> u32 {
        self.cumulative_game_score
    }
    pub fn with_cumulative_game_score(mut self, cumulative_game_score: u32) -> Self {
        self.cumulative_game_score = cumulative_game_score;
        self
    }
    pub fn search_score(&self) -> f64 {
        self.search_score
    }
    pub fn set_search_score(&mut self, search_score: f64) {
        self.search_score = search_score;
    }
    pub fn with_search_score(mut self, search_score: f64) -> Self {
        self.search_score = search_score;
        self
    }
    pub fn set_command(&mut self, command: Command) {
        self.command = Some(command);
    }
    pub fn with_command(mut self, command: Command) -> Self
    {
        debug_assert!(self.command.is_none());
        self.command = Some(command);
        self
    }
    pub fn is_command(&self) -> bool {
        self.command.is_some()
    }
    pub fn command(&self) -> Option<Command> {
        self.command
    }
    pub fn log(&self) {
        eprintln!("{:?}", *self);
    }
    #[inline]
    pub fn zobrist_hash(&self) -> ZobristHash {
        let mut rnd = Xorshift::with_seed(self.cumulative_game_score as u64);
        self.board.zobrist_hash() ^ rnd.next()
    }
}

#[test]
fn test_zobrist_hash() {
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 7, 0, 0, 0, 0],
        [0, 0, 0, 0, 8, 9, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 6, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 9, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 9, 0, 0, 0, 0],
        [0, 0, 0, 8, 3, 5, 0, 0, 0, 0],
        [0, 0, 0, 5, 8, 1, 0, 0, 0, 0],
        [0, 0, 0, 8, 6, 1, 5, 0, 0, 0],
        [0, 0, 0, 1, 5, 3, 3, 0, 0, 0],
        [0, 0, 0, 8, 1, 4, 8, 0, 0, 0],
        [0, 0, 1, 5, 1, 7, 7, 0, 0, 0],
    ];
    let x1 = SearchState::default()
        .with_cumulative_game_score(120)
        .with_board(Board::new(board))
        .with_cumulative_game_score(100);

    let x2 = SearchState::default()
        .with_cumulative_game_score(200)
        .with_board(Board::new(board));
    assert_ne!(x1, x2);
    let x2 = SearchState::default()
        .with_cumulative_game_score(120)
        .with_board(Board::default());

    assert_ne!(x1, x2);
}
#[test]
fn test_compare_search_state() {
    extern crate min_max_heap;
    let lower = SearchState::default().with_search_score(-1000.0);
    let med = SearchState::default().with_search_score(-100.0);
    let higher = SearchState::default().with_search_score(10000.0);
    let mut heaps = min_max_heap::MinMaxHeap::new();
    heaps.push(med);
    heaps.push(higher);
    heaps.push(lower);
    debug_assert_eq!(heaps.pop_max(), Some(higher));
    debug_assert_eq!(heaps.pop_max(), Some(med));
    debug_assert_eq!(heaps.pop_max(), Some(lower));
}