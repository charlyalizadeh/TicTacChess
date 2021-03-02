use std::fmt;
use std::convert::TryInto;
use rand::Rng;
use crate::find_magic::blockers;
use crate::find_magic::pieces::IndexPiece;
use crate::attacks;

#[derive(Debug)]
pub struct Magic {
    pub magic: u32,
    pub shift: u32,
    pub database: Vec<u32>
}
impl Magic {
    pub fn new() -> Self {
        Magic {
            magic: 0,
            shift: 0,
            database: Vec::new()
        }
    }
}
impl fmt::Display for Magic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "magic: {}\nshift: {}\ndatabase: {:?}", self.magic, self.shift, self.database)
    }
}
impl Clone for Magic {
    fn clone(&self) -> Self {
        Magic {
            magic: self.magic,
            shift: self.shift,
            database: self.database.clone()
        }
    }
}


pub fn find_magic(piece: &IndexPiece) -> Magic {
    let blocker = blockers::gen_blocker_boards(&piece);
    let bits = (blocker.len() as f64).log2() as u32 + 1;
    let mut database = vec![0;usize::pow(2, bits)];
    let mut rng = rand::thread_rng();
    let mut magic = rng.gen::<u32>();
    loop {
        let mut failed = false;
        for board in blocker.iter() {
            let move_board = attacks::get_attacks(*board, &piece);
            let index: u32 = board.wrapping_mul(magic) >> (32 - bits);
            if database[index as usize] != 0 && database[index as usize] != move_board {
                failed = false;
            }
            else {
                database[index as usize] = move_board;
            }
        }
        if !failed { break; }
        magic = rng.gen::<u32>();
        database = vec![0;usize::pow(2, bits)];
    }
    Magic { magic, database, shift: bits }
}
pub fn find_all_magic(rook: bool) -> [Magic;16] {
    let mut magics = vec![Magic::new();16];
    for i in 0..16 {
        let piece = if rook { IndexPiece::Rook(i) } else { IndexPiece::Bishop(i) };
        magics[i] = find_magic(&piece);
    }
    magics
        .try_into()
        .unwrap_or_else(|v: Vec<Magic>| panic!("Expected a Vec of length 16 but it was {}", v.len()))
}

pub fn get_move(sq: u32, magic: &Magic) -> u32 {
    magic.database[(sq.wrapping_mul(magic.magic) >> magic.shift) as usize]
}
