mod pack;
mod simulator;
mod field;
mod search_state;
mod command;
mod evaluation;
mod xorshift;

use std::io::{StdinLock, Stdin};


use crate::pack::Pack;
use crate::command::Command;
use crate::search_state::SearchStatus;
use crate::field::{Field, FIELD_WIDTH, INPUT_FIELD_HEIGHT};
use crate::xorshift::Xorshift;
use std::collections::{BinaryHeap, HashSet};
use crate::evaluation::evaluate_search_score;


const MAX_TURN: usize = 500;

#[derive(Debug)]
struct GameStatus {
    rest_time_milliseconds: u32,
    obstacle_block_count: u32,
    skill_point: u32,
    cumulative_game_score: u32,
    field: Field,
}


#[derive(Debug)]
struct Solver<'a> {
    packs: &'a Vec<Pack>,
    player: GameStatus,
    enemy: GameStatus,
}

impl<'a> Solver<'a> {
    fn new(packs: &'a Vec<Pack>, player: GameStatus, enemy: GameStatus) -> Solver {
        Solver { packs, player, enemy }
    }
    fn read_packs(sc: &mut Scanner<StdinLock>) -> Vec<Pack> {
        (0..MAX_TURN).map(|_| {
            let mut blocks = vec![0; 4];
            for i in 0..4 {
                blocks[i] = sc.read::<u8>();
            }
            let end: String = sc.read();
            assert_eq!(end, "END");
            Pack { blocks }
        }).collect::<Vec<Pack>>()
    }

    fn read_game_status(sc: &mut Scanner<StdinLock>) -> GameStatus {
        //read player data
        let rest_time_milliseconds: u32 = sc.read();
        let obstacle_block_count: u32 = sc.read();
        let skill_point: u32 = sc.read();
        let cumulative_game_score: u32 = sc.read();

        let mut input_field: [[u8; FIELD_WIDTH]; INPUT_FIELD_HEIGHT] = [[0; FIELD_WIDTH]; INPUT_FIELD_HEIGHT];
        for y in 0..INPUT_FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                input_field[y][x] = sc.read::<u8>();
            }
        }
        let end: String = sc.read();
        assert_eq!(end, "END");
        let field = Field::new(input_field);
        GameStatus { rest_time_milliseconds, obstacle_block_count, skill_point, cumulative_game_score, field }
    }

    pub fn think(&mut self, current_turn: usize) -> Option<Command> {

        let player = &self.player;
        let enemy = &self.enemy;
        eprintln!("rest time {}", player.rest_time_milliseconds);
        let root_search_state =
            SearchStatus::new(&player.field)
            .with_obstacle_block_count(player.obstacle_block_count)
            .with_spawn_obstacle_block_count(enemy.obstacle_block_count)
            .with_cumulative_game_score(player.cumulative_game_score);

        const FIRE_MAX_CHAIN_COUNT: u8 = 11;
        //Fire if chain count is over threshold
        {
            let mut search_state = root_search_state.clone();
            let mut max_chain_count = 0;
            let mut command: Option<Command> = None;
            search_state.update_obstacle_block();
            for rotate_count in 0..5 {
                let mut pack = self.packs[current_turn].clone();
                //rotate
                pack.rotates(rotate_count);
                for point in 0..9 {
                    let (_, chain_count) = simulator::simulate(&mut search_state.field.clone(), point, &pack);
                    if chain_count >= FIRE_MAX_CHAIN_COUNT && chain_count > max_chain_count {
                        max_chain_count = chain_count;
                        command = Some(Command::Drop((point, rotate_count)));
                    }
                }
            }
            //Fire!!
            if command.is_some() {
                return command;
            }
        }

        // beam search for a command
        const BEAM_DEPTH: usize = 15;
        const BEAM_WIDTH: usize = 80;
        let mut search_state_heap: Vec<BinaryHeap<SearchStatus>> = (0..BEAM_DEPTH + 1).map(|_| BinaryHeap::new()).collect();
        let mut searched_field: Vec<HashSet<Field>> = (0..BEAM_DEPTH + 1).map(|_| HashSet::new()).collect();

        //push an initial search state
        search_state_heap[0].push(root_search_state);
        let mut rnd = Xorshift::with_seed(current_turn as u64);
        for depth in 0..BEAM_DEPTH {
            //next state
            let search_turn = current_turn + depth;
            let mut iter = 0;
            while let Some( search_state) = &mut search_state_heap[depth].pop() {
                //Update obstacle block
                search_state.update_obstacle_block();
                //skip duplicate
                if searched_field[depth].contains(&search_state.field) {
                    continue
                }
                //insert
                searched_field[depth].insert(search_state.field);
                iter += 1;
                for rotate_count in 0..5 {
                    let mut pack = self.packs[search_turn].clone();
                    //rotate
                    pack.rotates(rotate_count);
                    for point in 0..9 {
                        let mut field = search_state.field.clone();
                        let (score, chain_count) = simulator::simulate(&mut field, point, &pack);
                        //Next field is dead and not to put it in state heap
                        if field.is_game_over() {
                            continue;
                        }
                        let mut next_search_state = search_state.clone()
                            .with_field(field)
                            .with_command(Command::Drop((point, rotate_count)))
                            .add_cumulative_game_score(score)
                            .add_spawn_obstacle_block_count(simulator::calculate_obstacle_count(chain_count, 0));
                        // Add a tiny value(0.0 ~ 1.0) to search score
                        // To randomize search score for the diversity of search
                        next_search_state.with_search_score(evaluation::evaluate_search_score(&next_search_state) + rnd.randf());
                        search_state_heap[depth + 1].push(next_search_state);
                    }
                }
                if iter >= BEAM_WIDTH {
                    break;
                }
            }
        }

        if let Some(result) = search_state_heap[BEAM_DEPTH].pop() {

            eprintln!("cumulative_score = {}, search_score = {}", result.cumulative_game_score, result.search_score);
            return result.command;
        }
        None
    }
}

#[test]
fn test_think_must_be_dead() {

}

fn solve() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    println!("togatogAI");

    let packs: Vec<Pack> = Solver::read_packs(&mut sc);
    loop {
        let current_turn: usize = sc.read();
        //read player data
        let player = Solver::read_game_status(&mut sc);
        let enemy = Solver::read_game_status(&mut sc);
        let mut solver = Solver::new(&packs, player, enemy);
        let command = solver.think(current_turn).unwrap_or(Command::Drop((0, 0)));
        match command {
            Command::Drop(v) => {
                println!("{} {}", v.0, v.1);
            }
            Command::Spell => {
                println!("S");
            }
            _ => {
                assert!(false);
            }
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| solve())
        .unwrap()
        .join()
        .unwrap();
}

//snippet from kenkoooo
pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self.stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_line(&mut self) -> String {
        use std::io::Read;
        let buf = self.stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b'\n' || b == b'\r')
            .take_while(|&b| b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
