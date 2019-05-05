use crate::board::Board;
use crate::command::Command;

extern crate csv;

use csv::Writer;

pub const FIRE_RIGHT_NOW_BOOST_SCORE: f64 = 1e20;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub search_result_score: f64,
    pub last_chain_count: u8,
    pub cumulative_game_score: u32,
    pub gain_game_score: u32,
    pub search_depth: usize,
    pub board: Board,
    pub command: Command,
    pub fire_right_now: bool
}


impl SearchResult {
    pub fn default() -> SearchResult {
        SearchResult { search_result_score: 0.0, last_chain_count: 0, cumulative_game_score: 0, gain_game_score: 0, search_depth: 0, board: Board::default(), command: Command::default(), fire_right_now: false }
    }
    pub fn to_csv<T: std::io::Write>(&self, file: T) -> Result<(), Box<std::error::Error>> {
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(&["cumulative_game_score", "last_chain_count", "search_depth"])?;
        wtr.serialize(vec![self.cumulative_game_score as u32, self.last_chain_count as u32, self.search_depth as u32])?;
        Ok(())
    }

    pub fn log(&self) {
        eprintln!("search_score: {:.10}", self.search_result_score);
        eprintln!("cumulative_game_score: {}", self.cumulative_game_score);
        eprintln!("gain_game_score: {}", self.gain_game_score);
        eprintln!("last_chain_count: {}", self.last_chain_count);
        eprintln!("search_depth: {}", self.search_depth);
        eprintln!("fire_right_now: {}", self.fire_right_now);
        /*eprintln!("Board: ");
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                eprint!("{} ", self.board.get(FIELD_HEIGHT - 1 - y, x));
            }
            eprintln!();
        }*/
        eprintln!("Command: {:?}", self.command);
    }
}


