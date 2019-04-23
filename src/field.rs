pub const FIELD_WIDTH: usize = 10;
pub const INPUT_FIELD_HEIGHT: usize = 16;
pub const FIELD_HEIGHT: usize = 18;
pub const DANGER_LINE_HEIGHT: usize = FIELD_HEIGHT + 1;
pub const EMPTY_BLOCK: u8 = 0;
pub const OBSTACLE_BLOCK: u8 = 11;
pub const ERASING_SUM: u8 = 10;


#[derive(Debug)]
pub struct Field {
    pub field: [[u8; FIELD_WIDTH]; FIELD_HEIGHT],
    //y starts from under left point
    pub heights: [usize; FIELD_WIDTH],
}

impl Field {
    pub fn drop_obstacles(&mut self) {
        for x in 0..FIELD_WIDTH {
            assert!(self.heights[x] < FIELD_HEIGHT);
            self.field[self.heights[x]][x] = OBSTACLE_BLOCK;
            self.heights[x] += 1;
        }
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Field) -> bool {
        self.field == other.field && self.heights == other.heights
    }
}


pub fn new(input_field: [[u8; FIELD_WIDTH]; INPUT_FIELD_HEIGHT]) -> Field {
    let mut heights: [usize; FIELD_WIDTH] = [0; FIELD_WIDTH];
    for x in 0..FIELD_WIDTH {
        let mut y = 0;
        while y < INPUT_FIELD_HEIGHT && input_field[INPUT_FIELD_HEIGHT - 1 - y][x] != EMPTY_BLOCK {
            y += 1;
        }
        heights[x] = y;
    }
    let mut field: [[u8; FIELD_WIDTH]; FIELD_HEIGHT] = [[0; FIELD_WIDTH]; FIELD_HEIGHT];
    for y in 0..INPUT_FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            field[y][x] = input_field[INPUT_FIELD_HEIGHT - 1 - y][x];
        }
    }

    Field { field, heights }
}


#[test]
fn test_heights() {
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
    assert_eq!(field.heights, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn drop_obstacles() {
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
    let mut field = new(field);
    //drop
    field.drop_obstacles();
    //
    let dropped_obstacles_field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 11],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 1],
        [0, 0, 0, 0, 0, 0, 0, 11, 1, 1],
        [0, 0, 0, 0, 0, 0, 11, 1, 1, 1],
        [0, 0, 0, 0, 0, 11, 1, 1, 1, 1],
        [0, 0, 0, 0, 11, 1, 1, 1, 1, 1],
        [0, 0, 0, 11, 1, 1, 1, 1, 1, 1],
        [0, 0, 11, 1, 1, 1, 1, 1, 1, 1],
        [0, 11, 1, 1, 1, 1, 1, 1, 1, 1],
        [11, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
    let mut dropped_obstacles_field = new(dropped_obstacles_field);
    assert_eq!(field, dropped_obstacles_field);
}