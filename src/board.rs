use std::fmt;
use crate::utils::misc::{
    split_str_chunks,
    set_board,
    reverse_string,
};
use crate::pieces::BoardPiece;

pub struct GameBoard {
    pub pieces: [BoardPiece;4]
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display: String = String::new();
        for piece in self.pieces.iter() {
            display.push_str(&piece.to_string());
            display.push_str("\n");
        }
        write!(f, "{}", display)
    }
}
