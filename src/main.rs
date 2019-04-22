use std::io::{StdinLock, Stdin};

const MAX_TURN: usize = 500;
const FIELD_WIDTH: usize = 10;
const FIELD_HEIGHT: usize = 16;


type Field = Vec<Vec<u8>>;
type Block = u8;

#[derive(Debug)]
struct Pack {
    blocks: Vec<Block>,
}
impl Pack {
    pub fn rotate(&mut self)  {
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
    let mut p = Pack {blocks: vec![9, 5, 0, 3]};
    p.rotate();
    assert_eq!(p.blocks, vec![0, 9, 3, 5]);
    p.rotate();
    assert_eq!(p.blocks, vec![3, 0, 5, 9]);
    p.rotate();
    assert_eq!(p.blocks, vec![5, 3, 9, 0]);
    p.rotate();
    assert_eq!(p.blocks, vec![9, 5, 0, 3]);
}

#[derive(Debug)]
struct GameStatus {
    rest_time_milliseconds: u32,
    dead_block_count: u32,
    skill_point: u32,
    game_score: u32,
    field: Field,
}

#[derive(Debug)]
struct Solver<'a> {
    packs: &'a Vec<Pack>,
    player: GameStatus,
    enemy: GameStatus,
}

impl<'a> Solver<'a> {
    fn new(packs: &'a Vec<Pack>, player: GameStatus, enemy: GameStatus) -> Solver {
        Solver { packs, player, enemy }
    }
    fn read_packs(sc: &mut Scanner<StdinLock>) -> Vec<Pack> {
        (0..MAX_TURN).map(|_| {
            let mut blocks = vec![0; 4];
            for i in 0..4 {
                blocks[i] = sc.read::<u8>();
            }
            let end: String = sc.read();
            assert_eq!(end, "END");
            Pack { blocks }
        }).collect::<Vec<Pack>>()
    }

    fn read_game_status(sc: &mut Scanner<StdinLock>) -> GameStatus {
        //read player data
        let rest_time_milliseconds: u32 = sc.read();
        let dead_block_count: u32 = sc.read();
        let skill_point: u32 = sc.read();
        let game_score: u32 = sc.read();
        let field: Field = (0..FIELD_HEIGHT).map(|_| {
            sc.vec::<u8>(FIELD_WIDTH)
        }).collect();
        let end: String = sc.read();
        assert_eq!(end, "END");
        GameStatus { rest_time_milliseconds, dead_block_count, skill_point, game_score, field }
    }
}

fn solve() {
    let s = std::io::stdin();
    let mut sc = Scanner { stdin: s.lock() };
    println!("togatogAI");
    //parse packn
    let packs: Vec<Pack> = Solver::read_packs(&mut sc);
    loop {
        let current_turn: usize = sc.read();
        //read player data
        let player = Solver::read_game_status(&mut sc);
        let enemy = Solver::read_game_status(&mut sc);
        let mut solver = Solver::new(&packs, player, enemy);
        println!("0 0");
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024) // 64MB
        .spawn(|| solve())
        .unwrap()
        .join()
        .unwrap();
}

//snippet from kenkoooo
pub struct Scanner<R> {
    stdin: R,
}

impl<R: std::io::Read> Scanner<R> {
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self.stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_line(&mut self) -> String {
        use std::io::Read;
        let buf = self.stdin
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b'\n' || b == b'\r')
            .take_while(|&b| b != b'\n' && b != b'\r')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
