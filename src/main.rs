extern crate clap;
extern crate serde_derive;

use clap::{SubCommand, ArgMatches};

extern crate togatog_ai;

use togatog_ai::scanner;
use togatog_ai::solver::Solver;
use togatog_ai::solver_config::{SolverConfig, SOLVER_VERSION};
use togatog_ai::command::Command;

fn bench(pack: std::fs::File, info: std::fs::File, seed: u64, output_file: std::fs::File) {
    let mut pack = scanner::Scanner { stdin: pack };
    let mut information = scanner::Scanner { stdin: info };

    let mut solver = Solver::default().with_seed(seed);
    solver.set_packs(Solver::read_packs(&mut pack));

    //read information and think at only one turn
    let current_turn: usize = information.read();
    let player = Solver::read_game_status(&mut information);
    let enemy = Solver::read_game_status(&mut information);
    solver.set_turn(current_turn);
    solver.set_game_status(player, enemy);
    solver.set_config(SolverConfig::default().with_beam(15, 500));
    let best_result = solver.think();

    best_result.to_csv(output_file).unwrap();
}

fn profile(pack: std::fs::File, info: std::fs::File, seed: u64) {
    let mut pack = scanner::Scanner { stdin: pack };
    let mut information = scanner::Scanner { stdin: info };

    let mut solver = Solver::default().with_seed(seed);

    solver.set_packs(Solver::read_packs(&mut pack));
    //read information and think at only one turn
    let current_turn: usize = information.read();
    let player = Solver::read_game_status(&mut information);
    let enemy = Solver::read_game_status(&mut information);
    solver.set_turn(current_turn);
    solver.set_game_status(player, enemy);
    solver.set_config(SolverConfig::default().with_beam(15, 500));
    let best_result = solver.think();
    eprintln!("{:?}", best_result);
}

fn run(matches: ArgMatches) {

    if let Some(matches) = matches.subcommand_matches("profile") {
        let pack = std::fs::File::open(matches.value_of("pack").expect("Invalid for pack file")).expect("Can't open a file");
        let info = std::fs::File::open(matches.value_of("info").expect("Invalid for information file")).expect("Can't open a file");
        let seed: u64 = matches.value_of("seed").unwrap().parse().unwrap();
        profile(pack, info, seed);
        return ;
    }
    if let Some(matches) = matches.subcommand_matches("bench") {
        let pack = std::fs::File::open(matches.value_of("pack").expect("Invalid for pack file")).expect("Can't open a file");
        let info = std::fs::File::open(matches.value_of("info").expect("Invalid for information file")).expect("Can't open a file");
        let output = std::fs::File::create(matches.value_of("output").expect("Invalid for output file")).expect("Can't create a file");
        let seed: u64 = matches.value_of("seed").unwrap().parse().unwrap();
        bench(pack, info, seed, output);
        return;
    }
    let seed: u64 = matches.value_of("seed").unwrap().parse().unwrap();

    let debug = matches.is_present("debug");
    //START!!
    if debug {
        eprintln!("togatog_ai_{}", SOLVER_VERSION);
        eprintln!("seed: {}", seed);
    }
    println!("togatog_ai_{}", SOLVER_VERSION);
    let s = std::io::stdin();
    let mut sc = scanner::Scanner { stdin: s.lock() };
    //create a default solver object
    let mut solver = Solver::default().with_seed(seed);
    //set debug option
    solver.set_debug(debug);
    //read and set packs
    solver.set_packs(Solver::read_packs(&mut sc));
    loop {
        let current_turn: usize = sc.read();
        solver.set_turn(current_turn);
        //read player data
        let player = Solver::read_game_status(&mut sc);
        let enemy = Solver::read_game_status(&mut sc);
        solver.set_game_status(player, enemy);
        let best_result = solver.think();
        Solver::output_command(best_result.command);
    }
}

fn main() {
    let matches = clap::App::new("solver")
        .about("A Solver for CODEVS Reborn")
        .version(SOLVER_VERSION)
        .author("togatoga")
        .subcommand(SubCommand::with_name("bench").about("Run benchmarks")
            .arg(clap::Arg::with_name("pack").help("The path of a pack file").short("p").long("pack").value_name("PACK").required(true))
            .arg(clap::Arg::with_name("info").help("The path of an information file").short("i").long("info").value_name("INFORMATION").required(true))
            .arg(clap::Arg::with_name("output").help("The path of an output csv file").short("o").long("output").value_name("OUTPUT").required(true))
            .arg(clap::Arg::with_name("seed").short("s").long("seed").help("seed for generating random number").default_value("28"))
        )
        .subcommand(SubCommand::with_name("profile").about("Run for profiler")
            .arg(clap::Arg::with_name("pack").help("The path of a pack file").short("p").long("pack").value_name("PACK").required(true))
            .arg(clap::Arg::with_name("info").help("The path of an information file").short("i").long("info").value_name("INFORMATION").required(true))
        )
        .arg(clap::Arg::with_name("debug").short("d").long("debug").help("print debug information verbosely"))
        .arg(clap::Arg::with_name("seed").short("s").long("seed").help("seed for generating random number").default_value("1024"))
        .get_matches();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| run(matches))
        .unwrap()
        .join()
        .unwrap();
}
