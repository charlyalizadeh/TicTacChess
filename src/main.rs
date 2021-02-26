mod attacks;
mod pieces;
mod constants;
mod utils;
mod board;
pub mod find_magic;
use pieces::BoardPiece;

fn main() {
    let game_board = board::GameBoard {
        pieces: [
            BoardPiece::Pawn(0),
            BoardPiece::Knight(0),
            BoardPiece::Bishop(0),
            BoardPiece::Rook(0)
        ]
    };
    println!("{}", game_board);
}

