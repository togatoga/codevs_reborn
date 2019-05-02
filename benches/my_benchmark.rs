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
    //solver object
    let mut solver = togatog_ai::solver::Solver::default();
    solver.set_packs(togatog_ai::solver::Solver::read_packs(&mut pack));
    //read information only one turn
    let current_turn: usize = info.read();
    let player = togatog_ai::solver::Solver::read_game_status(&mut info);
    let enemy = togatog_ai::solver::Solver::read_game_status(&mut info);
    solver.set_game_status(player, enemy);
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
                        solver_think_from_file(pack_file_name, info_file_name, SolverConfig::default());
                    }
                })).sample_size(5),
        );
    }
}

criterion_group!(benches,solver::think, simulator::estimate_max_chain_count);
criterion_main!(benches);