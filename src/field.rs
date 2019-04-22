pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 16;
pub const EMPTY_BLOCK: u8 = 0;

#[derive(Debug)]
pub struct Field {
    field: [[u8; FIELD_WIDTH]; FIELD_HEIGHT],
    heights: [u8; FIELD_WIDTH],
}


pub fn new(field: [[u8; FIELD_WIDTH]; FIELD_HEIGHT]) -> Field {
    let mut heights: [u8; FIELD_WIDTH] = [0; FIELD_WIDTH];
    for x in 0..FIELD_WIDTH {
        let mut y = 0;
        while y < FIELD_HEIGHT && field[y][x] == EMPTY_BLOCK {
            y += 1;
        }
        heights[x] = y as u8;
    }
    Field { field, heights }
}

