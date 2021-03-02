use std::fmt;
use crate::utils::misc::{
    split_str_chunks,
    set_board,
    reverse_string,
};
use crate::find_magic::magic::{ find_all_magic, Magic };

#[repr(u8)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
}
impl From<u8> for Piece {
    fn from(integer: u8) -> Self {
        match integer {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            _ => Piece::Rook,
        }
    }
}


pub struct Player {
    pub pieces: [u32;4]
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display: String = "PAWN  KNGHT BISHP ROOK\n".to_owned();
        let mut bits_rep: Vec<Vec<String>> = Vec::new();
        let mut reverse_bits: Vec<String> = Vec::new();
        for i in 0..4 {
            let bits = &format!("{:020b}", self.pieces[i]);
            reverse_bits.push(reverse_string(&bits));
            bits_rep.push(split_str_chunks(&reverse_bits[i], 5));
        }
        for i in (0..4).rev() {
            display.push_str(&bits_rep[0][i]);
            display.push_str(" ");
            display.push_str(&bits_rep[1][i]);
            display.push_str(" ");
            display.push_str(&bits_rep[2][i]);
            display.push_str(" ");
            display.push_str(&bits_rep[3][i]);
            display.push_str("\n");
        }
        write!(f, "{}", display)
    }
}
impl Player {
    pub fn new(pawn: u32, knight: u32, bishop: u32, rook: u32) -> Self {
        Player {
            pieces: [pawn, knight, bishop, rook]
        }
    }
}


pub struct Game {
    player1: Player,
    player2: Player,
    rook_magics: [Magic;16],
    bishop_magics: [Magic;16],
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = "=======================\n".to_owned() + 
                      &self.player1.to_string() +
                      "\n" +
                      &self.player2.to_string() + 
                      "=======================";
        write!(f, "{}", display)
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            player1: Player::new(0, 0, 0, 0),
            player2: Player::new(0, 0, 0, 0),
            rook_magics: find_all_magic(true),
            bishop_magics: find_all_magic(false)
        }
    }
}
