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

const BENCHMARK_SOLVER_CONFIG: SolverConfig = SolverConfig { beam_depth: 15, beam_width: 300, fire_max_chain_count: 10 };

fn solver_think_from_file(pack_file_name: &str, info_file_name: &str, config: SolverConfig) -> SearchResult {
    let pack_file = File::open(pack_file_name).expect("can't open a file");
    let info_file = File::open(info_file_name).expect("can't open a file");
    //read from pack file
    let mut pack = togatog_ai::scanner::Scanner { stdin: pack_file };
    let mut info = togatog_ai::scanner::Scanner { stdin: info_file };
    let packs: Vec<togatog_ai::pack::Pack> = togatog_ai::solver::Solver::read_packs(&mut pack);
    //read information only one turn
    let current_turn: usize = info.read();
    let player = togatog_ai::solver::Solver::read_game_status(&mut info);
    let enemy = togatog_ai::solver::Solver::read_game_status(&mut info);
    let config = togatog_ai::solver_config::SolverConfig::new(15, 500, 10);
    let mut solver = togatog_ai::solver::Solver::new(&packs, player, enemy);
    solver.set_config(config);
    //measure
    solver.think(current_turn)
}


mod solver {
    use super::*;
    use criterion::Benchmark;

    pub fn think(c: &mut Criterion) {
        c.bench("think",
                Benchmark::new("initial", |b| b.iter(|| {
                    let pack_file_name = "input/pack/pack_0000.pack";
                    let info_file_name = "input/information/initial.info";
                    solver_think_from_file(pack_file_name, info_file_name, BENCHMARK_SOLVER_CONFIG)
                })).sample_size(10),
        );
    }
}


criterion_group!(benches,solver::think);
criterion_main!(benches);