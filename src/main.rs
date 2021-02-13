mod board;
use nalgebra::Point2;
mod movegen;

fn main() {
    let mut mask = movegen::MoveGen::gen_bishop_mask();
    println!("{:?}", mask);
}
