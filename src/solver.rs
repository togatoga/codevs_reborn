extern crate fnv;
extern crate min_max_heap;

use self::min_max_heap::MinMaxHeap;
use crate::board::{
    Board, DANGER_LINE_HEIGHT, FIELD_HEIGHT, FIELD_WIDTH, INPUT_FIELD_HEIGHT, OBSTACLE_BLOCK,
};
use crate::command::Command;
use crate::evaluation::{
    evaluate_game_score_by_depth, evaluate_game_score_for_bomber, evaluate_search_result_score,
    evaluate_search_result_score_for_bomber, EvaluateCache, GAME_SCORE_DEPTH_RATES,
};
use crate::game_status::GameStatus;
use crate::pack::Pack;
use crate::scanner;
use crate::search_result::{SearchResult, FIRE_RIGHT_NOW_BOOST_SCORE};
use crate::search_state::SearchState;
use crate::simulator;
use crate::simulator::Simulator;
use crate::solver_config::{SolverConfig, DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT};
use crate::xorshift::Xorshift;


pub struct Solver {
    packs: Vec<Vec<(Pack, usize)>>,
    player: GameStatus,
    enemy: GameStatus,
    config: SolverConfig,
    simulator: Simulator,
    evaluate_cache: EvaluateCache,
    turn: usize,
    last_best_search_result: Option<(u8, usize)>,
    seed: u64,
    debug: bool, //debug mode
}

const MAX_TURN: usize = 500;

impl Solver {
    pub fn default() -> Solver {
        Solver {
            packs: Vec::new(),
            player: GameStatus::default(),
            enemy: GameStatus::default(),
            config: SolverConfig::default(),
            simulator: Simulator::default(),
            evaluate_cache: EvaluateCache::new(),
            turn: 0,
            last_best_search_result: None,
            seed: 1024,
            debug: false,
        }
    }
    pub fn new(
        packs: Vec<Vec<(Pack, usize)>>,
        player: GameStatus,
        enemy: GameStatus,
        config: SolverConfig,
        seed: u64,
        debug: bool,
    ) -> Solver {
        Solver {
            packs,
            player,
            enemy,
            config,
            simulator: Simulator::new(),
            evaluate_cache: EvaluateCache::new(),
            turn: 0,
            last_best_search_result: None,
            seed,
            debug,
        }
    }
    pub fn turn(&self) -> usize {
        self.turn
    }
    pub fn with_turn(mut self, turn: usize) -> Self {
        self.turn = turn;
        self
    }
    pub fn set_turn(&mut self, turn: usize) {
        self.turn = turn;
    }
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }
    pub fn set_packs(&mut self, packs: Vec<Vec<(Pack, usize)>>) {
        self.packs = packs;
    }
    pub fn set_game_status(&mut self, player: GameStatus, enemy: GameStatus) {
        //check whether solver should clear cache
        self.clear_cache_if_needed(&player);
        self.player = player;
        self.enemy = enemy;
    }
    pub fn set_config(&mut self, config: SolverConfig) {
        self.config = config;
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
    fn clear_cache_if_needed(&mut self, player: &GameStatus) {
        if self.debug {
            eprintln!(
                "Cache size for erasing all {}",
                self.evaluate_cache
                    .len_estimate_with_erasing_all_max_chain_count()
            );
            eprintln!(
                "Cache size for max chain count {}",
                self.evaluate_cache.len_estimate_max_chain_count()
            );
        }

        let previous_obstacle_count = self.player.obstacle_block_count();
        if previous_obstacle_count == 0 {
            //new player obstacle_block_count
            let line_block = simulator::calculate_spawn_obstacle_line_from_obstacle_count(
                player.obstacle_block_count(),
            );
            //enemy spawn new obstacle block
            if line_block > 0 {
                if self.debug {
                    eprintln!("Cache Clear because board get dirty!!");
                }
                self.evaluate_cache.clear();
                self.last_best_search_result = None;
                //debug_assert!(self.evaluate_cache.empty());
            }
        }
        //512MB
        if self.evaluate_cache.len_estimate_max_chain_count() > 512 * 1024 * 1024 {
            eprintln!(
                "Cache Clear because cache is too big: {}",
                self.evaluate_cache.len_estimate_max_chain_count()
            );
            self.evaluate_cache.clear();
        }
        if self
            .evaluate_cache
            .len_estimate_with_erasing_all_max_chain_count()
            > 512 * 1024 * 1024
        {
            eprintln!(
                "Cache Clear because cache is too big: {}",
                self.evaluate_cache.len_estimate_max_chain_count()
            );
            self.evaluate_cache.clear();
        }
    }

    pub fn read_packs<R: std::io::Read>(sc: &mut scanner::Scanner<R>) -> Vec<Vec<(Pack, usize)>> {
        (0..MAX_TURN)
            .map(|_| {
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
            })
            .collect()
    }

    pub fn read_game_status<R: std::io::Read>(sc: &mut scanner::Scanner<R>) -> GameStatus {
        //read player data
        let rest_time_milliseconds: u32 = sc.read();
        let obstacle_block_count: u32 = sc.read();
        let skill_point: u32 = sc.read();
        let cumulative_game_score: u32 = sc.read();

        let mut input_board: [[u8; FIELD_WIDTH]; INPUT_FIELD_HEIGHT] =
            [[0; FIELD_WIDTH]; INPUT_FIELD_HEIGHT];
        for y in 0..INPUT_FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                input_board[y][x] = sc.read::<u8>();
            }
        }
        let end: String = sc.read();
        debug_assert_eq!(end, "END");
        let board = Board::new(input_board);
        GameStatus::default()
            .with_rest_time_milliseconds(rest_time_milliseconds)
            .with_obstacle_block_count(obstacle_block_count)
            .with_skill_point(skill_point)
            .with_cumulative_game_score(cumulative_game_score)
            .with_board(board)
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

    fn should_fire_right_now(
        &self,
        chain_count: u8,
        max_enemy_chain_count: u8,
        need_kill_chain_count: u8,
    ) -> bool {
        if chain_count > max_enemy_chain_count {
            return false;
        }
        //chain count is fatal max chain count
        if chain_count >= need_kill_chain_count {
            if self.debug {
                eprintln!(
                    "Fire!!: A chain count is over to kill enemy!!: {} {}",
                    chain_count, need_kill_chain_count
                );
            }
            return true;
        }


        if self.enemy.obstacle_block_count() < 10 && chain_count >= max_enemy_chain_count {
            let enemy_obstacle_count =
                simulator::calculate_obstacle_count_from_chain_count(max_enemy_chain_count);
            let total_enemy_obstacle_count =
                enemy_obstacle_count + self.enemy.obstacle_block_count();
            let player_obstacle_count =
                simulator::calculate_obstacle_count_from_chain_count(chain_count);
            if player_obstacle_count > self.player.obstacle_block_count() {
                let player_spawned_obstacle_count =
                    player_obstacle_count - self.player.obstacle_block_count();
                if player_spawned_obstacle_count <= total_enemy_obstacle_count {
                    return false;
                }
                let spawned_obstacle_count =
                    player_spawned_obstacle_count - total_enemy_obstacle_count;
                let line_block = simulator::calculate_spawn_obstacle_line_from_obstacle_count(
                    spawned_obstacle_count,
                );
                if line_block >= 3 {
                    if self.debug {
                        eprintln!(
                            "Fire!!: Must spawn three line!! {}",
                            player_spawned_obstacle_count
                        );
                    }
                    return true;
                }
            }
        }

        false
    }
    fn gaze_enemy_need_kill_chain_count(&self) -> u8 {
        let enemy = &self.enemy;
        let board = enemy.board();
        let mut spawned_obstacle_count = 0;
        for x in 0..FIELD_WIDTH {
            for y in (0..board.heights[x]).rev() {
                if board.get(y, x) == OBSTACLE_BLOCK {
                    spawned_obstacle_count = std::cmp::max(spawned_obstacle_count, y + 1);
                    break;
                }
            }
        }
        let need_kill_line = (DANGER_LINE_HEIGHT - spawned_obstacle_count) as u8;
        for chain_count in 0..DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT {
            let spawned_obstacle_count =
                simulator::calculate_obstacle_count_from_chain_count(chain_count);
            let spawned_line = (spawned_obstacle_count / 10) as u8;
            if spawned_line >= need_kill_line {
                return chain_count;
            }
        }
        DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT
    }
    #[allow(dead_code)]
    fn gaze_enemy_max_chain_count_by_beam_search(&mut self, beam_depth: usize, beam_width: usize) {}
    //gaze enemy
    #[allow(dead_code)]
    fn gaze_enemy_max_chain_count(&mut self) -> u8 {
        let current_turn = self.turn();
        let mut max_chain_count = 0;
        for (pack, _) in self.packs[current_turn].iter() {
            for point in 0..9 {
                let mut board = self.enemy.board().clone();
                let chain_count = self.simulator.simulate(&mut board, point, &pack);
                max_chain_count = std::cmp::max(max_chain_count, chain_count);
            }
        }
        max_chain_count
    }
    pub fn beam_search_config(&self) -> (usize, usize) {
        let player = &self.player;
        if player.rest_time_milliseconds() >= 30000 {
            //more than 30 seconds
            if let Some(last_search_result) = self.last_best_search_result {
                let (last_chain_count, last_search_depth) = last_search_result;
                let (max_beam_depth, max_beam_width) = self.config.beam();
                //use normal beam
                if last_search_depth == 0 {
                    return self.config.beam();
                }
                //Too small chain count
                if last_chain_count <= 10 {
                    return self.config.beam();
                }
                return (
                    std::cmp::min(last_search_depth + 2, max_beam_depth),
                    max_beam_width,
                );
            }
            return self.config.beam();
        }
        if player.rest_time_milliseconds() >= 10000 {
            //more thatn 10 seconds
            return (5, 100);
        }
        (3, 100)
    }

    fn kill_bomber_mode(&self) -> bool {
        if self.enemy.skill_point() >= 48 {
            if self.player.cumulative_game_score() >= self.enemy.cumulative_game_score() {
                let diff = self.player.cumulative_game_score() - self.enemy.cumulative_game_score();
                if diff >= 50 {
                    return true;
                }
            }
        }
        false
    }
    fn should_spell_magic(&self) -> bool {
        if !self.kill_bomber_mode() && self.player.skill_point() >= 80 {
            for y in 0..FIELD_HEIGHT {
                for x in 0..FIELD_WIDTH {
                    //at least one 5
                    if self.player.board().get(y, x) == 5 {
                        if self.debug {
                            eprintln!("Sepll Magic!!");
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn think(&mut self) -> SearchResult {

        let current_turn = self.turn();
        let mut best_search_result = SearchResult::default();
        if self.kill_bomber_mode() {
            if self.debug {
                eprintln!("Kill Bomber!!");
            }
        }
        if self.should_spell_magic() {
            if self.debug {
                eprintln!("Sepll Magic!!");
            }
            best_search_result.command = Command::Spell;
            self.last_best_search_result = None;
            return best_search_result;
        }

        if self.debug {
            eprintln!("Turn: {}", current_turn);
            eprintln!("Rest Time(msec): {}", self.player.rest_time_milliseconds());
        }

        // beam search for a command
        let (beam_depth, beam_width): (usize, usize) = self.beam_search_config();
        self.last_best_search_result = None;
        if self.debug {
            eprintln!("Beam depth: {}, Beam width: {}", beam_depth, beam_width);
        }
        let mut search_state_heap: Vec<MinMaxHeap<SearchState>> =
            (0..beam_depth + 1).map(|_| MinMaxHeap::new()).collect();
        let mut searched_state = fnv::FnvHashSet::default();

        //Create an initial state
        let root_search_state = SearchState::default()
            .with_board(self.player.board())
            .with_obstacle_block_count(self.player.obstacle_block_count())
            .with_spawn_obstacle_block_count(self.enemy.obstacle_block_count())
            .with_cumulative_game_score(self.player.cumulative_game_score());
        //push an initial search state
        search_state_heap[0].push(root_search_state);
        let mut rnd = Xorshift::with_seed(current_turn as u64 + self.seed);

        //gaze enemy...

        let need_kill_chain_count = self.gaze_enemy_need_kill_chain_count();
        let (max_enemy_chain_count, height) = self
            .evaluate_cache
            .estimate_with_erasing_all_max_chain_count(&mut self.simulator, &self.enemy.board());
        let a = 1.07;
        let mut target_enemy_chain_count = max_enemy_chain_count as f64;
        for i in 0..height {
            target_enemy_chain_count *= a;
        }
        eprintln!("Before: target_enemy_chain_count: {}", target_enemy_chain_count);
        let target_enemy_chain_count = std::cmp::min(17, std::cmp::max(DEFAULT_FATAL_FIRE_MAX_CHAIN_COUNT, target_enemy_chain_count as u8));
        eprintln!("After: target_enemy_chain_count: {}", target_enemy_chain_count);
        for depth in 0..beam_depth {
            //next state
            let search_turn = current_turn + depth;
            if best_search_result.fire_right_now {
                eprintln!("Fire right now!!");
                break;
            }
            while let Some(search_state) = &mut search_state_heap[depth].pop_max() {
                //Update obstacle block
                search_state.update_obstacle_block_and_drop();
                //skip duplicate

                for (pack, rotate_count) in self.packs[search_turn].iter() {
                    for point in 0..9 {
                        let mut board = search_state.board();
                        let chain_count = self.simulator.simulate(&mut board, point, &pack);
                        //Next board is dead and not to put it in state heap
                        if board.is_game_over() {
                            continue;
                        }

                        //update these values
                        let gain_chain_game_score = simulator::calculate_game_score(chain_count);
                        let next_board = board;
                        let next_cumulative_game_score =
                            gain_chain_game_score + search_state.cumulative_game_score();
                        let next_spawn_obstacle_block_count =
                            simulator::calculate_obstacle_count_from_chain_count(chain_count)
                                + search_state.spawn_obstacle_block_count();
                        //create next search state from a previous state
                        let mut next_search_state = search_state
                            .clone()
                            .with_board(next_board)
                            .with_cumulative_game_score(next_cumulative_game_score)
                            .with_spawn_obstacle_block_count(next_spawn_obstacle_block_count);

                        next_search_state.update_obstacle_block();
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
                        debug_assert_eq!(
                            search_state.cumulative_game_score() + gain_chain_game_score,
                            next_search_state.cumulative_game_score()
                        );

                        // Add a tiny value(0.0 ~ 1.0) to search score
                        // To randomize search score for the diversity of search
                        let next_search_score = self
                            .evaluate_cache
                            .evaluate_search_score(&mut self.simulator, &next_search_state)
                            + rnd.randf();
                        next_search_state.set_search_score(next_search_score);

                        //push it to next beam
                        //prune fire state
                        if chain_count <= 10 {
                            search_state_heap[depth + 1].push(next_search_state);
                            //The number of next beam is over beam_width; pop minimum state
                            while search_state_heap[depth + 1].len() > beam_width {
                                search_state_heap[depth + 1].pop_min();
                            }
                            debug_assert!(search_state_heap[depth + 1].len() <= beam_width);
                        }

                        let mut target_search_result_score = if self.kill_bomber_mode() {
                            evaluate_search_result_score_for_bomber(
                                chain_count,
                                next_search_score,
                                depth
                            )
                        } else {
                            evaluate_search_result_score(
                                gain_chain_game_score,
                                next_search_score,
                                depth,
                                target_enemy_chain_count
                            )
                        };
                        //penalty for low chain count

                        if chain_count < target_enemy_chain_count && (target_enemy_chain_count - chain_count >= 3) {
                            let diff_chain_count = std::cmp::min(19, (target_enemy_chain_count - chain_count)) as usize;
                            target_search_result_score.0 *= GAME_SCORE_DEPTH_RATES[diff_chain_count];

                        }
                        //consider whether solver should fire at depth 0
                        /*let fire_right_now = if depth == 0
                            && self.should_fire_right_now(
                                chain_count,
                                max_enemy_chain_count,
                                need_kill_chain_count,
                            ) {
                            true
                        } else {
                            false
                        };
                        if fire_right_now {
                            target_search_result_score.0 *= FIRE_RIGHT_NOW_BOOST_SCORE;
                        }*/


                        //pick highest search result score
                        if target_search_result_score > best_search_result.search_result_score {
                            best_search_result.search_result_score = target_search_result_score;
                            best_search_result.gain_game_score = gain_chain_game_score;
                            best_search_result.cumulative_game_score =
                                next_search_state.cumulative_game_score();
                            best_search_result.last_chain_count = chain_count;
                            best_search_result.search_depth = depth;
                            best_search_result.board = next_search_state.board();
                            best_search_result.command = next_search_state.command().unwrap();
                            best_search_result.fire_right_now = false;
                        }
                    }
                }
            }
        }
        if self.debug {
            eprintln!("== Search Result ==");
            best_search_result.log();
        }
        self.last_best_search_result = Some((
            best_search_result.last_chain_count,
            best_search_result.search_depth,
        ));
        best_search_result
    }
}

