extern crate clap;
extern crate serde_derive;
use clap::{SubCommand, ArgMatches};

extern crate togatog_ai;
use togatog_ai::scanner;
use togatog_ai::pack;
use togatog_ai::solver::Solver;
use togatog_ai::solver_config::{SolverConfig, SOLVER_VERSION};

fn bench(pack: std::fs::File, info: std::fs::File, output_file: std::fs::File) {
    let mut pack = scanner::Scanner { stdin: pack };
    let mut information = scanner::Scanner { stdin: info };
    let packs: Vec<Vec<(pack::Pack, usize)>> = Solver::read_packs(&mut pack);

    //read information only one turn
    let current_turn: usize = information.read();
    let player = Solver::read_game_status(&mut information);
    let enemy = Solver::read_game_status(&mut information);
    let config = SolverConfig::new(15, 500, 10);
    let mut solver = Solver::new(&packs, player, enemy);
    solver.set_config(config);
    let best_result = solver.think(current_turn);

    best_result.to_csv(output_file).unwrap();
}

fn run(matches: ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("bench") {
        let pack = std::fs::File::open(matches.value_of("pack").expect("Invalid for pack file")).expect("Can't open a file");
        let info = std::fs::File::open(matches.value_of("info").expect("Invalid for information file")).expect("Can't open a file");
        let output = std::fs::File::create(matches.value_of("output").expect("Invalid for output file")).expect("Can't create a file");
        bench(pack, info, output);
        return;
    }

    let s = std::io::stdin();
    let mut sc = scanner::Scanner { stdin: s.lock() };
    let packs: Vec<Vec<(pack::Pack, usize)>> = Solver::read_packs(&mut sc);
    let debug = matches.is_present("debug");

    //START!!
    if debug {
        eprintln!("togatog_ai_{}", SOLVER_VERSION);
    }
    println!("togatog_ai_{}", SOLVER_VERSION);
    loop {
        let current_turn: usize = sc.read();
        //read player data
        let player = Solver::read_game_status(&mut sc);
        let enemy = Solver::read_game_status(&mut sc);
        let mut solver = Solver::new(&packs, player, enemy);
        solver.set_debug(debug);
        let best_result = solver.think(current_turn);
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
        ).arg(clap::Arg::with_name("debug").short("d").long("debug").help("print debug information verbosely"))
        .get_matches();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| run(matches))
        .unwrap()
        .join()
        .unwrap();
}
