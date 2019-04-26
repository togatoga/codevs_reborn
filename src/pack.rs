pub type Block = u8;

#[derive(Clone, Debug)]
pub struct Pack {
    pub blocks: Vec<Block>,
}

impl Pack {
    pub fn rotates(&mut self, rotate_count: usize) {
        for _ in 0..rotate_count {
            self.rotate();
        }
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
fn test_rotate() {
    let mut p = Pack { blocks: vec![9, 5, 0, 3] };
    p.rotate();
    assert_eq!(p.blocks, vec![0, 9, 3, 5]);
    p.rotate();
    assert_eq!(p.blocks, vec![3, 0, 5, 9]);
    p.rotate();
    assert_eq!(p.blocks, vec![5, 3, 9, 0]);
    p.rotate();
    assert_eq!(p.blocks, vec![9, 5, 0, 3]);
}
