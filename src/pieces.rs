use std::fmt;
use crate::utils::misc::{
    split_str_chunks,
    reverse_string
};

#[derive(Debug)]
pub enum IndexPiece {
    Rook(usize),
    Bishop(usize)
}

#[derive(Debug)]
pub enum BoardPiece {
    Pawn(u32),
    Knight(u32),
    Bishop(u32),
    Rook(u32)
}

impl fmt::Display for BoardPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = match *self {
            BoardPiece::Pawn(_) => "PAWN\n",
            BoardPiece::Knight(_) => "KNIGHT\n",
            BoardPiece::Bishop(_) => "BISHP\n",
            BoardPiece::Rook(_) => "ROOK\n",
        };
        let mut display = display.to_owned();
        let bits: String = match *self { 
            BoardPiece::Pawn(sq) |
            BoardPiece::Knight(sq) |
            BoardPiece::Bishop(sq) |
            BoardPiece::Rook(sq) => format!("{:020b}", sq).to_owned()
        };
        let bits = split_str_chunks(&bits, 5);
        for i in 0..4 {
            display.push_str(&reverse_string(bits[i]));
            display.push_str("\n");
        }
        write!(f, "{}", display)
    }
}
