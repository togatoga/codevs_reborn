use crate::pack;
use crate::field;
use crate::field::{EMPTY_BLOCK, FIELD_WIDTH};


fn drop_pack(field: &mut field::Field, point: usize, pack: pack::Pack) {
    assert!(0 <= point && point <= 8);
    let mut modified_blocks: Vec<(u8, u8)> = Vec::new();
    for y in (0..2).rev() {
        for x in (0..2).rev() {
            //skip empty block
            let block = pack.blocks[2 * y + x];
            if block == EMPTY_BLOCK {
                continue;
            }
            let nx = point + x;
            assert!(0 <= nx && nx < FIELD_WIDTH);
            let ny = field.bottoms[nx];
            field.field[ny][nx] = block;
            assert_ne!(field.bottoms[nx], 0);
            field.bottoms[nx] -= 1;
        }
    }
}


#[test]
fn test_drop_pack() {
    let mut raw_field = [
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
    let mut field = field::new(raw_field);
    let raw_blocks = vec![0, 9, 1, 2];
    let pack = pack::Pack { blocks: raw_blocks };
    drop_pack(&mut field, 1, pack);

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
    let mut dropped_field = field::new(dropped_field);
    assert_eq!(field, dropped_field);
}
