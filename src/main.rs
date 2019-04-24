mod pack;
mod simulator;
mod field;

use std::io::{StdinLock, Stdin};


use crate::pack::Pack;
use crate::field::{Field, FIELD_WIDTH, INPUT_FIELD_HEIGHT};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const MAX_TURN: usize = 500;

#[derive(Debug)]
struct GameStatus {
    rest_time_milliseconds: u32,
    obstacle_block_count: u32,
    skill_point: u32,
    cumulative_game_score: u32,
    field: Field,
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Drop((usize, usize)),
    Spell,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct SearchStatus {
    field: Field,
    obstacle_block_count: u32,
    skill_point: u32,
    cumulative_game_score: u32,
    command: Command,
    search_score: f64,
}

impl PartialOrd for SearchStatus {
    fn partial_cmp(&self, other: &SearchStatus) -> Option<Ordering> {
        Some(self.search_score.partial_cmp(&other.search_score))
    }
}
impl Ord for SearchStatus {
    fn cmp(&self, other: &SearchStatus) -> Ordering {
        self.search_score.partial_cmp(&other.search_score)
    }
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
        let field = field::new(input_field);
        GameStatus { rest_time_milliseconds, obstacle_block_count, skill_point, cumulative_game_score, field }
    }

    pub fn think(&mut self, current_turn: usize) -> Command {
        const BEAM_DEPTH: usize = 10;
        const BEAM_WIDTH: usize = 30;
        let mut command = Command::Drop((0, 0));
        let search_state_heap: Vec<BinaryHeap<SearchStatus>> = (0..BEAM_DEPTH).map(|_| BinaryHeap::new()).collect();

        for depth in 0..BEAM_DEPTH {
            let search_turn = current_turn + depth;

            //next state
            for point in 0..9 {
                for rotate_count in 0..5 {
                    let mut pack = self.packs[search_turn];
                    pack.rotates(rotate_count);

                }
            }
        }
        command
    }
}

fn solve() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    println!("togatogAI");
    //parse packn
    let packs: Vec<Pack> = Solver::read_packs(&mut sc);
    loop {
        let current_turn: usize = sc.read();
        //read player data
        let player = Solver::read_game_status(&mut sc);
        let enemy = Solver::read_game_status(&mut sc);
        let mut solver = Solver::new(&packs, player, enemy);
        let command = solver.think(current_turn);
        match command {
            Command::Drop(v) => {
                println!("{} {}", v.0, v.1);
            }
            Command::Spell => {
                println!("S");
            },
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
