pub const FIELD_WIDTH: usize = 10;
pub const INPUT_FIELD_HEIGHT: usize = 16;
pub const FIELD_HEIGHT: usize = 19;
pub const DANGER_LINE_HEIGHT: usize = INPUT_FIELD_HEIGHT + 1;
pub const EMPTY_BLOCK: u8 = 0;
pub const OBSTACLE_BLOCK: u8 = 11;
pub const ERASING_SUM: u8 = 10;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Field {
    pub field: [[u8; FIELD_WIDTH]; FIELD_HEIGHT],
    //y starts from under left point
    pub heights: [usize; FIELD_WIDTH],
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut field = vec![];
        for y in 0..FIELD_HEIGHT {
            field.push(format!("{:?}", self.field[FIELD_HEIGHT - 1 - y]));
        }
        write!(f, "Field {{ field: [{}], heights: [{:?}] }}", field.join(",\n"), self.heights)
    }
}
impl Field {
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

    pub fn drop_obstacles(&mut self) {
        for x in 0..FIELD_WIDTH {
            assert!(self.heights[x] < FIELD_HEIGHT);
            self.field[self.heights[x]][x] = OBSTACLE_BLOCK;
            self.heights[x] += 1;
        }
    }
    pub fn count_live_blocks(&self) -> u8 {
        let mut count = 0;
        for y in 0..FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                if self.field[y][x] != EMPTY_BLOCK && self.field[y][x] != OBSTACLE_BLOCK {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn is_game_over(&self) -> bool {
        for &x in self.heights.iter() {
            if x >= DANGER_LINE_HEIGHT {
                return true;
            }
        }
        false
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Field) -> bool {
        self.field == other.field && self.heights == other.heights
    }
}

#[test]
fn test_is_game_over() {
    let field = [
        [0, 0, 9, 11, 11, 4, 8, 8, 0, 0],
        [0, 2, 3, 11, 11, 11, 7, 9, 0, 0],
        [0, 2, 1, 1, 11, 11, 11, 11, 11, 0],
        [0, 3, 11, 6, 6, 11, 11, 11, 1, 0],
        [0, 8, 11, 11, 6, 6, 11, 11, 11, 0],
        [8, 7, 9, 2, 9, 7, 7, 7, 11, 0],
        [9, 5, 11, 4, 11, 9, 2, 9, 5, 11],
        [3, 8, 7, 8, 11, 11, 3, 9, 7, 9],
        [5, 9, 7, 11, 11, 11, 8, 11, 5, 6],
        [11, 11, 11, 11, 4, 3, 11, 9, 3, 11],
        [11, 11, 11, 11, 5, 11, 11, 5, 11, 11],
        [11, 11, 11, 2, 11, 11, 8, 11, 1, 7],
        [11, 11, 11, 4, 7, 4, 7, 11, 8, 11],
        [11, 11, 3, 11, 8, 7, 11, 11, 11, 11],
        [11, 11, 2, 5, 9, 5, 11, 5, 11, 11],
        [11, 11, 2, 3, 3, 2, 7, 1, 11, 11]
    ];
    let mut field = Field::new(field);
    assert!(!field.is_game_over());
    field.drop_obstacles();
    assert!(field.is_game_over());
}

#[test]
fn test_count_live_blocks() {
    let field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 7, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 4, 4, 8, 0, 0, 0, 0],
        [0, 0, 0, 9, 8, 4, 0, 0, 0, 0],
        [0, 0, 0, 3, 4, 8, 9, 0, 0, 0],
        [0, 0, 0, 5, 9, 4, 8, 0, 0, 0],
        [0, 0, 1, 6, 3, 4, 1, 0, 11, 0],
        [0, 0, 6, 5, 1, 2, 3, 4, 11, 0],
        [0, 0, 1, 3, 6, 2, 2, 1, 11, 0]
    ];
    let field = Field::new(field);
    assert_eq!(field.count_live_blocks(), 35);
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
    let field = Field::new(field);
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
    let mut field = Field::new(field);
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
    let dropped_obstacles_field = Field::new(dropped_obstacles_field);
    assert_eq!(field, dropped_obstacles_field);
}