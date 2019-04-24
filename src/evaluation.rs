use crate::field::Field;
use crate::pack::Pack;
use crate::simulator;
use crate::search_state::SearchStatus;


fn estimate_max_chain_count(field: &Field) -> (u8, Field) {
    let mut estimated_max_chain_count = 0;
    let mut estimated_field = field.clone();
    //drop single block and evaluate chain count
    for point in 0..9 {
        for num in 1..10 {
            let pack =  Pack {blocks: vec![0, 0, num, 0]};
            let mut simulated_field = field.clone();
            let (score, chain_count) = simulator::simulate(&mut simulated_field, point, &pack);
            if chain_count > estimated_max_chain_count {
                estimated_max_chain_count = chain_count;
                estimated_field = simulated_field;
            }

        }
    }
    (estimated_max_chain_count, estimated_field)
}

pub fn evaluate_search_score(search_state: &SearchStatus) -> f64 {
    let mut search_score: f64 = 0.0;

    let field = search_state.field;
    // max chain count
    let (estimated_max_chain_count, estimated_field) = estimate_max_chain_count(&field);

    search_score += estimated_max_chain_count as f64;
    search_score *= 10e5;
    // count live block
    search_score += (field.count_live_blocks() as f64 * 100.0) as f64;
    search_score
}

#[test]
fn test_estimate_max_chain_count() {

}
