use crate::field::EMPTY_BLOCK;

pub type Block = u8;
pub type BitBlock = u16;

const BASE_BIT: u16 = 4;
#[derive(Clone, Debug, PartialEq)]
pub struct Pack {
    //Board
    //9(0) 5(1)
    //0(2) 3(3)
    //Bit board
    //1001 0101
    //0000 0011
    bit_block: BitBlock,
}

impl Pack {
    pub fn new(blocks: &[Block; 4]) -> Pack {
        let mut pack = Pack {bit_block: 0};
        for i in 0..4 {
            pack.set(i, blocks[i]);
        }
        pack
    }
    pub fn get(&self, idx: usize) -> Block {
        assert!(idx < 4);
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
        assert!(idx < 4);
        let y = (idx / 2) as u16;
        let x = (idx % 2) as u16;
        let shift_bit = (BASE_BIT * (2 * y + x));
        let bit: u16 = 0b1111;
        //clear bit
        let clear_bit = !(bit << shift_bit);
        self.bit_block &= clear_bit;
        //set bit
        let value = value as u16;
        self.bit_block |= (value << shift_bit) as BitBlock;

    }
    fn drop(&mut self) {
        if self.get(2) == EMPTY_BLOCK {
            self.set(2, self.get(0));
        }
        if self.get(3) == EMPTY_BLOCK {
            self.set(3, self.get(1));
        }
    }
    pub fn rotates(&mut self, rotate_count: usize) {
        for _ in 0..rotate_count {
            self.rotate();
        }
        self.drop();
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
}


#[test]
fn test_equal() {
    let mut p = Pack::new(&[9, 5, 0, 3]);
    assert_eq!([p.get(0), p.get(1), p.get(2), p.get(3)], [9, 5, 0, 3]);

}
#[test]
fn test_rotate() {
    let mut p = Pack::new(&[9, 5, 0, 3]);

    p.rotate();
    //0 9 3 5
    assert_eq!([p.get(0), p.get(1), p.get(2), p.get(3)], [0, 9, 3, 5]);
    p.rotate();
    assert_eq!([p.get(0), p.get(1), p.get(2), p.get(3)], [3, 0, 5, 9]);
    p.rotate();
    assert_eq!([p.get(0), p.get(1), p.get(2), p.get(3)], [5, 3, 9, 0]);
    p.rotate();
    assert_eq!([p.get(0), p.get(1), p.get(2), p.get(3)], [9, 5, 0, 3]);
}
