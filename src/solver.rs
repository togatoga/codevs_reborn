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
use crate::solver_config::{SolverConfig, DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT};
use crate::search_result::SearchResult;
use self::min_max_heap::MinMaxHeap;
use crate::simulator::{calculate_obstacle_count, CHAIN_CUMULATIVE_SCORES};
use crate::evaluation::evaluate_game_score_by_depth;


pub struct Solver {
    packs: Vec<Vec<(Pack, usize)>>,
    cumulative_sum_pack: [[u8; 10]; MAX_TURN],
    //cumulative_sum_pack[block][turn]
    player: GameStatus,
    enemy: GameStatus,
    config: SolverConfig,
    debug: bool, //debug mode
}

const MAX_TURN: usize = 500;

impl Solver {
    pub fn default() -> Solver {
        Solver { packs: Vec::new(), cumulative_sum_pack: [[0; 10]; MAX_TURN], player: GameStatus::default(), enemy: GameStatus::default(), config: SolverConfig::default(), debug: false }
    }
    pub fn new(packs: Vec<Vec<(Pack, usize)>>, player: GameStatus, enemy: GameStatus, config: SolverConfig, debug: bool) -> Solver {
        Solver { packs, player, enemy, config, cumulative_sum_pack: [[0; 10]; MAX_TURN], debug: debug }
    }
    pub fn set_packs(&mut self, packs: Vec<Vec<(Pack, usize)>>) {
        self.packs = packs;
    }
    pub fn set_game_status(&mut self, player: GameStatus, enemy: GameStatus) {
        self.player = player;
        self.enemy = enemy;
    }
    pub fn set_config(&mut self, config: SolverConfig) {
        self.config = config;
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    #[allow(dead_code)]
    pub fn get_cumulative_sum_pack(&self, turn: usize) -> &[u8; 10] {
        &self.cumulative_sum_pack[turn]
    }
    #[allow(dead_code)]
    pub fn calculate_cumulative_sum_pack(&mut self) {
        for i in 0..MAX_TURN {
            let pack = &self.packs[i][0].0;
            for idx in 0..4 {
                let block = pack.get(idx);
                self.cumulative_sum_pack[i][block as usize] += 1;
            }
            if i >= 1 {
                for block in 1..10 {
                    //println!("{} {} {}", block, self.cumulative_sum_pack[i][block], self.cumulative_sum_pack[i - 1][block]);
                    self.cumulative_sum_pack[i][block] += self.cumulative_sum_pack[i - 1][block];
                }
            }
        }
    }
    pub fn read_packs<R: std::io::Read>(sc: &mut scanner::Scanner<R>) -> Vec<Vec<(Pack, usize)>> {
        (0..MAX_TURN).map(|_| {
            let mut blocks = [0; 4];
            for i in 0..4 {
                blocks[i] = sc.read::<u8>();
            }
            let end: String = sc.read();
            debug_assert_eq!(end, "END");

            let mut pack_set = fnv::FnvHashSet::default();
            let mut res = Vec::new();
            for i in 0..4 {
                let mut pack = Pack::new(&blocks);
                pack.rotates(i);
                //To make pack unique
                //5 8  0 8
                //0 5  5 5
                pack.drop();
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
        debug_assert_eq!(end, "END");
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
    fn should_fire_right_now(&self, chain_count: u8, max_enemy_chain_count: u8) -> bool {
        //chain count is over max
        if chain_count >= self.config.fire_max_chain_count {
            if self.debug {
                eprintln!("Fire!!: A chain count is max chain_count {} {}", chain_count, self.config.fire_max_chain_count);
            }
            return true;
        }


        if self.enemy.obstacle_block_count == 0 && chain_count >= max_enemy_chain_count {
            let enemy_obstacle_count = simulator::calculate_obstacle_count_from_chain_count(max_enemy_chain_count);
            let player_obstacle_count = simulator::calculate_obstacle_count_from_chain_count(chain_count);
            if player_obstacle_count > self.player.obstacle_block_count {
                let player_spawned_obstacle_count = player_obstacle_count - self.player.obstacle_block_count;
                let line_block = player_spawned_obstacle_count / 10;
                if line_block >= 3 {
                    if self.debug {
                        eprintln!("Fire!!: Must spawn three line!! {}", player_spawned_obstacle_count);
                    }
                    return true;
                }
            }
        }

        false
    }
    //gaze enemy
    #[allow(dead_code, )]
    fn gaze_enemy_max_chain_count(&self, current_turn: usize) -> u8 {
        let enemy = &self.enemy;
        let mut max_chain_count = 0;
        for (pack, rotate_count) in self.packs[current_turn].iter() {
            for point in 0..9 {
                let mut board = enemy.board.clone();
                let chain_count = simulator::simulate(&mut board, point, &pack);
                max_chain_count = std::cmp::max(max_chain_count, chain_count);
            }
        }
        max_chain_count
    }
    pub fn think(&mut self, current_turn: usize) -> SearchResult {
        let player = &self.player;
        let enemy = &self.enemy;
        let mut best_search_result = SearchResult::default();
        if self.debug {
            eprintln!("Turn: {}", current_turn);
            eprintln!("Rest Time(msec): {}", player.rest_time_milliseconds);
        }

        let mut root_search_state = SearchState::default();
        root_search_state.set_board(player.board);
        root_search_state.set_obstacle_block_count(player.obstacle_block_count);
        root_search_state.set_spawn_obstacle_block_count(enemy.obstacle_block_count);
        root_search_state.set_cumulative_game_score(player.cumulative_game_score);

        // beam search for a command
        let (beam_depth, beam_width): (usize, usize) = self.config.beam();
        if self.debug {
            eprintln!("Beam depth: {}, Beam width: {}", beam_depth, beam_width);
        }
        let mut search_state_heap: Vec<MinMaxHeap<SearchState>> = (0..beam_depth + 1).map(|_| MinMaxHeap::new()).collect();
        let mut searched_state = fnv::FnvHashSet::default();

        //push an initial search state
        search_state_heap[0].push(root_search_state);
        let mut rnd = Xorshift::with_seed((current_turn + 10) as u64);
        let mut fire_right_now = false;
        let max_enemy_chain_count = self.gaze_enemy_max_chain_count(current_turn);
        for depth in 0..beam_depth {
            //next state
            let search_turn = current_turn + depth;
            if fire_right_now {
                if self.debug {
                    eprintln!("Fire!! now");
                }
                debug_assert_eq!(depth, 1);
                break;
            }
            while let Some(search_state) = &mut search_state_heap[depth].pop_max() {
                //Update obstacle block
                search_state.update_obstacle_block();
                //skip duplicate

                for (pack, rotate_count) in self.packs[search_turn].iter() {
                    for point in 0..9 {
                        let mut board = search_state.board();
                        let chain_count = simulator::simulate(&mut board, point, &pack);
                        //Next board is dead and not to put it in state heap
                        if board.is_game_over() {
                            continue;
                        }

                        //consider whether solver should fire at depth 0
                        if depth == 0 && self.should_fire_right_now(chain_count, max_enemy_chain_count) {
                            fire_right_now = true;
                            //pick best chain count one
                            if chain_count > best_search_result.last_chain_count {
                                best_search_result.last_chain_count = chain_count;
                                best_search_result.command = Command::Drop((point, *rotate_count));
                                best_search_result.gain_game_score = simulator::calculate_game_score(chain_count);
                                best_search_result.cumulative_game_score = search_state.cumulative_game_score() + simulator::calculate_game_score(chain_count);
                                best_search_result.search_depth = depth;
                                best_search_result.board = board.clone();
                            }
                        }

                        //create next search state from a previous state
                        let mut next_search_state = search_state.clone();
                        //update these values
                        let gain_chain_game_score = CHAIN_CUMULATIVE_SCORES[chain_count as usize];
                        let next_board = board;
                        let next_cumulative_game_score = gain_chain_game_score + search_state.cumulative_game_score();
                        let next_spawn_obstacle_block_count = calculate_obstacle_count(gain_chain_game_score, 0) + search_state.spawn_obstacle_block_count();
                        next_search_state.set_board(next_board);
                        next_search_state.set_cumulative_game_score(next_cumulative_game_score);
                        next_search_state.set_spawn_obstacle_block_count(next_spawn_obstacle_block_count);
                        if !next_search_state.is_command() {
                            debug_assert_eq!(depth, 0);
                            next_search_state.set_command(Command::Drop((point, *rotate_count)));
                        }

                        //remove duplication
                        if searched_state.contains(&next_search_state.zobrist_hash()) {
                            continue;
                        }
                        //push it to hash set
                        searched_state.insert(next_search_state.zobrist_hash());
                        debug_assert_eq!(search_state.cumulative_game_score() + gain_chain_game_score, next_search_state.cumulative_game_score());

                        // Add a tiny value(0.0 ~ 1.0) to search score
                        // To randomize search score for the diversity of search
                        let next_search_score = evaluation::evaluate_search_score(&next_search_state) + rnd.randf();
                        next_search_state.set_search_score(next_search_score);

                        //push it to next beam
                        search_state_heap[depth + 1].push(next_search_state);
                        //The number of next beam is over beam_width; pop minimum state
                        while search_state_heap[depth + 1].len() > beam_width {
                            search_state_heap[depth + 1].pop_min();
                        }
                        debug_assert!(search_state_heap[depth + 1].len() <= beam_width);


                        let best_score = evaluate_game_score_by_depth( best_search_result.gain_game_score, best_search_result.search_depth);
                        let target_score = evaluate_game_score_by_depth(gain_chain_game_score, depth);
                        if target_score > best_score {
                            best_search_result.gain_game_score = gain_chain_game_score;
                            best_search_result.cumulative_game_score = next_search_state.cumulative_game_score();
                            best_search_result.last_chain_count = chain_count;
                            best_search_result.search_depth = depth;
                            best_search_result.board = next_search_state.board();
                            best_search_result.command = next_search_state.command().unwrap();
                        }
                    }
                }
            }
        }
        if self.debug {
            eprintln!("== Search Result ==");
            best_search_result.log();
        }
        best_search_result
    }
}

