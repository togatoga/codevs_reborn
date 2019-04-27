#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;
use std::fs::File;

//internal
extern crate togatog_ai;

use togatog_ai::solver_config::SolverConfig;
use togatog_ai::solver::Solver;
use togatog_ai::search_result::SearchResult;

fn solver_think_from_file(pack_file_name: &str, info_file_name: &str, config: SolverConfig) -> SearchResult {
    let pack_file = File::open(pack_file_name).expect("can't open a file");
    let info_file = File::open(info_file_name).expect("can't open a file");
    //read from pack file
    let mut pack = togatog_ai::scanner::Scanner { stdin: pack_file };
    let mut info = togatog_ai::scanner::Scanner { stdin: info_file };
    let packs: Vec<Vec<(togatog_ai::pack::Pack, usize)>> = togatog_ai::solver::Solver::read_packs(&mut pack);
    //read information only one turn
    let current_turn: usize = info.read();
    let player = togatog_ai::solver::Solver::read_game_status(&mut info);
    let enemy = togatog_ai::solver::Solver::read_game_status(&mut info);
    let mut solver = togatog_ai::solver::Solver::new(&packs, player, enemy);
    solver.set_config(config);
    //measure
    solver.think(current_turn)
}


mod solver {
    use super::*;
    use criterion::Benchmark;
    use togatog_ai::solver_config::{DEFAULT_BEAM_DEPTH, DEFAULT_BEAM_WIDTH, DEFAULT_FIRE_MAX_CHAIN_COUNT};

    pub fn think(c: &mut Criterion) {
        c.bench("think",
                Benchmark::new("initial", |b| b.iter(|| {
                    for i in 0..10 {
                        let pack_file_name: &str = &format!("input/pack/pack_{:04}.pack", i);
                        let info_file_name: &str = "input/information/initial.info";
                        solver_think_from_file(pack_file_name, info_file_name, SolverConfig::new(DEFAULT_BEAM_DEPTH, DEFAULT_BEAM_WIDTH, DEFAULT_FIRE_MAX_CHAIN_COUNT));
                    }
                })).sample_size(5),
        );
    }
}

mod simulator {
    use togatog_ai::{evaluation, board};
    use criterion::{Criterion, Benchmark};


    pub fn estimate_max_chain_count(c: &mut Criterion) {
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
        let board_12_chain = board::Board::new(board);
        c.bench("estimate_max_chain_count",
                Benchmark::new("estimate_max_chain_count", move |b| b.iter(|| {
                    evaluation::estimate_max_chain_count(&board_12_chain.clone())
                })),
        );
    }
}
criterion_group!(benches,solver::think, simulator::estimate_max_chain_count);
criterion_main!(benches);