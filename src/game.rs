use std::fmt;
use std::cmp;
use crate::find_magic::{ find_all_magic, Magic, get_move };
use crate::utils::misc::{
    split_str_chunks,
    reverse_string,
};
use crate::constants;

// utils
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
    //
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
        _ => panic!("Wrong value of sq: {}. If index in [4, 9, 14, 19] then one of the piece is on the ghost column, else the piece is beyond the limit of the board (>19).", index)
    };
    let column = match index {
        0 | 5 | 10 | 15 => 0,
        1 | 6 | 11 | 16 => 1,
        2 | 7 | 12 | 17 => 2,
        3 | 8 | 13 | 18 => 3,
        _ => panic!("Wrong value of sq: {}. If index in [4, 9, 14, 19] then one of the piece is on the ghost column, else the piece is beyond the limit of the board (>19).", index)
    };
    [row, column]
}

// Player struct used to store bitboard and various operations
#[derive(Clone)]
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
    fn is_piece_on(&self, piece: usize) -> bool {
        self.pieces[piece] != 0
    }
    fn get_piece_on(&self, square: u32) -> usize {
        for i in 0..4 {
            if self.pieces[i] == square {
                return i
            }
        }
        4
    }
    fn get_full_board(&self) -> u32 {
        self.pieces[0] | self.pieces[1] | self.pieces[2] | self.pieces[3]
    }
    fn get_nb_piece_on(&self) -> u32 {
        let mut nb_piece = 0;
        for i in 0..4 {
            if self.pieces[i] != 0 { nb_piece += 1 }
        }
        nb_piece
    }
    pub fn is_terminal(&self) -> bool {
        let full_board = self.get_full_board();
        (full_board & full_board >> 1 & full_board >> 2  & full_board >> 3)  |
        (full_board & full_board >> 5 & full_board >> 10 & full_board >> 15) |
        (full_board & full_board >> 6 & full_board >> 12 & full_board >> 18) |
        (full_board & full_board >> 4 & full_board >> 8  & full_board >> 12) != 0
    }
    fn get_nb_following_piece_2(&self) -> u32 {
        let full_board = self.get_full_board();
        (full_board & full_board >> 1).count_ones() +
        (full_board & full_board >> 5).count_ones() +
        (full_board & full_board >> 6).count_ones() +
        (full_board & full_board >> 4).count_ones()
    }
    fn get_nb_following_piece_3(&self) -> u32 {
        let full_board = self.get_full_board();
        (full_board & full_board >> 1 & full_board >> 2 ).count_ones() +
        (full_board & full_board >> 5 & full_board >> 10).count_ones() +
        (full_board & full_board >> 6 & full_board >> 12).count_ones() +
        (full_board & full_board >> 4 & full_board >> 8 ).count_ones()
    }
    fn get_nb_following_piece(&self, nb_piece: usize) -> u32 {
        match nb_piece {
            2 => self.get_nb_following_piece_2(),
            3 => self.get_nb_following_piece_3(),
            _ => panic!("get_nb_following_piece not implemented for nb_piece > 3")
        }
    }
}


// Game struct used to store the magic bitboard for the rook and the bishop and the players board
pub struct Game {
    iteration: u64,
    pub player1: Player,
    pub player2: Player,
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

fn get_possible_moves_piece(player: &Player, opponent: &Player, piece_index: usize, bishop_magics: &[Magic;16], rook_magics: &[Magic;16]) -> Vec<[u32;3]> {
    let full_board = player.get_full_board() | opponent.get_full_board();
    let mut possible_moves: Vec<[u32;3]> = Vec::new();
    // Case when the piece is not on the board
    if player.pieces[piece_index] == 0 {
        for i in 0..20 {
            if i % 5 == 4 { continue }
            let board = (2 as u32).pow(i);
            if board & full_board == 0 {
                possible_moves.push([piece_index as u32, 0, 1 << i]);
            }
        }
    }
    // Case when the piece is on the board
    else {
        let src = player.pieces[piece_index];
        let coord_src = get_coord_from_index(get_index_from_square(src));
        let board_index = (3 - coord_src[0]) * 4 + coord_src[1];
        let attacks = match piece_index {
            0 => {
                if player.pawn_bottom {
                    let mut temp_attacks = constants::PAWN_ATTACKS_BOTTOM[board_index as usize] & opponent.get_full_board();
                    if coord_src[0] != 3 {
                        temp_attacks |= (src >> 5) & !(player.get_full_board() | opponent.get_full_board())
                    }
                    temp_attacks
                } else {
                    let mut temp_attacks = constants::PAWN_ATTACKS_TOP[board_index as usize] & opponent.get_full_board();
                    if coord_src[0] != 0 {
                        temp_attacks |= (src << 5) & !(player.get_full_board() | opponent.get_full_board())
                    }
                    temp_attacks
                }
            }
            1 => constants::KNIGHT_ATTACKS[board_index as usize] & !(player.get_full_board()),
            2 => get_move(full_board, &bishop_magics[board_index as usize]) & !(player.get_full_board()),
            3 => get_move(full_board, &rook_magics[board_index as usize]) & !(player.get_full_board()),
            _ => panic!("{} is not a valid piece index", piece_index)
        };
        for i in 0..20 {
            if i % 5 == 4 { continue }
            if attacks & (1 << i) != 0 {
                possible_moves.push([piece_index as u32, src, 1 << i])
            }
        }
    }
    possible_moves
}
fn get_possible_moves(player: &Player, opponent: &Player, bishop_magics: &[Magic;16], rook_magics: &[Magic;16]) -> Vec<[u32;3]> {
    let mut possible_moves: Vec<[u32;3]> = Vec::new();
    for i in 0..4 {
        possible_moves.append(&mut get_possible_moves_piece(&player, &opponent, i, &bishop_magics, &rook_magics));
    }
    possible_moves
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
fn is_valid_move(bishop_magics: &[Magic;16], rook_magics: &[Magic;16], player: &Player, opponent: &Player, piece_index: usize, src: u32, dst: u32) -> bool {
    let coord_src = get_coord_from_index(get_index_from_square(src));
    let board_index = (3 - coord_src[0]) * 4 + coord_src[1];
    let full_board = (player.get_full_board() | opponent.get_full_board()) & !(src);
    let valid_attacks = match piece_index {
        0 => {
            if player.pawn_bottom {
                let mut temp_attacks = constants::PAWN_ATTACKS_BOTTOM[board_index as usize] & opponent.get_full_board();
                if coord_src[0] != 3 {
                    temp_attacks |= (src >> 5) & !(player.get_full_board() | opponent.get_full_board())
                }
                temp_attacks
            } else {
                let mut temp_attacks = constants::PAWN_ATTACKS_TOP[board_index as usize] & opponent.get_full_board();
                if coord_src[0] != 0 {
                    temp_attacks |= (src << 5) & !(player.get_full_board() | opponent.get_full_board())
                }
                temp_attacks
            }
        }
        1 => constants::KNIGHT_ATTACKS[board_index as usize] & !(player.get_full_board()),
        2 => get_move(full_board & constants::BISHOP_ATTACKS[board_index as usize], &bishop_magics[board_index as usize]) & !(player.get_full_board()),
        3 => get_move(full_board & constants::ROOK_ATTACKS[board_index as usize], &rook_magics[board_index as usize]) & !(player.get_full_board()),
        _ => panic!("{} is not a valid piece index", piece_index)
    };
    (dst & valid_attacks) != 0
}
pub fn apply_move(player: &mut Player, opponent: &mut Player, move_coord: [u32;3], bishop_magics: &[Magic;16], rook_magics: &[Magic;16]) -> bool {
    let is_piece_on = player.is_piece_on(move_coord[0] as usize);
    if is_piece_on && is_valid_move(bishop_magics, rook_magics, &player, &opponent, move_coord[0] as usize, move_coord[1], move_coord[2]) {
        player.set_piece(move_coord[0] as usize, move_coord[2]);
        let opponent_piece_on_index = opponent.get_piece_on(move_coord[2]);
        if opponent_piece_on_index != 4 {
            opponent.set_piece(opponent_piece_on_index, 0);
        }
        return true
    } else if !is_piece_on && (move_coord[2] & !(player.get_full_board() | opponent.get_full_board())) != 0 {
        player.set_piece(move_coord[0] as usize, move_coord[2]);
        return true
    } else {
        return false
    };
}
pub fn apply_move_algebraic(player: &mut Player, opponent: &mut Player, algebraic_move: &str, bishop_magics: &[Magic;16], rook_magics: &[Magic;16]) -> bool {
    let move_coord = match player.convert_algebraic_to_coord(algebraic_move) {
        Some(coord) => coord,
        None => {
            println!("Invalid syntax/Impossible move");
            return false
        }
    };
    apply_move(player, opponent, move_coord, bishop_magics, rook_magics)
}
// MinMax / NegaMax
fn heuristic(player: &Player, opponent: &Player) -> i32 {
    let mut score = player.get_nb_piece_on() as i32 - opponent.get_nb_piece_on() as i32;
    score += (2 * player.get_nb_following_piece(3) + player.get_nb_following_piece(2)) as i32;
    score -= (2 * opponent.get_nb_following_piece(3) + opponent.get_nb_following_piece(2)) as i32;
    score
}
fn is_terminal(player: &Player, opponent: &Player) -> bool {
    return player.is_terminal() || opponent.is_terminal();
}
fn negamax(depth: u32, player: &mut Player, opponent: &mut Player, alpha: i32, beta: i32, bishop_magics: &[Magic;16], rook_magics: &[Magic;16], color: i32) -> i32 {
    if opponent.is_terminal() {
        return color * -(1001 + depth as i32);
    }
    if depth == 0 {
        return color * heuristic(&player, &opponent);
    }
    let mut value: i32 = -5;
    let mut alpha = alpha;
    for m in get_possible_moves(&player, &opponent, &bishop_magics, &rook_magics) {
        let mut player = player.clone();
        let mut opponent = opponent.clone();
        apply_move(&mut player, &mut opponent, m, &bishop_magics, &rook_magics);
        value = cmp::max(value, -negamax(depth - 1, &mut opponent, &mut player, -beta, -alpha, &bishop_magics, &rook_magics, -color));
        alpha = cmp::max(alpha, value);
        if value >= beta { return -value }
    }
    -value
}
pub fn get_best_move(depth: u32, player: &Player, opponent: &Player, bishop_magics: &[Magic;16], rook_magics: &[Magic;16]) -> [u32;3] {
    let mut best_move: [u32;3] = [4, 0, 0];
    let mut best_value = -10000;
    let alpha = -1001;
    let beta = 1001;
    for m in get_possible_moves(&player, &opponent, bishop_magics, rook_magics) {
        let mut player = player.clone();
        let mut opponent = opponent.clone();
        apply_move(&mut player, &mut opponent, m, &bishop_magics, &rook_magics);
        let value = negamax(depth, &mut opponent, &mut player, alpha, beta, bishop_magics, rook_magics, -1);
        if value > best_value {
            best_value = value;
            best_move = m;
        }
    }
    best_move
}

impl Game {
    // Constructor
    pub fn new() -> Self {
        Game {
            iteration: 0,
            player1: Player::new(0, 0, 0, 0, false),
            player2: Player::new(0, 0, 0, 0, true),
            rook_magics: find_all_magic(true),
            bishop_magics: find_all_magic(false)
        }
    }

    // Display utilies
    pub fn get_display_cli(&self) -> [[char;4];4] {
        let mut display_rows = [['.', '.', '.', '.'],
                                ['.', '.', '.', '.'],
                                ['.', '.', '.', '.'],
                                ['.', '.', '.', '.']];
        fill_char_array_with_piece(&mut display_rows, &self.player1, "white");
        fill_char_array_with_piece(&mut display_rows, &self.player2, "black");
        display_rows
    }
    pub fn display_cli(&self) {
        let display_rows = self.get_display_cli();
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

    // Main game execution
    pub fn run_cli(&mut self) {
        println!("Welcome! This program is an AI playing TicTacChess game.");
        println!("If you don't know the rules you can find them here: https://github.com/charlyalizadeh/TicTacChess#rules-of-tictacchess");
        println!("This engine uses algebraic notation: https://en.wikipedia.org/wiki/Algebraic_notation_(chess)");
        println!("Enjoy!");
        println!("Would like to start ? (yes/no)");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        if choice.trim() == "yes" {
            self.display_cli();
            println!("Please enter the move in algebraic notation:");
            let mut algebraic_move = String::new();
            std::io::stdin().read_line(&mut algebraic_move).unwrap();
            while !apply_move_algebraic(&mut self.player1, &mut self.player2, &algebraic_move, &self.bishop_magics, &self.rook_magics) {
                algebraic_move = String::new();
                self.display_cli();
                println!("Move invalid!");
                println!("Please enter the move in algebraic notation:");
                std::io::stdin().read_line(&mut algebraic_move).unwrap();
            }
            self.iteration += 1;
        }
        loop {
            // player2 plays
            let best_move = get_best_move(4, &self.player2, &self.player1, &self.bishop_magics, &self.rook_magics);
            apply_move(&mut self.player2, &mut self.player1, best_move, &self.bishop_magics, &self.rook_magics);
            if self.player2.is_terminal() {
                self.display_cli();
                println!("You lost!");
                break;
            }

            // player1 plays
            self.display_cli();
            println!("Please enter the move in algebraic notation:");
            let mut algebraic_move = String::new();
            std::io::stdin().read_line(&mut algebraic_move).unwrap();
            while !apply_move_algebraic(&mut self.player1, &mut self.player2, &algebraic_move, &self.bishop_magics, &self.rook_magics) {
                algebraic_move = String::new();
                self.display_cli();
                println!("Move invalid!");
                println!("Please enter the move in algebraic notation:");
                std::io::stdin().read_line(&mut algebraic_move).unwrap();
            }
            if self.player1.is_terminal() {
                self.display_cli();
                println!("You Won!");
                break;
            }
            self.iteration += 1;
        }
    }
}
