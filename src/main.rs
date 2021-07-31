#![allow(dead_code)]

//mod attacks;
mod constants;
mod utils;
mod find_magic;
mod game;

fn main() {
    let mut g = game::Game::new();
    g.run_cli();
}
