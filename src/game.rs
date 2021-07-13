use std::fmt;
use crate::find_magic::{ find_all_magic, Magic, get_move };
use crate::utils::misc::{
    split_str_chunks,
    set_board,
    reverse_string,
};
use crate::constants;

//#[repr(u8)]
//pub enum Piece {
//    Pawn,
//    Knight,
//    Bishop,
//    Rook,
//}
//impl From<u8> for Piece {
//    fn from(integer: u8) -> Self {
//        match integer {
//            0 => Piece::Pawn,
//            1 => Piece::Knight,
//            2 => Piece::Bishop,
//            _ => Piece::Rook,
//        }
//    }
//}
//
fn get_piece_index(piece: &char) -> usize {
    match piece {
        'p' | 'P' => 0,
        'n' | 'N' => 1,
        'b' | 'B' => 2,
        'r' | 'R' => 3,
        _ => 4,
    }
}
fn get_index_from_square(sq: u32) -> u32 {
    (sq as f64).log2() as u32 // here we assume that p only contains one bit so log2(p) will be an integer
}
fn get_square_from_index(index: u32) -> u32 {
    (2 as u32).pow(index)
}
fn get_coord_from_index(index: u32) -> [u32;2] {
    // There must be a way to get the row and column in a smarter way but I feel a little
    // bit dumb today.
    // The tricky part (didn't think about it so mayby not so tricky) is that index will
    // take the following value depending on the 1 position:
    //   15 16 17 18 (19)
    //   10 11 12 13 (14)
    //   5  6  7  8  (9 )
    //   0  1  2  3  (4 )
    //               Ghost
    //               column
    let row = match index {
        0..=3 => 3,
        5..=8 => 2,
        10..=13 => 1,
        15..=18 => 0,
        _ => panic!("Wrong value of sq: {}. If index in [4, 10, 14, 20] then one of the piece is on the ghost column, else the piece is beyond the limit of the board (>19).", index)
    };
    let column = match index {
        0 | 5 | 10 | 15 => 0,
        1 | 6 | 11 | 16 => 1,
        2 | 7 | 12 | 17 => 2,
        3 | 8 | 13 | 18 => 3,
        _ => panic!("Wrong value of sq: {}. If index in [4, 10, 14, 20] then one of the piece is on the ghost column, else the piece is beyond the limit of the board (>19).", index)
    };
    [row, column]
}


pub struct Player {
    pub pieces: [u32;4],
    pub pawn_bottom: bool,
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
    pub fn new(pawn: u32, knight: u32, bishop: u32, rook: u32, pawn_bottom: bool) -> Self {
        Player {
            pieces: [pawn, knight, bishop, rook],
            pawn_bottom,
        }
    }
    fn convert_algebraic_to_coord(&self, string: &str) -> Option<[u32;3]> {
        let char_vec: Vec<char> = string.trim().chars().collect();
        if char_vec.len() > 5 || char_vec[2].to_digit(10) == None || char_vec[4].to_digit(10) == None {
            return None;
        }
        let piece_index = get_piece_index(&char_vec[0]);
        let row_src: u32 = match char_vec[1] {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
           _ => return None
        };
        let col_src: u32 = char_vec[2].to_digit(10).unwrap() - 1;
        let row_dst: u32 = match char_vec[3] {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
           _ => return None
        };
        let col_dst: u32 = char_vec[4].to_digit(10).unwrap() - 1;
        let coord = [piece_index as u32, 1 << (col_src * 5 + row_src), 1 << (col_dst * 5 + row_dst)];
        Some(coord)
    }
    fn set_piece(&mut self, piece_index: usize, piece_square: u32) {
        self.pieces[piece_index] = piece_square;
    }
    fn get_square(&self, square: u32) -> Option<usize> {
        self.pieces.iter().position(|&r| r == square)
    }
    fn is_piece_on(&mut self, piece: usize) -> bool {
        self.pieces[piece] != 0
    }
    fn get_full_board(&self) -> u32 {
        self.pieces[0] | self.pieces[1] | self.pieces[2] | self.pieces[3]
    }
}


fn fill_char_array_with_piece(char_array: &mut [[char;4];4], player: &Player, color: &str) {
        for (i, p) in player.pieces.iter().enumerate() {
            if *p == 0 { continue; }
            let coord = get_coord_from_index(get_index_from_square(*p));
            match i {
                0 => char_array[coord[0] as usize][coord[1] as usize] = if color == "white" { 'P' } else { 'p' },
                1 => char_array[coord[0] as usize][coord[1] as usize] = if color == "white" { 'N' } else { 'n' },
                2 => char_array[coord[0] as usize][coord[1] as usize] = if color == "white" { 'B' } else { 'b' },
                3 => char_array[coord[0] as usize][coord[1] as usize] = if color == "white" { 'R' } else { 'r' },
                _ => panic!("It seems like `player1.pieces` has more than 4 element. No idea how you managed to do that.")
            }
        }
}

pub struct Game {
    iteration: u64,
    player1: Player,
    player2: Player,
    pub rook_magics: [Magic;16],
    pub bishop_magics: [Magic;16],
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


fn is_valid_move(bishop_magics: &[Magic;16], rook_magics: &[Magic;16], player: &Player, opponent: &Player, piece_index: usize, src: u32, dst: u32) -> bool {
    let coord_src = get_coord_from_index(get_index_from_square(src));
    let board_index = (3 - coord_src[0]) * 4 + coord_src[1];
    let full_board = (player.get_full_board() | opponent.get_full_board()) & !(src);
    println!("board_index: {}", board_index);
    let valid_attacks = match piece_index {
        0 => {
            if player.pawn_bottom {
                let mut temp_attacks = constants::PAWN_ATTACKS_BOTTOM[board_index as usize] & opponent.get_full_board();
                if coord_src[0] != 3 {
                    temp_attacks |= (src >> 5) & !(player.get_full_board() & opponent.get_full_board())
                }
                temp_attacks
            } else {
                let mut temp_attacks = constants::PAWN_ATTACKS_TOP[board_index as usize] & opponent.get_full_board();
                if coord_src[0] != 0 {
                    temp_attacks |= (src << 5) & !(player.get_full_board() & opponent.get_full_board())
                }
                temp_attacks
            }
        }
        1 => constants::KNIGHT_ATTACKS[board_index as usize] & !(player.get_full_board()),
        2 => get_move(full_board, &bishop_magics[board_index as usize]) & !(player.get_full_board()),
        3 => get_move(full_board, &rook_magics[board_index as usize]) & !(player.get_full_board()),
        _ => panic!("{} is not a valid piece index", piece_index)
    };
    println!("valid_attacks: {}", valid_attacks);
    (dst & valid_attacks) != 0
}

impl Game {
    pub fn new() -> Self {
        Game {
            iteration: 0,
            player1: Player::new(0, 0, 0, 0, false),
            player2: Player::new(0, 0, 0, 0, true),
            rook_magics: find_all_magic(true),
            bishop_magics: find_all_magic(false)
        }
    }
    pub fn display_cli(&self) {
        let mut display_rows = [['.', '.', '.', '.'],
                            ['.', '.', '.', '.'],
                            ['.', '.', '.', '.'],
                            ['.', '.', '.', '.']];
        fill_char_array_with_piece(&mut display_rows, &self.player1, "white");
        fill_char_array_with_piece(&mut display_rows, &self.player2, "black");
        for (i, row) in display_rows.iter().enumerate() {
            let row_number = match i {
                0 => 4,
                1 => 3,
                2 => 2,
                3 => 1,
                _ => panic!("How tf did you get here ?")
            };
            println!("{} {}", row_number, row.iter().collect::<String>());
        }
        println!("  abcd")
    }
    pub fn run_cli(&mut self) {
        println!("Welcome! This program is an AI playing TicTacChess game.");
        println!("If you don't know the rules you can find them here: https://github.com/charlyalizadeh/TicTacChess#rules-of-tictacchess");
        println!("This engine uses algebraic notation: https://en.wikipedia.org/wiki/Algebraic_notation_(chess)");
        println!("Enjoy!");
        println!("Would like to start ? (yes/no)");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        if choice.trim() == "yes" {
            println!("{}", self.player1);
            self.display_cli();
            println!("Please enter the move in algebraic notation:");
            let mut algebraic_move = String::new();
            std::io::stdin().read_line(&mut algebraic_move).unwrap();
            while !self.apply_move(&algebraic_move, "player1") {
                algebraic_move = String::new();
                println!("{}", self.player1);
                self.display_cli();
                println!("Move invalid!");
                println!("Please enter the move in algebraic notation:");
                std::io::stdin().read_line(&mut algebraic_move).unwrap();
            }
        }
        loop {
            println!("{}", self.player1);
            self.display_cli();
            println!("Please enter the move in algebraic notation:");
            let mut algebraic_move = String::new();
            std::io::stdin().read_line(&mut algebraic_move).unwrap();
            while !self.apply_move(&algebraic_move, "player1") {
                algebraic_move = String::new();
                println!("{}", self.player1);
                self.display_cli();
                println!("Move invalid!");
                println!("Please enter the move in algebraic notation:");
                std::io::stdin().read_line(&mut algebraic_move).unwrap();
            }
        }
    }
    fn apply_move(&mut self, string: &str, round: &str) -> bool {
        let (mut player, mut opponent) = match round {
            "player1" => (&mut self.player1, &mut self.player2),
            "player2" => (&mut self.player2, &mut self.player1),
            _ => panic!("{} is not a valid round", round)
        };
        let move_coord = match player.convert_algebraic_to_coord(string) {
            Some(coord) => coord,
            None => {
                println!("Invalid syntax/Impossible move");
                return false
            }
        };
        let is_piece_on = player.is_piece_on(move_coord[0] as usize);
        if is_piece_on && is_valid_move(&self.bishop_magics, &self.rook_magics, &player, &opponent, move_coord[0] as usize, move_coord[1], move_coord[2]) {
            player.set_piece(move_coord[0] as usize, move_coord[2]);
            return true
        } else if !is_piece_on && (move_coord[2] & !(player.get_full_board() | opponent.get_full_board())) != 0 {
            player.set_piece(move_coord[0] as usize, move_coord[2]);
            return true
        } else {
            return false
        };
    }
}
