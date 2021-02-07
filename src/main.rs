mod board;
use nalgebra::Point2;

fn main() {
    let mut game_board = board::GameBoard::new();
    println!("{}", game_board);
    game_board.set_piece(board::Piece::Rook, Point2::new(0, 1));
    println!("{}", game_board);
}
