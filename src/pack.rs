use crate::field::EMPTY_BLOCK;

pub type Block = u8;

#[derive(Clone, Debug, PartialEq)]
pub struct Pack {
    pub blocks: [Block; 4],
}

impl Pack {
    pub fn new(blocks: &[Block; 4]) -> Pack {
        let mut pack = Pack {blocks: *blocks};
        pack
    }
    fn drop(&mut self) {
        if self.blocks[2] == EMPTY_BLOCK {
            self.blocks[2] = self.blocks[0];
        }
        if self.blocks[3] == EMPTY_BLOCK {
            self.blocks[3] = self.blocks[1];
        }
    }
    pub fn rotates(&mut self, rotate_count: usize) {
        for _ in 0..rotate_count {
            self.rotate();
        }
        self.drop();
    }
    fn rotate(&mut self) {
        let tmp1 = self.blocks[0];
        let tmp2 = self.blocks[1];
        self.blocks[0] = self.blocks[2];
        self.blocks[1] = tmp1;
        let tmp1 = self.blocks[3];
        self.blocks[3] = tmp2;
        self.blocks[2] = tmp1;
    }
}


#[test]
fn test_equal() {

}
#[test]
fn test_rotate() {
    let mut p = Pack { blocks: [9, 5, 0, 3] };
    p.rotate();
    assert_eq!(p.blocks, [0, 9, 3, 5]);
    p.rotate();
    assert_eq!(p.blocks, [3, 0, 5, 9]);
    p.rotate();
    assert_eq!(p.blocks, [5, 3, 9, 0]);
    p.rotate();
    assert_eq!(p.blocks, [9, 5, 0, 3]);
}
