use crate::board::EMPTY_BLOCK;
use crate::zobrist_hash_table::ZobristHash;
use crate::xorshift::Xorshift;

pub type Block = u8;
pub type BitBlock = u16;

const BASE_BIT: u16 = 4;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Pack {
    //Board
    //9(0) 5(1)
    //0(2) 3(3)
    //Bit board
    //1001 0101
    //0000 0011
    bit_block: BitBlock,
}

impl Eq for Pack {}

impl Pack {
    pub fn default() -> Pack {
        Pack { bit_block: 0 }
    }
    pub fn new(blocks: &[Block; 4]) -> Pack {
        let mut pack = Pack { bit_block: 0 };
        for i in 0..4 {
            pack.set(i, blocks[i]);
        }
        pack
    }
    pub fn clear(&mut self) {
        self.bit_block = 0;
    }
    pub fn hash(&self) -> ZobristHash {
        let mut rnd = Xorshift::with_seed(self.bit_block as u64);
        rnd.next()
    }
    pub fn get(&self, idx: usize) -> Block {
        debug_assert!(idx < 4);
        let y = (idx / 2) as u16;
        let x = (idx % 2) as u16;
        let shift_bit = BASE_BIT * (2 * y + x);
        //Bit
        //9(1001) 5(0101)
        //0(0000) 3(0011)
        //0011 0000 0101 1001
        let bit: u16 = 0b1111;
        let block = ((self.bit_block >> shift_bit) & bit) as Block;
        block
    }
    pub fn set(&mut self, idx: usize, value: Block) {
        debug_assert!(idx < 4);
        let y = (idx / 2) as u16;
        let x = (idx % 2) as u16;
        let shift_bit = BASE_BIT * (2 * y + x);
        let bit: u16 = 0b1111;
        //clear bit
        let clear_bit = !(bit << shift_bit);
        self.bit_block &= clear_bit;
        //set bit
        let value = value as u16;
        self.bit_block |= (value << shift_bit) as BitBlock;
    }
    pub fn drop(&mut self) {
        if self.get(2) == EMPTY_BLOCK {
            self.set(2, self.get(0));
            self.set(0, EMPTY_BLOCK);
        }
        if self.get(3) == EMPTY_BLOCK {
            self.set(3, self.get(1));
            self.set(1, EMPTY_BLOCK);
        }
    }
    pub fn rotates(&mut self, rotate_count: usize) {
        for _ in 0..rotate_count {
            self.rotate();
        }
    }
    fn rotate(&mut self) {
        let tmp1 = self.get(0);
        let tmp2 = self.get(1);
        self.set(0, self.get(2));
        self.set(1, tmp1);
        let tmp1 = self.get(3);
        self.set(3, tmp2);
        self.set(2, tmp1);
    }
    #[allow(dead_code)]
    pub fn vec(&self) -> Vec<Block> {
        vec![self.get(0), self.get(1), self.get(2), self.get(3)]
    }
}


#[test]
fn test_equal() {
    let p = Pack::new(&[9, 5, 0, 3]);
    debug_assert_eq!(p.vec(), [9, 5, 0, 3]);
    let p = Pack::new(&[8, 5, 5, 0]);
    debug_assert_eq!(p.vec(), [8, 5, 5, 0]);
}

#[test]
fn test_rotate() {
    let mut p = Pack::new(&[9, 5, 0, 3]);
    debug_assert_eq!(p.vec(), [9, 5, 0, 3]);

    p.rotate();
    debug_assert_eq!(p.vec(), [0, 9, 3, 5]);

    p.rotate();
    debug_assert_eq!(p.vec(), [3, 0, 5, 9]);

    p.rotate();
    debug_assert_eq!(p.vec(), [5, 3, 9, 0]);

    p.rotate();
    debug_assert_eq!(p.vec(), [9, 5, 0, 3]);

    let mut p = Pack::new(&[1, 2, 3, 4]);
    debug_assert_eq!(p.vec(), [1, 2, 3, 4]);
    p.rotate();
    debug_assert_eq!(p.vec(), [3, 1, 4, 2]);
    p.rotate();
    debug_assert_eq!(p.vec(), [4, 3, 2, 1]);
    p.rotate();
    debug_assert_eq!(p.vec(), [2, 4, 1, 3]);
    p.rotate();
    debug_assert_eq!(p.vec(), [1, 2, 3, 4]);
}

#[test]
fn drop() {
    let mut p = Pack::new(&[5, 8, 0, 5]);
    p.drop();
    debug_assert_eq!(p.vec(), [0, 8, 5, 5]);
    let mut p = Pack::new(&[3, 8, 5, 0]);
    p.drop();
    debug_assert_eq!(p.vec(), [3, 0, 5, 8]);

    let mut p = Pack::new(&[1, 2, 3, 4]);
    p.drop();
    debug_assert_eq!(p.vec(), [1, 2, 3, 4]);
}