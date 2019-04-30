use crate::board::{Board, FIELD_WIDTH};
use crate::command::Command;
use std::cmp::Ordering;
use crate::zobrist_hash_table::ZobristHash;
use crate::xorshift::Xorshift;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SearchState {
    board: Board,
    obstacle_block_count: u32, //obstacle block count what enemy spawned and attacked player
    spawn_obstacle_block_count: u32, //spawn obstacle block count what player spawned and attacked enemy
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

    pub fn update_obstacle_block(&mut self) {
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
    pub fn obstacle_block_count(&self) -> u32 {
        self.obstacle_block_count
    }
    pub fn set_obstacle_block_count(&mut self, count: u32) {
        self.obstacle_block_count = count;
    }
    pub fn spawn_obstacle_block_count(&self) -> u32 {
        self.spawn_obstacle_block_count
    }
    pub fn set_spawn_obstacle_block_count(&mut self, count: u32) {
        self.spawn_obstacle_block_count += count;
    }
    pub fn board(&self) -> Board {
        self.board
    }
    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }
    pub fn cumulative_game_score(&self) -> u32 {
        self.cumulative_game_score
    }
    pub fn set_cumulative_game_score(&mut self, cumulative_game_score: u32) {
        self.cumulative_game_score = cumulative_game_score;
    }
    pub fn search_score(&self) -> f64 {
        self.search_score
    }
    pub fn set_search_score(&mut self, search_score: f64) {
        self.search_score = search_score;
    }
    pub fn set_command(&mut self, command: Command) {
        debug_assert!(self.command.is_none());
        self.command = Some(command);
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
fn test_compare_search_state() {
    extern crate min_max_heap;
    let mut lower = SearchState::default();
    lower.set_search_score(-1000.0);
    let mut med = SearchState::default();
    med.set_search_score(-1000.0);
    let mut higher = SearchState::default();
    higher.set_search_score(1000.0);
    let mut heaps = min_max_heap::MinMaxHeap::new();
    heaps.push(med);
    heaps.push(higher);
    heaps.push(lower);
    debug_assert_eq!(heaps.pop_max(), Some(higher));
    debug_assert_eq!(heaps.pop_max(), Some(med));
    debug_assert_eq!(heaps.pop_max(), Some(lower));
}