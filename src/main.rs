mod board;
use nalgebra::Point2;
mod movegen;

fn main() {
    let mask = movegen::MoveGen::gen_rook_mask();
    println!("{:?}", mask);
}
