use std::fmt;
#[path = "utils/misc.rs"] pub mod misc;
use misc::{
    split_str_chunks,
    set_board,
    reverse_string,
};
use nalgebra::Point2;


pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook
}

pub struct GameBoard {
    pawn: u32,
    knight: u32,
    bishop: u32,
    rook: u32
}

impl GameBoard {
    pub fn new() -> Self {
        GameBoard {
            pawn: 0,
            knight: 0,
            bishop: 0,
            rook: 0
        }
    }

    pub fn set_piece(&mut self, piece: Piece, coord: Point2<u8>) {
        match piece {
            Piece::Pawn => set_board(&mut self.pawn, coord),
            Piece::Knight => set_board(&mut self.knight, coord),
            Piece::Bishop => set_board(&mut self.bishop, coord),
            Piece::Rook => set_board(&mut self.rook, coord)
        }
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Clean this function
        // I understand lifetime but I wonder what other solution I have than assigning the value
        // of the format to a variable
        let pawn_str = &format!("{:020b}", self.pawn);
        let knight_str = &format!("{:020b}", self.knight);
        let bishop_str = &format!("{:020b}", self.bishop);
        let rook_str = &format!("{:020b}", self.rook);
        let pawn_str_split = split_str_chunks(&pawn_str, 5);
        let knight_str_split = split_str_chunks(&knight_str, 5);
        let bishop_str_split = split_str_chunks(&bishop_str, 5);
        let rook_str_split = split_str_chunks(&rook_str, 5);
        let mut display: String = "PAWN  KNIGHT BISHOP ROOK\n".to_owned();
        for i in 0..4 {
            display.push_str(&reverse_string(pawn_str_split[i]));
            display.push_str(" ");
            display.push_str(&reverse_string(knight_str_split[i]));
            display.push_str("  ");
            display.push_str(&reverse_string(bishop_str_split[i]));
            display.push_str("  ");
            display.push_str(&reverse_string(rook_str_split[i]));
            display.push_str("\n");
        }
        write!(f, "{}", display)
    }
}
