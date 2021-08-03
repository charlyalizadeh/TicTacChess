#![allow(dead_code)]

//mod attacks;
mod constants;
mod utils;
mod find_magic;
mod game;
mod gui;

fn main() {
    //let mut g = game::Game::new();
    //g.run_cli();
    let mut board_gui = gui::BoardGUI::new([700, 700]);
    board_gui.run();
}
