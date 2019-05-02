use crate::board::{Board, FIELD_WIDTH, EMPTY_BLOCK, OBSTACLE_BLOCK, ERASING_SUM, FIELD_HEIGHT};
use crate::pack::Pack;
use crate::simulator;
use crate::search_state::SearchState;
use crate::simulator::DIRECTION_YXS;
use crate::solver_config;
use std::collections::HashSet;
use crate::solver_config::DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT;
use fnv::FnvHashMap;
use crate::zobrist_hash_table::ZobristHash;

//max 20

//(10 / 13) ^ 0 (10 / 13) ^ 1 (10 / 13) ^ 2
//a = 4 / 5
//a ^ 0 a ^ 1 a ^  2
const GAME_SCORE_DEPTH_RATES: [f64; 20] = [1.0, 0.9090909090909091, 0.8264462809917354, 0.7513148009015777, 0.6830134553650706, 0.620921323059155, 0.5644739300537773, 0.5131581182307067, 0.4665073802097333, 0.4240976183724848, 0.38554328942953164, 0.35049389948139237, 0.3186308177103567, 0.2896643797366879, 0.2633312543060799, 0.23939204936916353, 0.21762913579014864, 0.19784466890013513, 0.17985878990921375, 0.16350799082655795];

//chain count: 7
//score: 19
//line obstacle: (19 / 2) = 9 / 10 = 0
const NOT_SPAWN_MAX_CHAIN_COUNT: u8 = 7;


pub struct EvaluateCache {
    cache: FnvHashMap<ZobristHash, u8>,
}

impl EvaluateCache {
    pub fn new() -> EvaluateCache {
        EvaluateCache { cache: FnvHashMap::default() }
    }
    //too heavy function
    pub fn estimate_max_chain_count(&mut self, board: &Board) -> u8 {
        if let Some(cache_max_chain_count) = self.cache.get(&board.zobrist_hash()) {
            return cache_max_chain_count.clone();
        }
        let mut estimated_max_chain_count = 0;
        let mut estimated_board = board.clone();
        //drop single block and evaluate chain count

        for x in 0..FIELD_WIDTH {
            let y = board.heights[x] as i8;
            let x = x as i8;
            let mut dropped_num = HashSet::new();
            for &dyx in DIRECTION_YXS.iter() {
                if !simulator::is_on_board(y + dyx.0, x + dyx.1) {
                    continue;
                }
                let ny: usize = (y + dyx.0) as usize;
                let nx: usize = (x + dyx.1) as usize;
                let neighbor_block = board.get(ny, nx);
                if neighbor_block == EMPTY_BLOCK || neighbor_block == OBSTACLE_BLOCK {
                    continue;
                }
                let num = ERASING_SUM - neighbor_block;
                //skip
                if dropped_num.contains(&num) {
                    continue;
                }
                dropped_num.insert(num);
                let mut pack = Pack::default();
                let mut point = x as usize;
                if x != 9 {
                    pack.set(2, num);
                } else {
                    point -= 1;
                    pack.set(3, num);
                }
                let mut simulated_board = board.clone();
                let chain_count = simulator::simulate(&mut simulated_board, point, &pack);
                if chain_count > estimated_max_chain_count {
                    estimated_max_chain_count = chain_count;
                    estimated_board = simulated_board;
                }
            }
        }
        self.cache.insert(board.zobrist_hash(), estimated_max_chain_count);
        estimated_max_chain_count
    }


    pub fn evaluate_search_score(&mut self, search_state: &SearchState) -> f64 {
        let mut search_score: f64 = 0.0;

        let board = search_state.board();
        // game score
        // max chain count

        let estimated_max_chain_count = self.estimate_max_chain_count(&board);
        search_score += estimated_max_chain_count as f64 * 10e5;
        // count live block
        search_score += (board.count_live_blocks() as f64 * 1000.0) as f64;
        // pattern match
        search_score += evaluate_pattern_match_cnt(&board) as f64 * 1.0;

        search_score
    }
}


pub fn evaluate_game_score_by_depth(game_score: u32, depth: usize) -> f64 {
    debug_assert!(depth < 20);
    let max_fatal_gain_score = simulator::calculate_obstacle_count_from_chain_count(DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT);
    //small chain count
    /*if game_score <= simulator::calculate_game_score(NOT_SPAWN_MAX_CHAIN_COUNT) {
        return 1.0 * GAME_SCORE_DEPTH_RATES[depth] * GAME_SCORE_DEPTH_RATES[depth];
    }*/
    std::cmp::min(max_fatal_gain_score, game_score) as f64 * GAME_SCORE_DEPTH_RATES[depth]
}

pub fn evaluate_pattern_match_cnt(board: &Board) -> u8 {
    let mut pattern_match_cnt = 0;
    for x in 0..FIELD_WIDTH {
        for y in 0..board.heights[x] {
            let block = board.get(y, x);
            if block == EMPTY_BLOCK || block == OBSTACLE_BLOCK {
                continue;
            }
            //short jump
            if y + 2 < FIELD_HEIGHT {
                if block + board.get(y + 2, x) == ERASING_SUM {
                    pattern_match_cnt += 1;
                }
            }
            //big jump
            /*if y + 3 < FIELD_HEIGHT {
                if block + board.get(y + 3, x) == ERASING_SUM {
                    pattern_match_cnt += 1
                }
            }*/
            //short keima
            if y + 2 < FIELD_HEIGHT && x + 1 < FIELD_WIDTH {
                if block + board.get(y + 2, x + 1) == ERASING_SUM {
                    pattern_match_cnt += 1;
                }
            }
            if y + 2 < FIELD_HEIGHT && x > 1 {
                if block + board.get(y + 2, x - 1) == ERASING_SUM {
                    pattern_match_cnt += 1;
                }
            }
            /*//big keima
            if y + 3 < FIELD_HEIGHT && x + 1 < FIELD_WIDTH {
                if block + board.get(y + 3, x + 1) == ERASING_SUM {
                    pattern_match_cnt += 1;
                }
            }
            if y + 3 < FIELD_HEIGHT && x > 1 {
                if block + board.get(y + 3, x - 1) == ERASING_SUM {
                    pattern_match_cnt += 1;
                }
            }*/
        }
    }
    pattern_match_cnt
}





#[test]
fn test_evaluate_game_score_by_depth() {
    let depth = 2;
    let score = 120;
    debug_assert_eq!(evaluate_game_score_by_depth(score, depth), 76.80000000000001);
}

#[test]
fn test_evaluate_pattern_match() {
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 9, 9, 9, 0, 0, 0, 0, 0, 0],
        [0, 9, 9, 9, 0, 0, 0, 0, 0, 0],
        [0, 11, 2, 2, 0, 0, 0, 0, 0, 0],
        [0, 11, 1, 11, 0, 0, 0, 0, 0, 0],
    ];
    let board = Board::new(board);
    let cnt = evaluate_pattern_match_cnt(&board);
    debug_assert_eq!(cnt, 3);
}

#[test]
fn test_estimate_max_chain_count() {
    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 9],
    ];
    let mut evaluate_cache = EvaluateCache::new();
    let max_chain_count = evaluate_cache.estimate_max_chain_count(&Board::new(board));
    debug_assert_eq!(max_chain_count, 1);


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
        [0, 0, 1, 5, 1, 7, 7, 0, 0, 0]
    ];
    let max_chain_count = evaluate_cache.estimate_max_chain_count(&Board::new(board));
    debug_assert_eq!(max_chain_count, 12);
}
