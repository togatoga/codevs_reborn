mod pack;
mod simulator;
mod field;
mod search_state;
mod command;
mod evaluation;
mod xorshift;
mod scanner;
mod solver;
mod game_status;

use crate::command::Command;
use crate::solver::Solver;

fn solve() {
    let s = std::io::stdin();
    let mut sc = scanner::Scanner { stdin: s.lock() };
    println!("togatogAI");

    let packs: Vec<pack::Pack> = Solver::read_packs(&mut sc);
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
