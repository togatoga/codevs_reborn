pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT: usize = 16;
pub const EMPTY_BLOCK: u8 = 0;

#[derive(Debug)]
pub struct Field {
    field: [[u8; FIELD_WIDTH]; FIELD_HEIGHT],
    bottoms: [u8; FIELD_WIDTH],
}


pub fn new(field: [[u8; FIELD_WIDTH]; FIELD_HEIGHT]) -> Field {
    let mut bottoms: [u8; FIELD_WIDTH] = [0; FIELD_WIDTH];
    for x in 0..FIELD_WIDTH {
        let mut y = 0;
        while y < FIELD_HEIGHT && field[y][x] == EMPTY_BLOCK {
            y += 1;
        }
        bottoms[x] = y as u8;
    }
    Field { field, bottoms }
}


#[test]
fn test_bottoms() {
    let field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        [0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let field = new(field);
    assert_eq!(field.bottoms, [16, 15, 14, 13, 12, 11, 10, 9, 8, 7]);
}