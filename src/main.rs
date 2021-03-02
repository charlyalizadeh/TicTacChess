#![allow(dead_code)]

mod attacks;
mod constants;
mod utils;
mod game;
pub mod find_magic;

fn main() {
    let pawn_masks = utils::mask::gen_pawn_attacks_bottom();
    println!("{:?}", pawn_masks);
}

