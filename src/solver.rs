extern crate min_max_heap;
extern crate fnv;

use crate::scanner;
use crate::pack::Pack;
use crate::command::Command;
use crate::search_state::SearchState;
use crate::board::{Board, FIELD_WIDTH, INPUT_FIELD_HEIGHT};
use crate::xorshift::Xorshift;
use crate::evaluation;
use crate::simulator;
use crate::game_status::GameStatus;
use crate::solver_config::{SolverConfig, DEFAULT_BEAM_DEPTH, DEFAULT_BEAM_WIDTH, DEFAULT_FIRE_MAX_CHAIN_COUNT};
use crate::search_result::SearchResult;
use self::min_max_heap::MinMaxHeap;
use crate::zobrist_hash_table::ZobristHash;

#[derive(Debug)]
pub struct Solver<'a> {
    packs: &'a Vec<Vec<(Pack, usize)>>,
    //(pack, rotate_count)
    player: GameStatus,
    enemy: GameStatus,
    config: SolverConfig,
}

const MAX_TURN: usize = 500;

impl<'a> Solver<'a> {
    pub fn new(packs: &'a Vec<Vec<(Pack, usize)>>, player: GameStatus, enemy: GameStatus) -> Solver {
        let config = SolverConfig::new(DEFAULT_BEAM_DEPTH, DEFAULT_BEAM_WIDTH, DEFAULT_FIRE_MAX_CHAIN_COUNT);
        Solver { packs, player, enemy, config }
    }
    pub fn set_config(&mut self, config: SolverConfig) {
        self.config = config;
    }
    pub fn read_packs<R: std::io::Read>(sc: &mut scanner::Scanner<R>) -> Vec<Vec<(Pack, usize)>> {
        (0..MAX_TURN).map(|_| {
            let mut blocks = [0; 4];
            for i in 0..4 {
                blocks[i] = sc.read::<u8>();
            }
            let end: String = sc.read();
            assert_eq!(end, "END");

            let mut pack_set = fnv::FnvHashSet::default();
            let mut res = Vec::new();
            for i in 0..4 {
                let mut pack = Pack::new(&blocks);
                pack.rotates(i);
                if pack_set.contains(&pack) {
                    continue;
                }
                res.push((pack.clone(), i));
                pack_set.insert(pack);
            }
            res
        }).collect()
    }

    pub fn read_game_status<R: std::io::Read>(sc: &mut scanner::Scanner<R>) -> GameStatus {
        //read player data
        let rest_time_milliseconds: u32 = sc.read();
        let obstacle_block_count: u32 = sc.read();
        let skill_point: u32 = sc.read();
        let cumulative_game_score: u32 = sc.read();

        let mut input_board: [[u8; FIELD_WIDTH]; INPUT_FIELD_HEIGHT] = [[0; FIELD_WIDTH]; INPUT_FIELD_HEIGHT];
        for y in 0..INPUT_FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                input_board[y][x] = sc.read::<u8>();
            }
        }
        let end: String = sc.read();
        assert_eq!(end, "END");
        let board = Board::new(input_board);
        GameStatus { rest_time_milliseconds, obstacle_block_count, skill_point, cumulative_game_score, board }
    }
    pub fn output_command(command: Command) {
        match command {
            Command::Drop(v) => {
                println!("{} {}", v.0, v.1);
            }
            Command::Spell => {
                println!("S");
            }
        }
    }
    pub fn think(&mut self, current_turn: usize) -> SearchResult {
        let player = &self.player;
        let mut best_search_result = SearchResult { last_chain_count: 0, turn: current_turn, cumulative_game_score: player.cumulative_game_score, board: player.board.clone(), command: Command::Drop((0, 0)) };
        let enemy = &self.enemy;
        let root_search_state =
            SearchState::new(&player.board)
                .with_obstacle_block_count(player.obstacle_block_count)
                .with_spawn_obstacle_block_count(enemy.obstacle_block_count)
                .with_cumulative_game_score(player.cumulative_game_score);


        let fire_max_chain_count: u8 = self.config.fire_max_chain_count;
        //Fire if chain count is over threshold
        {
            let mut search_state = root_search_state.clone();
            let mut max_chain_count = 0;

            search_state.update_obstacle_block();
            let mut fire = false;
            for (pack, rotate_count) in self.packs[current_turn].iter() {
                for point in 0..9 {
                    let (_, chain_count) = simulator::simulate(&mut search_state.board.clone(), point, &pack);
                    if chain_count >= fire_max_chain_count && chain_count > max_chain_count {
                        max_chain_count = chain_count;
                        best_search_result.command = Command::Drop((point, *rotate_count));
                        fire = true;
                    }
                }
            }

            //Fire!!
            if fire {
                return best_search_result;
            }
        }

        // beam search for a command
        let (beam_depth, beam_width): (usize, usize) = self.config.beam();
        let mut search_state_heap: Vec<MinMaxHeap<SearchState>> = (0..beam_depth + 1).map(|_| MinMaxHeap::new()).collect();
        let mut searched_board: Vec<fnv::FnvHashSet<ZobristHash>> = (0..beam_depth + 1).map(|_| fnv::FnvHashSet::default()).collect();

        //push an initial search state
        search_state_heap[0].push(root_search_state);
        let mut rnd = Xorshift::with_seed(current_turn as u64);

        for depth in 0..beam_depth {
            //next state
            let search_turn = current_turn + depth;

            while let Some(search_state) = &mut search_state_heap[depth].pop_max() {
                //Update obstacle block
                search_state.update_obstacle_block();
                //skip duplicate
                if searched_board[depth].contains(&search_state.zobrist_hash()) {
                    continue;
                }
                //insert
                searched_board[depth].insert(search_state.zobrist_hash());
                for (pack, rotate_count) in self.packs[search_turn].iter() {
                    for point in 0..9 {
                        let mut board = search_state.board.clone();
                        let (score, chain_count) = simulator::simulate(&mut board, point, &pack);
                        //Next board is dead and not to put it in state heap
                        if board.is_game_over() {
                            continue;
                        }
                        let mut next_search_state = search_state.clone()
                            .with_board(board)
                            .with_command(Command::Drop((point, *rotate_count)))
                            .add_cumulative_game_score(score)
                            .add_spawn_obstacle_block_count(simulator::calculate_obstacle_count(chain_count, 0));
                        assert_eq!(search_state.cumulative_game_score + score, next_search_state.cumulative_game_score);
                        // Add a tiny value(0.0 ~ 1.0) to search score
                        // To randomize search score for the diversity of search
                        next_search_state.with_search_score(evaluation::evaluate_search_score(&next_search_state) + rnd.randf());
                        //push it to next beam
                        search_state_heap[depth + 1].push(next_search_state);
                        //The number of next beam is over beam_width; pop minimum state
                        while search_state_heap[depth + 1].len() > beam_width {
                            search_state_heap[depth + 1].pop_min();
                        }
                        assert!(search_state_heap[depth + 1].len() <= beam_width);

                        if next_search_state.cumulative_game_score > best_search_result.cumulative_game_score {
                            best_search_result.cumulative_game_score = next_search_state.cumulative_game_score;
                            best_search_result.last_chain_count = chain_count;
                            best_search_result.turn = search_turn;
                            best_search_result.board = next_search_state.board;
                            best_search_result.command = next_search_state.command.unwrap();
                        }
                        assert!(next_search_state.cumulative_game_score <= best_search_result.cumulative_game_score);
                    }
                }
            }
        }
        best_search_result
    }
}

#[test]
fn test_read_packs() {}

