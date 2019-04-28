use crate::board::{FIELD_HEIGHT, INPUT_FIELD_HEIGHT, FIELD_WIDTH};
use crate::zobrist_hash_table::{ZOBRIST_HASH_TABLE_BOARD, ZobristHash};
//8 * 5 = 40
pub const BIT_FIELD_WIDTH: usize = 5;
const BASE_BIT: u8 = 4;


#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct BitBoard {
    bits: [[u8; BIT_FIELD_WIDTH]; FIELD_HEIGHT],
    zobrist_hash: ZobristHash,
}

impl PartialEq for BitBoard {
    fn eq(&self, other: &BitBoard) -> bool {
         self.bits == other.bits && self.zobrist_hash == other.zobrist_hash
    }
}

impl BitBoard {
    pub fn new(input_board: [[u8; FIELD_WIDTH]; INPUT_FIELD_HEIGHT]) -> BitBoard {
        let mut bit_board = BitBoard { bits: [[0; BIT_FIELD_WIDTH]; FIELD_HEIGHT], zobrist_hash: 0 };
        for y in 0..INPUT_FIELD_HEIGHT {
            for x in 0..FIELD_WIDTH {
                bit_board.set(y, x, input_board[INPUT_FIELD_HEIGHT - 1 - y][x]);
            }
        }
        bit_board
    }
    #[inline]
    pub fn set(&mut self, y: usize, x: usize, value: u8) {
        assert!(value <= 11);
        let x_idx = x / 2;
        //clear  hash
        self.zobrist_hash ^= ZOBRIST_HASH_TABLE_BOARD[y][x_idx][self.bits[y][x_idx] as usize];
        //clear bit
        //5(0101) 9(1001)
        //0101 1001
        if x % 2 == 0 {
            self.bits[y][x_idx] &= 0b00001111;
            self.bits[y][x_idx] |= value << BASE_BIT;
        } else {
            self.bits[y][x_idx] &= 0b11110000;
            self.bits[y][x_idx] |= value;
        }
        //set hash
        self.zobrist_hash ^= ZOBRIST_HASH_TABLE_BOARD[y][x_idx][self.bits[y][x_idx] as usize];

    }
    #[inline]
    pub fn get(&self, y: usize, x: usize) -> u8 {
        let x_idx = x / 2;
        let bit = self.bits[y][x_idx];
        //clear bit
        //5(0101) 9(1001)
        //0101 1001
        if x % 2 == 0 {
            //0101
            bit >> BASE_BIT
        } else {
            //1001
            bit & 0b00001111
        }
    }
    #[inline]
    pub fn zobrist_hash(&self) -> ZobristHash {
        self.zobrist_hash
    }
}

#[test]
fn test_zobrist_hash() {
    let input_board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 11, 0, 0, 2, 0],
        [0, 0, 0, 0, 11, 11, 11, 11, 5, 0],
        [0, 0, 0, 0, 11, 11, 11, 11, 2, 0],
        [0, 0, 0, 11, 11, 1, 4, 4, 5, 11],
        [0, 7, 0, 4, 2, 3, 11, 11, 4, 11],
        [0, 5, 11, 2, 1, 5, 1, 5, 8, 11],
        [8, 11, 3, 11, 6, 7, 4, 7, 9, 3],
        [8, 11, 11, 11, 5, 2, 4, 7, 6, 5],
        [11, 11, 11, 7, 4, 4, 9, 9, 2, 1],
        [11, 3, 2, 11, 8, 11, 3, 4, 3, 11],
        [11, 2, 11, 11, 11, 9, 11, 4, 11, 11],
        [11, 11, 11, 1, 11, 11, 7, 11, 11, 9],
        [11, 11, 5, 7, 1, 7, 11, 11, 6, 8]
    ];
    let mut bit_board = BitBoard::new(input_board);
    bit_board.set(0, 0, 9);
    bit_board.set(0, 1, 8);
    bit_board.set(0, 2, 7);
    bit_board.set(10, 9, 5);

    let board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 11, 0, 0, 2, 0],
        [0, 0, 0, 0, 11, 11, 11, 11, 5, 0],
        [0, 0, 0, 0, 11, 11, 11, 11, 2, 5],
        [0, 0, 0, 11, 11, 1, 4, 4, 5, 11],
        [0, 7, 0, 4, 2, 3, 11, 11, 4, 11],
        [0, 5, 11, 2, 1, 5, 1, 5, 8, 11],
        [8, 11, 3, 11, 6, 7, 4, 7, 9, 3],
        [8, 11, 11, 11, 5, 2, 4, 7, 6, 5],
        [11, 11, 11, 7, 4, 4, 9, 9, 2, 1],
        [11, 3, 2, 11, 8, 11, 3, 4, 3, 11],
        [11, 2, 11, 11, 11, 9, 11, 4, 11, 11],
        [11, 11, 11, 1, 11, 11, 7, 11, 11, 9],
        [9, 8, 7, 7, 1, 7, 11, 11, 6, 8]
    ];
    let board = BitBoard::new(board);
    assert_eq!(bit_board.bits, board.bits);
    assert_eq!(bit_board.zobrist_hash, board.zobrist_hash);


}

#[test]
fn test_bit_board() {
    let input_board = [
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 11, 0],
        [0, 0, 0, 0, 0, 11, 0, 0, 2, 0],
        [0, 0, 0, 0, 11, 11, 11, 11, 5, 0],
        [0, 0, 0, 0, 11, 11, 11, 11, 2, 0],
        [0, 0, 0, 11, 11, 1, 4, 4, 5, 11],
        [0, 7, 0, 4, 2, 3, 11, 11, 4, 11],
        [0, 5, 11, 2, 1, 5, 1, 5, 8, 11],
        [8, 11, 3, 11, 6, 7, 4, 7, 9, 3],
        [8, 11, 11, 11, 5, 2, 4, 7, 6, 5],
        [11, 11, 11, 7, 4, 4, 9, 9, 2, 1],
        [11, 3, 2, 11, 8, 11, 3, 4, 3, 11],
        [11, 2, 11, 11, 11, 9, 11, 4, 11, 11],
        [11, 11, 11, 1, 11, 11, 7, 11, 11, 9],
        [11, 11, 5, 7, 1, 7, 11, 11, 6, 8]
    ];
    let bit_board = BitBoard::new(input_board);
    for y in 0..INPUT_FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            assert_eq!(bit_board.get(y, x), input_board[INPUT_FIELD_HEIGHT - 1 - y][x]);
        }
    }
}