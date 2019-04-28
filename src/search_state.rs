use crate::board::{Board, FIELD_WIDTH};
use crate::command::Command;
use std::cmp::Ordering;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SearchState {
    pub board: Board,
    pub obstacle_block_count: u32,
    pub spawn_obstacle_block_count: u32,
    pub skill_point: u32,
    pub cumulative_game_score: u32,
    pub command: Option<Command>,
    pub search_score: f64,
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
    pub fn new(board: &Board) -> SearchState {
        SearchState { board: *board, obstacle_block_count: 0, spawn_obstacle_block_count: 0, skill_point: 0, cumulative_game_score: 0, command: None, search_score: 0.0 }
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
    pub fn with_obstacle_block_count(&mut self, count: u32) -> SearchState {
        self.obstacle_block_count = count;
        *self
    }
    pub fn add_spawn_obstacle_block_count(&mut self, count: u32) -> SearchState {
        self.spawn_obstacle_block_count += count;
        *self
    }
    pub fn with_spawn_obstacle_block_count(&mut self, count: u32) -> SearchState {
        self.spawn_obstacle_block_count = count;
        *self
    }
    pub fn with_board(&mut self, board: Board) -> SearchState {
        self.board = board;
        *self
    }
    pub fn add_cumulative_game_score(&mut self, score: u32) -> SearchState {
        self.cumulative_game_score += score;
        *self
    }
    pub fn with_cumulative_game_score(&mut self, cumulative_game_score: u32) -> SearchState {
        self.cumulative_game_score = cumulative_game_score;
        *self
    }
    pub fn with_search_score(&mut self, search_score: f64) -> SearchState {
        self.search_score = search_score;
        *self
    }
    pub fn with_command(&mut self, command: Command) -> SearchState {
        if self.command.is_none() {
            self.command = Some(command);
        }
        *self
    }
}


#[test]
fn test_compare_search_state() {
    extern crate min_max_heap;

    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 6, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 2, 0, 0],
        [0, 0, 0, 0, 0, 0, 11, 11, 0, 0],
        [0, 0, 0, 0, 0, 11, 7, 7, 0, 0],
        [0, 11, 11, 11, 0, 11, 11, 11, 0, 0],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 6, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 6, 2, 9, 11, 11],
        [11, 11, 6, 6, 11, 6, 2, 6, 11, 11]
    ];

    let lower = SearchState::new(&Board::new(board)).with_search_score(-1000.0);
    let med = SearchState::new(&Board::new(board)).with_search_score(0.0);
    let higher = SearchState::new(&Board::new(board)).with_search_score(100000.0);
    let mut heaps = min_max_heap::MinMaxHeap::new();
    heaps.push(med);
    heaps.push(higher);
    heaps.push(lower);
    assert_eq!(heaps.pop_max(), Some(higher));
    assert_eq!(heaps.pop_max(), Some(med));
    assert_eq!(heaps.pop_max(), Some(lower));
}