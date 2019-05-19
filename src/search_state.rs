use crate::board::{Board, FIELD_WIDTH};
use crate::command::Command;

use crate::search_result::SearchResult;
use crate::xorshift::Xorshift;
use crate::zobrist_hash_table::{ZobristHash, ZOBRIST_HASH_TABLE_SCORE};
use std::cmp::Ordering;
use crate::pack::Pack;
use std::hash::Hash;
use crate::simulator::calculate_game_score;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SearchState {
    board: Board,
    obstacle_block_count: u32,
    //obstacle block count what enemy spawned and attacked player
    spawn_obstacle_block_count: u32,
    //spawn obstacle block count what player spawned and attacked enemy
    skill_point: u32,
    cumulative_game_score: u32,
    //transition data
    command: Option<Command>,
    chain_count: u8,
    point: usize,
    pack: Pack,
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
        SearchState {
            board: Board::default(),
            obstacle_block_count: 0,
            spawn_obstacle_block_count: 0,
            skill_point: 0,
            cumulative_game_score: 0,
            command: None,
            chain_count: 0,
            point: 0,
            pack: Pack::default(),
            search_score: 0.0,
        }
    }
    pub fn new(
        board: Board,
        obstacle_block_count: u32,
        spawn_obstacle_block_count: u32,
        skill_point: u32,
        cumulative_game_score: u32,
        command: Option<Command>,
        chain_count: u8,
        point: usize,
        pack: Pack,
        search_score: f64,
    ) -> SearchState {
        SearchState {
            board,
            obstacle_block_count,
            spawn_obstacle_block_count,
            skill_point,
            cumulative_game_score,
            command,
            chain_count,
            point,
            pack,
            search_score,
        }
    }

    pub fn update_obstacle_block_and_drop(&mut self) {
        self.update_obstacle_block();
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
    pub fn with_obstacle_block_count(mut self, count: u32) -> Self {
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
    pub fn with_board(mut self, board: Board) -> Self {
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
    pub fn with_command(mut self, command: Command) -> Self {
        debug_assert!(self.command.is_none());
        self.command = Some(command);
        self
    }
    pub fn chain_count(&self) -> u8 {
        self.chain_count
    }
    pub fn set_chain_count(&mut self, chain_count: u8) {
        self.chain_count = chain_count;
    }
    pub fn with_chain_count(mut self, chain_count: u8) -> Self {
        self.chain_count = chain_count;
        self
    }
    pub fn chain_game_score(&self) -> u32 {
        calculate_game_score(self.chain_count)
    }
    pub fn set_point(&mut self, point: usize) {
        self.point = point;
    }
    pub fn with_point(mut self, point: usize) -> Self {
        self.point = point;
        self
    }
    pub fn set_pack(&mut self, pack: Pack) {
        self.pack = pack;
    }
    pub fn with_pack(mut self, pack: Pack) -> Self {
        self.pack = pack;
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
        self.board.zobrist_hash() ^ ZOBRIST_HASH_TABLE_SCORE[std::cmp::min(ZOBRIST_HASH_TABLE_SCORE.len() - 1,  self.cumulative_game_score() as usize)]
    }
    #[inline]
    pub fn transition_zobrist_hash(&self) -> ZobristHash {
        let mut rnd = Xorshift::with_seed(self.point as u64);
        self.zobrist_hash() ^ rnd.next() ^ self.pack.hash()
    }
}

#[test]
fn test_transition_zobrist_hash() {
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 9, 4, 0, 0, 0],
        [0, 0, 0, 0, 0, 2, 3, 0, 0, 0],
        [0, 0, 0, 0, 6, 9, 4, 0, 0, 0],
        [0, 0, 4, 0, 11, 11, 8, 0, 0, 0],
        [0, 0, 8, 0, 11, 11, 7, 0, 11, 0],
        [0, 6, 11, 9, 11, 11, 5, 0, 3, 0],
        [0, 11, 11, 7, 2, 2, 11, 3, 1, 0],
        [11, 11, 11, 11, 2, 6, 11, 11, 11, 11],
        [11, 11, 3, 11, 3, 9, 11, 11, 11, 11],
        [11, 3, 5, 11, 3, 9, 8, 11, 6, 11]
    ];

    let mut pack = Pack::new(&[4, 5, 6, 0]);
    let rotate_count = 3;
    //5 0 4 6
    pack.rotates(3);
    let s = SearchState::default()
        .with_spawn_obstacle_block_count(0)
        .with_obstacle_block_count(0)
        .with_board(Board::new(board))
        .with_point(3)
        .with_pack(pack);

    let mut t = s.clone().with_pack(Pack::new(&[5, 0, 4, 6]));
    debug_assert_eq!(s.transition_zobrist_hash(), t.transition_zobrist_hash());
    t.set_point(1);
    debug_assert_ne!(s.transition_zobrist_hash(),t.zobrist_hash());
}

#[test]
fn test_drop_obstacle_board() {
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

    let s = SearchState::default()
        .with_spawn_obstacle_block_count(0)
        .with_obstacle_block_count(20)
        .with_board(Board::new(board));
    let mut b = s.board();
    b.drop_obstacles();
    assert_ne!(b, s.board());
}

#[test]
fn test_update_obstacle_block_and_drop() {
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

    let mut s = SearchState::default()
        .with_spawn_obstacle_block_count(0)
        .with_obstacle_block_count(20)
        .with_board(Board::new(board));
    s.update_obstacle_block_and_drop();
    assert_eq!(s.spawn_obstacle_block_count, 0);
    assert_eq!(s.obstacle_block_count, 10);
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 11, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 2, 11, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 7, 0, 0, 0, 0],
        [0, 0, 0, 0, 8, 9, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 6, 0, 0, 0, 0],
        [0, 0, 0, 0, 7, 9, 0, 0, 0, 0],
        [0, 0, 0, 11, 4, 9, 0, 0, 0, 0],
        [0, 0, 0, 8, 3, 5, 0, 0, 0, 0],
        [0, 0, 0, 5, 8, 1, 11, 0, 0, 0],
        [0, 0, 0, 8, 6, 1, 5, 0, 0, 0],
        [0, 0, 0, 1, 5, 3, 3, 0, 0, 0],
        [0, 0, 11, 8, 1, 4, 8, 0, 0, 0],
        [11, 11, 1, 5, 1, 7, 7, 11, 11, 11],
    ];
    assert_eq!(s.board, Board::new(board));
}

#[test]
fn test_update_obstacle_block() {
    let mut s = SearchState::default()
        .with_spawn_obstacle_block_count(30)
        .with_obstacle_block_count(20);
    assert_eq!(s.obstacle_block_count(), 20);
    assert_eq!(s.spawn_obstacle_block_count(), 30);
    s.update_obstacle_block();
    assert_eq!(s.obstacle_block_count(), 0);
    assert_eq!(s.spawn_obstacle_block_count(), 10);
    let mut s = SearchState::default()
        .with_spawn_obstacle_block_count(30)
        .with_obstacle_block_count(60);
    assert_eq!(s.obstacle_block_count(), 60);
    assert_eq!(s.spawn_obstacle_block_count(), 30);
    s.update_obstacle_block();
    s.update_obstacle_block();
    assert_eq!(s.obstacle_block_count(), 30);
    assert_eq!(s.spawn_obstacle_block_count(), 0);
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