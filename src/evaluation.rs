use crate::board::{Board, FIELD_WIDTH, EMPTY_BLOCK, OBSTACLE_BLOCK, ERASING_SUM, FIELD_HEIGHT};
use crate::pack::Pack;
use crate::simulator;
use crate::search_state::SearchState;
use crate::simulator::DIRECTION_YXS;
use std::collections::HashSet;


pub fn estimate_max_chain_count(board: &Board) -> (u8, Board) {
    let mut estimated_max_chain_count = 0;
    let mut estimated_board = board.clone();
    //drop single block and evaluate chain count

    for x in 0..FIELD_WIDTH {
        let y =  board.heights[x] as i8;
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
            let (_, chain_count) = simulator::simulate(&mut simulated_board, point, &pack);
            if chain_count > estimated_max_chain_count {
                estimated_max_chain_count = chain_count;
                estimated_board = simulated_board;
            }
        }
    }

    (estimated_max_chain_count, estimated_board)
}


pub fn evaluate_pattern_match_cnt(board: &Board) -> u8 {
    let mut pattern_match_cnt = 0;
    for x in 0..FIELD_WIDTH {
        for y in 0..board.heights[x] {
            let block = board.get(y, x);
            if block == EMPTY_BLOCK || block == OBSTACLE_BLOCK {
                continue
            }
            //short jump
            if y + 2  < FIELD_HEIGHT {
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


pub fn evaluate_search_score(search_state: &SearchState) -> f64 {
    let mut search_score: f64 = 0.0;

    let board = search_state.board;
    // game score
    // max chain count
    let (estimated_max_chain_count, _) = estimate_max_chain_count(&board);
    search_score += estimated_max_chain_count as f64 * 10e5;
    // count live block
    search_score += (board.count_live_blocks() as f64 * 1000.0) as f64;

    search_score += evaluate_pattern_match_cnt(&board) as f64 * 1.0;

    search_score
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
    assert_eq!(cnt, 6);
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
    let (max_chain_count, estimated_board) = estimate_max_chain_count(&Board::new(board));
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
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
    ];
    assert_eq!(max_chain_count, 1);
    assert_eq!(estimated_board, Board::new(board));


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
    let (max_chain_count, _) = estimate_max_chain_count(&Board::new(board));
    assert_eq!(max_chain_count, 12);
}
