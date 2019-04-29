use crate::board::{Board, FIELD_HEIGHT, FIELD_WIDTH};
use crate::command::Command;

extern crate csv;

use csv::Writer;


#[derive(Debug, Clone)]
pub struct SearchResult {
    pub last_chain_count: u8,
    pub cumulative_game_score: u32,
    pub gain_game_score: u32,
    pub search_depth: usize,
    pub board: Board,
    pub command: Command,
}


impl SearchResult {
    pub fn default() -> SearchResult {
        SearchResult { last_chain_count: 0, cumulative_game_score: 0, gain_game_score: 0, search_depth: 0, board: Board::default(), command: Command::default() }
    }
    pub fn to_csv<T: std::io::Write>(&self, file: T) -> Result<(), Box<std::error::Error>> {
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(&["cumulative_game_score", "last_chain_count", "search_depth"])?;
        wtr.serialize(vec![self.cumulative_game_score as u32, self.last_chain_count as u32, self.search_depth as u32])?;
        Ok(())
    }

    pub fn log(&self) {
        eprintln!("cumulative_game_score: {}", self.cumulative_game_score);
        eprintln!("gain_game_score: {}", self.gain_game_score);
        eprintln!("last_chain_count: {}", self.last_chain_count);
        eprintln!("search_depth: {}", self.search_depth);
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


