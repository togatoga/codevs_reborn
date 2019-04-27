use crate::pack;
use crate::field;
use crate::field::{EMPTY_BLOCK, FIELD_WIDTH, FIELD_HEIGHT, OBSTACLE_BLOCK, ERASING_SUM, Field};
use std::collections::HashSet;
use crate::pack::Pack;

const CHAIN_CUMULATIVE_SCORES: [u32; 50] = [0, 1, 2, 4, 6, 9, 13, 19, 27, 37, 50, 67, 90, 120, 159, 210, 276, 362, 474, 620, 810, 1057, 1378, 1795, 2337, 3042, 3959, 5151, 6701, 8716, 11335, 14740, 19167, 24923, 32405, 42132, 54778, 71218, 92590, 120373, 156491, 203445, 264485, 343838, 446997, 581103, 755441, 982081, 1276713, 1659735];

const DIRECTION_YXS: [(i8, i8); 8] = [
    (0, 1),// right
    (-1, 1), //upper right
    (-1, 0), //top
    (-1, -1),//upper left
    (0, -1), //left
    (1, -1), //down left
    (1, 0),//down
    (1, 1), //down right
];

fn is_on_field(y: i8, x: i8) -> bool {
    if y < 0 || y as usize >= FIELD_HEIGHT {
        return false;
    }
    if x < 0 || x as usize >= FIELD_WIDTH {
        return false;
    }
    return true;
}

fn drop_pack(field: &mut field::Field, point: usize, pack: &pack::Pack) -> Vec<(usize, usize)> {
    assert!(point <= 8);

    let mut modified_blocks: Vec<(usize, usize)> = Vec::new(); //(y, x)
    for y in (0..2).rev() {
        for x in (0..2).rev() {
            //skip empty block
            let block = pack.blocks[2 * y + x];
            if block == EMPTY_BLOCK {
                continue;
            }
            let nx = point + x;
            assert!(nx < FIELD_WIDTH);
            let ny = field.heights[nx];
            assert!(ny < FIELD_HEIGHT);
            field.field[ny][nx] = block;
            field.heights[nx] += 1;
            modified_blocks.push((ny, nx));
        }
    }
    modified_blocks
}


pub fn calculate_obstacle_count(chain_count: u8, skill_chain_count: u32) -> u32 {
    (chain_count / 2) as u32 + skill_chain_count / 2
}

fn calculate_erase_blocks(field: &field::Field, modified_blocks: &Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut erase_blocks: HashSet<(usize, usize)> = HashSet::new();
    for &(y, x) in modified_blocks.iter() {
        let block = field.field[y][x];
        assert!(block != EMPTY_BLOCK && block != OBSTACLE_BLOCK);
        let y: i8 = y as i8;
        let x: i8 = x as i8;
        for &dyx in DIRECTION_YXS.iter() {
            if !is_on_field(y + dyx.0, x + dyx.1) {
                continue;
            }
            let ny: usize = (y + dyx.0) as usize;
            let nx: usize = (x + dyx.1) as usize;
            let neighbor_block = field.field[ny][nx];
            if neighbor_block == EMPTY_BLOCK || neighbor_block == OBSTACLE_BLOCK {
                continue;
            }
            //block and neighbor_block are erased
            if block + neighbor_block == ERASING_SUM {
                erase_blocks.insert((y as usize, x as usize));
                erase_blocks.insert((ny, nx));
            }
        }
    }
    return erase_blocks;
}

fn apply_erase_blocks(field: &mut field::Field, erase_blocks: &HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
    assert!(!erase_blocks.is_empty());

    let old_heights = field.heights;
    //erase
    for &(y, x) in erase_blocks.iter() {
        field.field[y][x] = EMPTY_BLOCK;
        //update heights
        field.heights[x] = std::cmp::min(field.heights[x], y);
    }

    let mut modified_blocks: Vec<(usize, usize)> = Vec::new();
    //erase and drop
    for x in 0..FIELD_WIDTH {
        let new_height = field.heights[x];
        let old_height = old_heights[x];
        for y in new_height..old_height {
            let drop_block = field.field[y][x];
            if drop_block == EMPTY_BLOCK {
                continue;
            }
            let ny = field.heights[x];
            field.field[ny][x] = drop_block;
            if drop_block != OBSTACLE_BLOCK {
                modified_blocks.push((ny, x));
            }
            field.heights[x] += 1;
            field.field[y][x] = EMPTY_BLOCK;
        }
    }
    modified_blocks
}

pub fn simulate(field: &mut field::Field, point: usize, pack: &pack::Pack) -> (u32, u8) {
    let mut modified_blocks = drop_pack(field, point, &pack);
    let mut score: u32 = 0;
    let mut chain_count: u8 = 0;
    while !modified_blocks.is_empty() {
        let erase_blocks = calculate_erase_blocks(&field, &modified_blocks);
        if erase_blocks.is_empty() {
            break;
        }
        chain_count += 1;
        modified_blocks = apply_erase_blocks(field, &erase_blocks);
    }
    score += CHAIN_CUMULATIVE_SCORES[chain_count as usize];
    (score, chain_count)
}

#[test]
fn test_simulate_same_field() {
    let field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 4, 0, 8, 0, 0, 0, 0, 0, 0],
        [0, 8, 0, 1, 0, 0, 0, 0, 0, 0],
        [7, 7, 0, 1, 0, 0, 0, 0, 0, 0],
        [6, 5, 0, 2, 0, 0, 0, 0, 0, 0],
        [2, 9, 4, 7, 1, 0, 0, 7, 2, 0],
        [6, 7, 2, 4, 8, 0, 0, 2, 1, 0],
        [2, 2, 7, 9, 9, 0, 0, 6, 3, 0],
        [7, 6, 6, 9, 4, 9, 3, 9, 3, 6]
    ];
    let old_field = Field::new(field);
    let mut simulated_field = Field::new(field);
    simulate(&mut simulated_field, 7, &Pack::new(&[0, 9, 1, 9]));
    // 0 9   => 0 0
    // 1 9      0 0
    assert_eq!(simulated_field, old_field);
}

#[test]
fn test_simulate_with_obstacles() {
    //score 210
    //chain_count 15
    let raw_field = [
        [0, 0, 0, 11, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 11, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 11, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 7, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 2, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 7, 0, 0, 0, 0, 0, 0],
        [0, 0, 11, 4, 11, 0, 0, 0, 0, 0],
        [0, 0, 11, 9, 11, 11, 7, 0, 0, 0],
        [0, 0, 11, 6, 11, 11, 11, 7, 0, 0],
        [0, 0, 3, 1, 8, 11, 11, 9, 0, 0],
        [0, 0, 8, 6, 1, 3, 5, 11, 0, 0],
        [0, 0, 6, 5, 8, 8, 1, 11, 0, 0],
        [0, 11, 7, 7, 7, 5, 11, 3, 0, 0],
        [11, 6, 9, 2, 1, 6, 2, 11, 11, 11],
        [11, 11, 3, 2, 5, 6, 2, 9, 11, 11],
        [11, 11, 3, 9, 3, 9, 2, 6, 11, 11]
    ];
    let mut field = field::Field::new(raw_field);
    let pack = pack::Pack::new(&[6, 7, 2, 0]);
    //drop
    field.drop_obstacles();
    let (score, chain_count) = simulate(&mut field, 7, &pack);
    assert_eq!((score, chain_count), (210, 15));

    let simulated_field = [
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
    let simulated_field = field::Field::new(simulated_field);
    assert_eq!(field, simulated_field);
}

#[test]
fn test_simulate() {
    //score 120
    //chain_count 13
    let raw_field = [
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
        [0, 0, 1, 6, 3, 4, 1, 0, 0, 0],
        [0, 0, 6, 5, 1, 2, 3, 4, 0, 0],
        [0, 0, 1, 3, 6, 2, 2, 1, 0, 0]
    ];

    let mut field = field::Field::new(raw_field);
    let pack = pack::Pack::new(&[7, 6, 6, 9]);
    let (score, chain_count) = simulate(&mut field, 6, &pack);
    assert_eq!((score, chain_count), (120, 13));


    let simulated_field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 1, 3, 4, 4, 0, 0, 0, 0]
    ];
    let simulated_field = field::Field::new(simulated_field);
    assert_eq!(field, simulated_field);
}

#[test]
fn test_drop_pack() {
    let raw_field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 8, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 8, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 8, 5, 0, 0, 0, 0],
        [0, 0, 0, 3, 6, 7, 0, 7, 0, 0],
        [0, 0, 6, 9, 9, 2, 0, 1, 0, 0],
        [0, 0, 8, 3, 3, 3, 0, 1, 3, 0],
        [0, 4, 1, 1, 8, 5, 3, 1, 6, 0]
    ];
    let mut field = field::Field::new(raw_field);
    let pack = pack::Pack::new(&[0, 9, 1, 2]);
    let modified_blocks = drop_pack(&mut field, 1, &pack);
    assert_eq!(modified_blocks, vec![(3, 2), (1, 1), (4, 2)]);

    let dropped_field = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 8, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 8, 4, 0, 0, 0, 0],
        [0, 0, 9, 0, 8, 5, 0, 0, 0, 0],
        [0, 0, 2, 3, 6, 7, 0, 7, 0, 0],
        [0, 0, 6, 9, 9, 2, 0, 1, 0, 0],
        [0, 1, 8, 3, 3, 3, 0, 1, 3, 0],
        [0, 4, 1, 1, 8, 5, 3, 1, 6, 0]
    ];
    let dropped_field = field::Field::new(dropped_field);
    assert_eq!(field, dropped_field);
}
