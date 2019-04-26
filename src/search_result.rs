use crate::field::Field;
use crate::command::Command;

extern crate csv;

use csv::Writer;


#[derive(Debug, Clone)]
pub struct SearchResult {
    pub last_chain_count: u8,
    pub cumulative_game_score: u32,
    pub turn: usize,
    pub field: Field,
    pub command: Command,
}


impl SearchResult {
    pub fn to_csv<T: std::io::Write>(&self, file: T) -> Result<(), Box<std::error::Error>> {
        let mut wtr = Writer::from_writer(file);
        wtr.write_record(&["cumulative_game_score", "last_chain_count", "best_result.turn"])?;
        wtr.serialize(vec![self.cumulative_game_score as u32, self.last_chain_count as u32, self.turn as u32])?;
        Ok(())
    }
}
