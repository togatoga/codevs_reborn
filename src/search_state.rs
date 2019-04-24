use crate::field::Field;
use crate::command::Command;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SearchStatus {
    pub field: Field,
    pub obstacle_block_count: u32,
    pub skill_point: u32,
    pub cumulative_game_score: u32,
    pub command: Option<Command>,
    pub search_score: f64,
}

impl Eq for SearchStatus {}

impl PartialOrd for SearchStatus {
    fn partial_cmp(&self, other: &SearchStatus) -> Option<Ordering> {
        self.search_score.partial_cmp(&other.search_score)
    }
}

impl Ord for SearchStatus {
    fn cmp(&self, other: &SearchStatus) -> Ordering {
        self.search_score.partial_cmp(&other.search_score).unwrap()
    }
}


impl SearchStatus {
    pub fn new(field: &Field) -> SearchStatus {
        SearchStatus { field: *field, obstacle_block_count: 0, skill_point: 0, cumulative_game_score: 0, command: None, search_score: 0.0 }
    }
    pub fn with_cumulative_game_score(&mut self, cumulative_game_score: u32) -> SearchStatus {
        self.cumulative_game_score = cumulative_game_score;
        *self
    }
    pub fn with_search_score(&mut self, search_score: f64) -> SearchStatus {
        self.search_score = search_score;
        *self
    }
    pub fn with_command(&mut self, command: Command) -> SearchStatus {
        if self.command.is_none() {
            self.command = Some(command);
        }
        *self
    }
}


#[test]
fn test_compare_search_state() {
    let field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 6, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 2, 0, 0],
        [0, 0, 0, 0, 0, 0, 11, 11, 0, 0],
        [0, 0, 0, 0, 0, 11, 7, 7, 0, 0],
        [0, 11, 11, 11, 0, 11, 11, 11, 0, 0],
        [11, 11, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 6, 11, 11, 11, 11, 11, 11, 11, 11],
        [11, 11, 11, 11, 11, 6, 2, 9, 11, 11],
        [11, 11, 6, 6, 11, 6, 2, 6, 11, 11]
    ];

    let lower = SearchStatus::new(&Field::new(field)).with_search_score(-1000.0);
    let med = SearchStatus::new(&Field::new(field)).with_search_score(0.0);
    let higher = SearchStatus::new(&Field::new(field)).with_search_score(100000.0);
    let mut heaps = BinaryHeap::new();
    heaps.push(med);
    heaps.push(higher);
    heaps.push(lower);
    assert_eq!(heaps.pop(), Some(higher));
    assert_eq!(heaps.pop(), Some(med));
    assert_eq!(heaps.pop(), Some(lower));


}