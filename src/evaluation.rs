use crate::board::{Board, EMPTY_BLOCK, ERASING_SUM, FIELD_HEIGHT, FIELD_WIDTH, OBSTACLE_BLOCK};
use crate::pack::Pack;

use crate::search_state::SearchState;
use crate::simulator;
use crate::simulator::{Simulator, DIRECTION_YXS};
use crate::solver_config::DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT;

use crate::zobrist_hash_table::ZobristHash;
use fnv::{FnvHashMap, FnvHashSet};
//max 20

//(10 / 13) ^ 0 (10 / 13) ^ 1 (10 / 13) ^ 2
//a = 4 / 5
//a ^ 0 a ^ 1 a ^  2
pub const GAME_SCORE_DEPTH_RATES: [f64; 20] = [
    1.0,
    0.9090909090909091,
    0.8264462809917354,
    0.7513148009015777,
    0.6830134553650706,
    0.620921323059155,
    0.5644739300537773,
    0.5131581182307067,
    0.4665073802097333,
    0.4240976183724848,
    0.38554328942953164,
    0.35049389948139237,
    0.3186308177103567,
    0.2896643797366879,
    0.2633312543060799,
    0.23939204936916353,
    0.21762913579014864,
    0.19784466890013513,
    0.17985878990921375,
    0.16350799082655795,
];

//chain count: 7
//score: 19
//line obstacle: (19 / 2) = 9 / 10 = 0
#[warn(dead_code)]
const NOT_SPAWN_MAX_CHAIN_COUNT: u8 = 7;


pub struct EvaluateCache {
    cache_estimate_max_chain_count: FnvHashMap<ZobristHash, (u8, u8)>,
    cache_estimate_with_erasing_all_max_chain_count: FnvHashMap<ZobristHash, u8>,
}

impl EvaluateCache {
    pub fn new() -> EvaluateCache {
        EvaluateCache {
            cache_estimate_max_chain_count: FnvHashMap::default(),
            cache_estimate_with_erasing_all_max_chain_count: FnvHashMap::default(),
        }
    }
    pub fn default() -> EvaluateCache {
        EvaluateCache {
            cache_estimate_max_chain_count: FnvHashMap::default(),
            cache_estimate_with_erasing_all_max_chain_count: FnvHashMap::default(),
        }
    }
    /*pub fn empty(&self) -> bool {
        self.len() == 0
    }*/
    pub fn len_estimate_max_chain_count(&self) -> usize {
        self.cache_estimate_max_chain_count.len()
    }
    pub fn len_estimate_with_erasing_all_max_chain_count(&self) -> usize {
        self.cache_estimate_with_erasing_all_max_chain_count.len()
    }
    pub fn clear(&mut self) {
        self.cache_estimate_with_erasing_all_max_chain_count.clear();
        self.cache_estimate_max_chain_count.clear();
    }

    pub fn estimate_with_erasing_all_max_chain_count(&mut self, simulator: &mut Simulator, board: &Board) -> u8 {
        if let Some(chain_count) = self.cache_estimate_with_erasing_all_max_chain_count.get(&board.zobrist_hash()) {
            return chain_count.clone();
        }
        let mut max_chain_count = 0;
        for x in 0..FIELD_WIDTH {
            for y in (0..board.heights[x]).rev() {
                let block = board.get(y, x);
                if block == OBSTACLE_BLOCK || block == EMPTY_BLOCK {
                    continue;
                }
                let erasable: bool = {
                    let mut ok = false;
                    //top
                    if y + 1 < FIELD_HEIGHT {
                        //top
                        if board.get(y + 1, x) == EMPTY_BLOCK {
                            ok = true;
                        }
                        //top right
                        if x + 1 < FIELD_WIDTH && board.get(y + 1, x + 1) == EMPTY_BLOCK {
                            ok = true;
                        }
                        //top left
                        if x >= 1 && board.get(y + 1, x - 1) == EMPTY_BLOCK {
                            ok = true;
                        }
                    }
                    //right
                    if x + 1 < FIELD_WIDTH && board.get(y, x + 1) == EMPTY_BLOCK {
                        ok = true;
                    }
                    //left
                    if x >= 1 && board.get(y, x - 1) == EMPTY_BLOCK {
                        ok = true;
                    }
                    //down
                    if y >= 1 {
                        //down
                        if board.get(y - 1, x) == EMPTY_BLOCK {
                            ok = true;
                        }
                        //down right
                        if x + 1 < FIELD_WIDTH && board.get(y - 1, x + 1) == EMPTY_BLOCK {
                            ok = true;
                        }
                        //down left
                        if x >= 1 && board.get(y - 1, x - 1) == EMPTY_BLOCK {
                            ok = true;
                        }
                    }
                    ok
                };
                if !erasable {
                    break;
                }
                simulator.init();
                let mut simulated_board = board.clone();
                simulator.erase_blocks.push((y, x));
                simulator.apply_erase_blocks(&mut simulated_board);
                let chain_count = simulator.calculate_chain_count(&mut simulated_board);
                max_chain_count = std::cmp::max(max_chain_count, chain_count + 1);
            }
        }
        self.cache_estimate_with_erasing_all_max_chain_count.insert(board.zobrist_hash(), max_chain_count);
        max_chain_count
    }
    //too heavy function
    pub fn estimate_max_chain_count(
        &mut self,
        simulator: &mut Simulator,
        board: &Board,
    ) -> (u8, u8) {
        if let Some(cache_max_chain_count) = self.cache_estimate_max_chain_count.get(&board.zobrist_hash()) {
            return *cache_max_chain_count;
        }
        let mut estimated_max_chain: (u8, u8) = (0, 0);

        for x in 0..FIELD_WIDTH {
            let y = board.heights[x];
            let x = x;
            let mut prune = true;
            if x > 0 {
                let left = board.heights[x - 1];
                if y <= left + 1 {
                    prune = false;
                }
            }
            if x < FIELD_WIDTH - 1 {
                let right = board.heights[x + 1];
                if y <= right + 1 {
                    prune = false;
                }
            }
            if prune {
                continue;
            }
            let y = y as i8;
            let x = x as i8;
            let mut dropped_num_bit: u16 = 0;
            let mut pack = Pack::default();
            for &dyx in DIRECTION_YXS.iter() {
                //top
                if dyx == (1, 0) {
                    continue;
                }
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
                if ((dropped_num_bit >> num) & 1) == 1 {
                    continue;
                }
                dropped_num_bit |= 1 << num;

                let mut point = x as usize;
                pack.clear();
                if x != 9 {
                    pack.set(2, num);
                } else {
                    point -= 1;
                    pack.set(3, num);
                }

                let chain_count = simulator.simulate(&mut board.clone(), point, &pack);
                if chain_count > 0 {
                    if chain_count > estimated_max_chain.0 {
                        estimated_max_chain = (chain_count, 1);
                    } else if chain_count == estimated_max_chain.0 {
                        estimated_max_chain.1 += 1;
                    }
                }
            }
        }
        self.cache_estimate_max_chain_count.insert(board.zobrist_hash(), estimated_max_chain);
        estimated_max_chain
    }


    pub fn evaluate_search_score(
        &mut self,
        simulator: &mut Simulator,
        search_state: &SearchState,
        kind: usize,
    ) -> f64 {
        let mut search_score: f64 = 0.0;

        let mut board = search_state.board();

        if search_state.obstacle_block_count() >= 10 {
            board.drop_obstacles();
        }
        // game score
        // max chain count
        if kind == 0 {
            let (estimated_max_chain_count, _) = self.estimate_max_chain_count(simulator, &board);
            search_score += estimated_max_chain_count as f64 * 1e5;
        } else {
            let estimated_max_erasing_chain_count = self.estimate_with_erasing_all_max_chain_count(simulator, &board);
            search_score += estimated_max_erasing_chain_count as f64 * 1e5;
        }

        // count live block
        let (live_block_count, obstacle_block_count) = board.count_blocks();
        if kind == 0 {
            search_score += (live_block_count as f64 * 1000.0) as f64;
        } else {
            search_score += (live_block_count as f64 * 1100.0) as f64;
        }

        // pattern match
        let (keima, jump, _) = evaluate_pattern_match_cnt(&board);
        // search_score += (three_chain * 10) as f64;
        search_score += (keima * 2 * 10) as f64;
        search_score += (jump * 1 * 10) as f64;

        for x in 0..FIELD_WIDTH {
            //height
            if kind == 0 {
                search_score += 0.01 * board.heights[x] as f64;
            } else {
                search_score += 0.1 * board.heights[x] as f64;
            }

            for y in 0..board.heights[x] {
                let block = board.get(y, x);
                debug_assert_ne!(block, EMPTY_BLOCK);
                /*if block != OBSTACLE_BLOCK {
                    search_score += y as f64 * 0.001;
                } else {
                    search_score -= y as f64 * 0.0001;
                }*/
                if kind == 0 {
                    if x >= 5 {
                        search_score += (9 - x) as f64 * 0.01;
                    } else {
                        search_score += x as f64 * 0.01;
                    }
                } else {
                    if x >= 5 {
                        search_score += (9 - x) as f64 * 0.1;
                    } else {
                        search_score += x as f64 * 0.1;
                    }
                }
                if block == OBSTACLE_BLOCK {
                    continue;
                }
                //down right
                if y >= 1 && x + 1 < FIELD_WIDTH {
                    let target_block = board.get(y - 1, x + 1);
                    if target_block != OBSTACLE_BLOCK && target_block != EMPTY_BLOCK {
                        search_score += 0.1;
                    }
                }
                //right
                if x + 1 < FIELD_WIDTH {
                    let target_block = board.get(y, x + 1);
                    if target_block != OBSTACLE_BLOCK && target_block != EMPTY_BLOCK {
                        search_score += 0.1;
                    }
                }
                //top right
                if y + 1 < FIELD_HEIGHT && x + 1 < FIELD_WIDTH {
                    let target_block = board.get(y + 1, x + 1);
                    if target_block != OBSTACLE_BLOCK && target_block != EMPTY_BLOCK {
                        search_score += 0.1;
                    }
                }
                //top
                if y + 1 < FIELD_HEIGHT {
                    let target_block = board.get(y + 1, x);
                    if target_block != OBSTACLE_BLOCK && target_block != EMPTY_BLOCK {
                        search_score += 0.1;
                    }
                }
            }
        }
        search_score
    }
}

pub fn evaluate_search_result_score_for_bomber(
    chain_count: u8,
    search_score: f64,
    depth: usize,
) -> (f64, f64) {
    (
        evaluate_game_score_for_bomber(chain_count, depth),
        search_score * GAME_SCORE_DEPTH_RATES[depth],
    )
}

pub fn evaluate_search_result_score(
    chain_game_score: u32,
    search_score: f64,
    depth: usize,
) -> (f64, f64) {
    (
        evaluate_game_score_by_depth(chain_game_score, depth),
        search_score * GAME_SCORE_DEPTH_RATES[depth],
    )
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

pub fn evaluate_game_score_for_bomber(chain_count: u8, depth: usize) -> f64 {
    debug_assert!(depth < 20);
    if chain_count == 1 {
        return 1e-10;
    } else if chain_count == 2 {
        return 1e-100;
    }
    //bomber
    let game_score = simulator::calculate_game_score(chain_count);
    let max_score = simulator::calculate_obstacle_count_from_chain_count(3);
    std::cmp::min(max_score, game_score) as f64 * GAME_SCORE_DEPTH_RATES[depth]
}

pub fn evaluate_game_score_by_depth(game_score: u32, depth: usize) -> f64 {
    debug_assert!(depth < 20);

    let max_fatal_gain_score =
        simulator::calculate_obstacle_count_from_chain_count(DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT);
    std::cmp::min(max_fatal_gain_score, game_score) as f64 * GAME_SCORE_DEPTH_RATES[depth]
        + (game_score as f64).log10()
}


pub fn evaluate_pattern_match_cnt(board: &Board) -> (u8, u8, u8) {
    let mut keima = 0;
    let mut jump = 0;
    let mut three_chain = 0;
    for x in 0..FIELD_WIDTH {
        for y in 0..board.heights[x] {
            let block = board.get(y, x);
            if block == EMPTY_BLOCK || block == OBSTACLE_BLOCK {
                continue;
            }
            //short jump
            if y + 2 < FIELD_HEIGHT {
                if block + board.get(y + 2, x) == ERASING_SUM {
                    jump += 1;
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
                    keima += 1;
                    let a = board.get(y + 1, x + 1);
                    //9 0 9
                    //5 2 11
                    //5 1 11
                    //8 11 11
                    //keima x keima
                    if y + 3 < FIELD_HEIGHT {
                        //left keima
                        let b = board.get(y + 3, x);
                        if a + b == ERASING_SUM {
                            three_chain += 1;
                        }
                        //right keima
                        if x + 2 < FIELD_WIDTH {
                            let b = board.get(y + 3, x + 2);
                            if a + b == ERASING_SUM {
                                three_chain += 1;
                            }
                        }
                    }
                }
            }
            if y + 2 < FIELD_HEIGHT && x > 1 {
                if block + board.get(y + 2, x - 1) == ERASING_SUM {
                    keima += 1;
                    let a = board.get(y + 1, x - 1);
                    if y + 3 < FIELD_HEIGHT {
                        //keima x keima
                        //left keima
                        if x >= 2 {
                            let b = board.get(y + 3, x - 2);
                            if a + b == ERASING_SUM {
                                three_chain += 1;
                            }
                        }
                        //right keima
                        let b = board.get(y + 3, x);
                        if a + b == ERASING_SUM {
                            three_chain += 1;
                        }
                    }
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
    (keima, jump, three_chain)
}

#[test]
fn test_simoid() {
    let score = sigmoid(1.0) - sigmoid(0.0);
    debug_assert_eq!(score, 10.0);
}


#[test]
fn test_estimate_with_erasing_all() {
    let board = [
        [0, 0, 0, 0, 0, 6, 0, 0, 0, 0],
        [0, 0, 0, 6, 0, 5, 0, 0, 0, 0],
        [0, 0, 11, 3, 0, 11, 7, 0, 0, 0],
        [0, 0, 11, 11, 0, 6, 1, 0, 0, 0],
        [0, 0, 11, 11, 0, 8, 7, 0, 0, 0],
        [0, 0, 7, 11, 8, 11, 4, 0, 0, 0],
        [0, 0, 11, 6, 5, 11, 9, 3, 0, 0],
        [0, 0, 11, 11, 3, 11, 11, 3, 0, 0],
        [0, 0, 1, 11, 11, 5, 11, 1, 0, 0],
        [0, 0, 8, 3, 11, 11, 8, 11, 0, 0],
        [0, 0, 6, 5, 11, 9, 11, 11, 1, 0],
        [11, 11, 2, 1, 11, 8, 11, 6, 11, 11],
        [11, 11, 4, 3, 11, 3, 3, 6, 11, 11],
        [11, 11, 1, 2, 1, 5, 11, 11, 11, 11],
        [11, 11, 2, 7, 4, 4, 2, 11, 11, 11],
        [11, 11, 9, 9, 5, 3, 3, 11, 11, 11]
    ];
    let mut evaluate_cache = EvaluateCache::new();
    let chain_count = evaluate_cache.estimate_with_erasing_all_max_chain_count(&mut Simulator::new(), &Board::new(board));
    assert_eq!(chain_count, 11);
    let board = [
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 09, 11, 11, 11, 11]
    ];
    let chain_count = evaluate_cache.estimate_with_erasing_all_max_chain_count(&mut Simulator::new(), &Board::new(board));
    assert_eq!(chain_count, 0);
}


#[test]
fn test_evaluate_search_result_score() {
    let p1 = evaluate_search_result_score(simulator::calculate_game_score(10), 100.0, 1);
    let p2 = evaluate_search_result_score(simulator::calculate_game_score(9), 6727592.0, 1);
    assert!(p1 >= p2);
}


#[test]
fn test_evaluate_game_score_by_depth() {
    let score = simulator::calculate_game_score(10);
    debug_assert_eq!(evaluate_game_score_by_depth(score, 0), 51.69897000433602);
    let score = simulator::calculate_game_score(11);
    debug_assert_eq!(evaluate_game_score_by_depth(score, 0), 68.82607480270083);

    let score = simulator::calculate_game_score(12);
    debug_assert_eq!(evaluate_game_score_by_depth(score, 2), 76.33440779869551);
    let score = simulator::calculate_game_score(13);
    debug_assert_eq!(evaluate_game_score_by_depth(score, 2), 88.85604075017984);

    let score = simulator::calculate_game_score(14);
    debug_assert_eq!(evaluate_game_score_by_depth(score, 3), 81.0894512189861);

    let s1 = evaluate_game_score_by_depth(simulator::calculate_game_score(14), 11);
    let s2 = evaluate_game_score_by_depth(simulator::calculate_game_score(12), 7);
    debug_assert!(s1 < s2);
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
    let (keima, jump, _) = evaluate_pattern_match_cnt(&board);
    debug_assert_eq!((keima, jump), (2, 1));


    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 2, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 7, 0, 0, 0],
        [0, 0, 0, 0, 0, 8, 4, 0, 0, 0],
        [0, 0, 0, 0, 0, 9, 3, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 3, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 2, 0, 0, 0],
        [0, 0, 0, 0, 8, 6, 2, 0, 0, 0],
        [0, 0, 0, 0, 8, 9, 6, 0, 0, 0],
        [0, 0, 0, 0, 7, 8, 8, 0, 0, 0],
        [0, 0, 0, 0, 8, 4, 4, 0, 0, 0],
        [0, 0, 0, 6, 1, 3, 2, 4, 0, 0],
    ];
    let (keima, jump, _) = evaluate_pattern_match_cnt(&Board::new(board));
    debug_assert_eq!((keima, jump), (5, 5));
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

    let (max_chain_count, height) =
        evaluate_cache.estimate_max_chain_count(&mut Simulator::new(), &Board::new(board));
    debug_assert_eq!((max_chain_count, height), (1, 1));


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
    let (max_chain_count, height) =
        evaluate_cache.estimate_max_chain_count(&mut Simulator::new(), &Board::new(board));
    debug_assert_eq!((max_chain_count, height), (12, 0));

    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 2, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 7, 0, 0, 0],
        [0, 0, 0, 0, 0, 8, 4, 0, 0, 0],
        [0, 0, 0, 0, 0, 9, 3, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 3, 0, 0, 0],
        [0, 0, 0, 0, 0, 6, 2, 0, 0, 0],
        [0, 0, 0, 0, 8, 6, 2, 0, 0, 0],
        [0, 0, 0, 0, 8, 9, 6, 0, 0, 0],
        [0, 0, 0, 0, 7, 8, 8, 0, 0, 0],
        [0, 0, 0, 0, 8, 4, 4, 0, 0, 0],
        [0, 0, 0, 6, 1, 3, 2, 4, 0, 0],
    ];
    let (max_chain_count, height) =
        evaluate_cache.estimate_max_chain_count(&mut Simulator::new(), &Board::new(board));
    debug_assert_eq!((max_chain_count, height), (11, 1));
}
