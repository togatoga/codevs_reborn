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
mod solver_config;

extern crate clap;

use crate::command::Command;
use crate::solver::Solver;
use clap::{SubCommand, ArgMatches};

fn bench(pack: std::fs::File, info: std::fs::File) {
    let mut pack = scanner::Scanner { stdin: pack };
    let mut information = scanner::Scanner { stdin: info };
    let packs: Vec<pack::Pack> = Solver::read_packs(&mut pack);
    //read information only one turn
    let current_turn: usize = information.read();
    let player = Solver::read_game_status(&mut information);
    let enemy = Solver::read_game_status(&mut information);
    let mut solver = Solver::new(&packs, player, enemy);

    let command = solver.think(current_turn).unwrap_or(Command::Drop((0, 0)));
    println!("{:?}", command);
}

fn run(matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("bench") {
        let pack = std::fs::File::open(matches.value_of("pack").expect("Invalid for pack file")).expect("Can't open a file");
        let info = std::fs::File::open(matches.value_of("info").expect("Invalid for information file")).expect("Can't open a file");

        bench(pack, info);
        return;
    }

    let s = std::io::stdin();
    let mut sc = scanner::Scanner { stdin: s.lock() };
    println!("togatoga_ai");

    let packs: Vec<pack::Pack> = Solver::read_packs(&mut sc);
    loop {
        let current_turn: usize = sc.read();
        //read player data
        let player = Solver::read_game_status(&mut sc);
        let enemy = Solver::read_game_status(&mut sc);
        let mut solver = Solver::new(&packs, player, enemy);
        let command = solver.think(current_turn).unwrap_or(Command::Drop((0, 0)));
        Solver::output_command(command);
    }
}

fn main() {
    let matches = clap::App::new("solver")
        .about("A Solver for CODEVS Reborn")
        .version("1.0")
        .author("togatoga")
        .subcommand(SubCommand::with_name("bench").about("Run benchmarks")
            .arg(clap::Arg::with_name("pack").help("The path of a pack file").short("p").long("pack").value_name("PACK").required(true))
            .arg(clap::Arg::with_name("info").help("The path of an information file").short("i").long("info").value_name("INFORMATION").required(true))
        )
        .get_matches();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| run(matches))
        .unwrap()
        .join()
        .unwrap();
}
