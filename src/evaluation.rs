use crate::board::{Board, EMPTY_BLOCK, ERASING_SUM, FIELD_HEIGHT, FIELD_WIDTH, OBSTACLE_BLOCK};
use crate::pack::Pack;

use crate::search_state::SearchState;
use crate::simulator;
use crate::simulator::{Simulator, DIRECTION_YXS};
use crate::solver_config::DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT;

use crate::zobrist_hash_table::ZobristHash;
use fnv::FnvHashMap;
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
    cache: FnvHashMap<ZobristHash, (u8, usize)>,
}

impl EvaluateCache {
    pub fn new() -> EvaluateCache {
        EvaluateCache {
            cache: FnvHashMap::default(),
        }
    }
    pub fn default() -> EvaluateCache {
        EvaluateCache {
            cache: FnvHashMap::default(),
        }
    }
    pub fn empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    //too heavy function
    pub fn estimate_max_chain_count(&mut self, simulator: &mut Simulator, board: &Board) -> (u8, usize) {
        if let Some(cache_max_chain_count) = self.cache.get(&board.zobrist_hash()) {
            return *cache_max_chain_count;
        }
        let mut estimated_max_chain: (u8, usize) = (0, 0);

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
                if (dropped_num_bit >> num) == 1 {
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
                if (chain_count, y as usize) > estimated_max_chain {
                    estimated_max_chain = (chain_count, y as usize);
                }
            }
        }
        self.cache
            .insert(board.zobrist_hash(), estimated_max_chain);
        estimated_max_chain
    }


    pub fn evaluate_search_score(
        &mut self,
        simulator: &mut Simulator,
        search_state: &SearchState,
    ) -> f64 {
        let mut search_score: f64 = 0.0;

        let mut board = search_state.board();
        if search_state.obstacle_block_count() >= 10 {
            board.drop_obstacles();
        }
        // game score
        // max chain count
        let (estimated_max_chain_count, height) = self.estimate_max_chain_count(simulator, &board);
        search_score += estimated_max_chain_count as f64 * 10e5;
        search_score += height as f64 * 0.1;
        // count live block
        search_score += (board.count_live_blocks() as f64 * 1000.0) as f64;

        // pattern match
        let (keima, jump) = evaluate_pattern_match_cnt(&board);
        search_score += (keima * 2 * 10) as f64;
        search_score += (jump * 10) as f64;

        for x in 0..FIELD_WIDTH {
            //height
            search_score += 0.01 * board.heights[x] as f64;

            for y in 0..board.heights[x] {
                debug_assert_ne!(board.get(y, x), EMPTY_BLOCK);
                if board.get(y, x) == OBSTACLE_BLOCK {
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

pub fn evaluate_search_result_score_for_bomber(chain_count: u8, search_score: f64, depth: usize) -> f64 {
    1e5 * evaluate_game_score_for_bomber(chain_count, depth)
        + (0.000001  * search_score.log10() * GAME_SCORE_DEPTH_RATES[depth])
}

pub fn evaluate_search_result_score(chain_game_score: u32, search_score: f64, depth: usize) -> f64 {
    1e5 * evaluate_game_score_by_depth(chain_game_score, depth)
        + (0.000001  * search_score.log10() * GAME_SCORE_DEPTH_RATES[depth])
}


pub fn evaluate_game_score_for_bomber(chain_count: u8, depth: usize) -> f64 {
    debug_assert!(depth < 20);
    if chain_count == 1 {
        return -100.0;
    } else if chain_count == 2 {
        return -200.0;
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

pub fn evaluate_pattern_match_cnt(board: &Board) -> (u8, u8) {
    let mut keima = 0;
    let mut jump = 0;
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
                }
            }
            if y + 2 < FIELD_HEIGHT && x > 1 {
                if block + board.get(y + 2, x - 1) == ERASING_SUM {
                    keima += 1;
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
    (keima, jump)
}


#[test]
fn test_evaluate_game_score_by_depth() {
    
    let score = simulator::calculate_game_score(10);
    debug_assert_eq!(
        evaluate_game_score_by_depth(score, 0),
        51.69897000433602
    );
    let score = simulator::calculate_game_score(11);
    debug_assert_eq!(
        evaluate_game_score_by_depth(score, 0),
        68.82607480270083
    );

    let score = simulator::calculate_game_score(12);
    debug_assert_eq!(
        evaluate_game_score_by_depth(score, 2),
        76.33440779869551
    );
    let score = simulator::calculate_game_score(13);
    debug_assert_eq!(
        evaluate_game_score_by_depth(score, 2),
        88.85604075017984);

    let score = simulator::calculate_game_score(14);
    debug_assert_eq!(
        evaluate_game_score_by_depth(score, 3),
        81.0894512189861);
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
    let (keima, jump) = evaluate_pattern_match_cnt(&board);
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
        [0, 0, 0, 6, 1, 3, 2, 4, 0, 0]
    ];
    let (keima, jump) = evaluate_pattern_match_cnt(&Board::new(board));
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
        [0, 0, 0, 6, 1, 3, 2, 4, 0, 0]
    ];
    let (max_chain_count, height) = evaluate_cache.estimate_max_chain_count(&mut Simulator::new(), &Board::new(board));
    debug_assert_eq!((max_chain_count, height), (11, 1));
}
