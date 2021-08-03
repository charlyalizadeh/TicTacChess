use std::time::Duration;
use std::thread;
use rand::{thread_rng, Rng};
use sfml::{
    audio::{Sound, SoundBuffer},
    graphics::{
        Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
        Transformable, Image, Sprite, Texture
    },
    system::{Clock, Time, Vector2f},
    window::{ContextSettings, Event, Key, Style, mouse::Button},
};
use std::{
    env, f32::consts::PI, fs::File,
    collections::HashMap,
    ops::DerefMut,
    ops::Deref
};
use crate::game::{ Game,
                   apply_move_algebraic, apply_move,
                   get_best_move
};

pub struct BoardGUI {
    dim: [u32;2],
}

impl BoardGUI {
    pub fn new(dim: [u32;2]) -> Self {
        BoardGUI {
            dim: dim,
        }
    }
    pub fn run(&self) {
        // Create the game
        let mut game = Game::new();
        let mut round = 1;

        // Create the window
        let mut context_settings = ContextSettings::default();
        let mut window = RenderWindow::new(
            (self.dim[0], self.dim[1]),
            "SFML TicTacChess",
            Style::CLOSE,
            &context_settings,
        );


        // Create the border of the board
        let square_size = Vector2f::new(((self.dim[0] - 18) / 4) as f32, ((self.dim[1] - 60) / 4) as f32);
        println!("{:?}", square_size);
        // TODO: dont be lazy and find the correct way to do this
        let mut squares: [RectangleShape;16] = [
            RectangleShape::new(), RectangleShape::new(), RectangleShape::new(), RectangleShape::new(),
            RectangleShape::new(), RectangleShape::new(), RectangleShape::new(), RectangleShape::new(),
            RectangleShape::new(), RectangleShape::new(), RectangleShape::new(), RectangleShape::new(),
            RectangleShape::new(), RectangleShape::new(), RectangleShape::new(), RectangleShape::new()
        ];
        let mut color_index: usize = 0;
        let colors = [Color::rgb(154, 99, 43), Color::rgb(205, 165, 96)];
        for i in 0..4 {
            for j in 0..4 {
                println!("{:?}", square_size);
                squares[i * 4 + j].set_size(square_size);
                squares[i * 4 + j].set_outline_thickness(3.);
                squares[i * 4 + j].set_outline_color(Color::BLACK);
                squares[i * 4 + j].set_fill_color(colors[color_index]);
                if color_index == 1 { color_index -= 1 } else { color_index += 1 };
                squares[i * 4 + j].set_origin(Vector2f::new(-(i as f32 * square_size.x) - 15., -(j as f32 * square_size.y) - 15.));
            }
            if color_index == 1 { color_index -= 1 } else { color_index += 1 };
        }

        // Row/Column label
        let font = Font::from_file("fonts/InputMono-Black.ttf").unwrap();
        let mut column_label = [
            Text::new(&String::from("a"), &font, 15),
            Text::new(&String::from("b"), &font, 15),
            Text::new(&String::from("c"), &font, 15),
            Text::new(&String::from("d"), &font, 15),
        ];
        let mut row_label = [
            Text::new(&String::from("4"), &font, 15),
            Text::new(&String::from("3"), &font, 15),
            Text::new(&String::from("2"), &font, 15),
            Text::new(&String::from("1"), &font, 15),
        ];
        for i in 0..4 {
            column_label[i].set_origin(Vector2f::new(-(i as f32 * square_size.x + square_size.x / 2.) - 15., 5.));
            row_label[i].set_origin(Vector2f::new(0., -(i as f32 * square_size.y + square_size.y / 2.) - 15.));
            column_label[i].set_fill_color(Color::BLACK);
            row_label[i].set_fill_color(Color::BLACK);
            column_label[i].set_outline_color(Color::BLACK);
            row_label[i].set_outline_color(Color::BLACK);
        }


        // Text input
        let mut string = String::from("Enter your move here.");
        let mut text = Text::new(&string, &font, 15);
        text.set_fill_color(Color::rgb(130, 130, 130));
        text.set_outline_color(Color::rgb(130, 130, 130));
        //text.set_outline_thickness(2.);
        text.set_origin(Vector2f::new(- (self.dim[0] as f32 / 2. - string.len() as f32 * 4.), - (self.dim[1] as f32 - 30.)));

        // Pieces
        let mut textures = Vec::new();
        let mut pieces: HashMap<&str, Sprite> = HashMap::new();
        let piece_names = ["white_pawn", "white_knight", "white_bishop", "white_rook",
                           "black_pawn", "black_knight", "black_bishop", "black_rook"];
        for piece_name in piece_names.iter() {
            textures.push(Texture::from_file(&format!("./static/pieces_sprite/{}.png", piece_name)).unwrap());
        }
        for (i, piece_name) in piece_names.iter().enumerate() {
            pieces.insert(piece_name, Sprite::with_texture(&textures[i]));
        }
        let best_move = get_best_move(4, &game.player2, &game.player1, &game.bishop_magics, &game.rook_magics);
        apply_move(&mut game.player2, &mut game.player1, best_move, &game.bishop_magics, &game.rook_magics);

        // Main loop
        loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => window.close(),
                    Event::TextEntered { unicode } => {
                        if string == "Enter your move here." || string == "Move invalid!" {
                            string = String::from("");
                        }
                        if unicode == 0x08 as char {
                            if string == "" {
                                string = String::from("Enter your move here.")
                            }
                            else {
                                string.pop();
                            }
                        }
                        else if unicode == 0xD as char {
                            if !apply_move_algebraic(&mut game.player1, &mut game.player2, &string, &game.bishop_magics, &game.rook_magics) {
                                string = String::from("Move invalid!");
                            }
                            else {
                                let best_move = get_best_move(4, &game.player2, &game.player1, &game.bishop_magics, &game.rook_magics);
                                apply_move(&mut game.player2, &mut game.player1, best_move, &game.bishop_magics, &game.rook_magics);
                                string = String::from("Enter your move here.")
                            }
                        }
                        else if unicode != 0x16 as char && unicode != 0x03 as char {
                            let is_valid = match string.len() {
                                0 => {
                                    ['p', 'b', 'n', 'r'].contains(&unicode)
                                },
                                1 | 3 => {
                                    ['a', 'A', 'b', 'B', 'c', 'C', 'd', 'D'].contains(&unicode)
                                },
                                2 | 4 => {

                                    ['1', '2', '3', '4'].contains(&unicode)
                                },
                                _ => false,
                            };
                            if is_valid {
                                string.push(unicode);
                            }
                        }
                        if string == "" {
                            string = String::from("Enter your move here.")
                        }
                        text.set_string(&string);
                        text.set_origin(Vector2f::new(- (self.dim[0] as f32 / 2. - string.len() as f32 * 4.), - (self.dim[1] as f32 - 30.)));
                    }
                    _ => {
                    }
                }
            }

            // PLAYING AGAINST ITSELF
            //if round == 1 {
            //    let best_move = get_best_move(4, &game.player2, &game.player1, &game.bishop_magics, &game.rook_magics);
            //    apply_move(&mut game.player2, &mut game.player1, best_move, &game.bishop_magics, &game.rook_magics);
            //    round = 0;
            //}
            //else {
            //    let best_move = get_best_move(4, &game.player1, &game.player2, &game.bishop_magics, &game.rook_magics);
            //    apply_move(&mut game.player1, &mut game.player2, best_move, &game.bishop_magics, &game.rook_magics);
            //    round = 1;
            //}
            //
            window.clear(Color::WHITE);
            for i in 0..16 {
                window.draw(&squares[i]);
            }
            for i in 0..4 {
                window.draw(&column_label[i]);
                window.draw(&row_label[i]);
            }
            window.draw(&text);
            let display_cli = game.get_display_cli();
            for (i, row) in display_cli.iter().enumerate() {
                for (j, square) in row.iter().enumerate() {
                    let piece: Option<&str> = match square {
                        'p' => Some("black_pawn"),
                        'P' => Some("white_pawn"),
                        'n' => Some("black_knight"),
                        'N' => Some("white_knight"),
                        'b' => Some("black_bishop"),
                        'B' => Some("white_bishop"),
                        'r' => Some("black_rook"),
                        'R' => Some("white_rook"),
                        _ => None,
                    };
                    match piece {
                        None => {},
                        Some(piece_name) => {
                            let x = -(j as f32 * square_size.x) - 15. - square_size.x / 2. + pieces[piece_name].texture().unwrap().size().x as f32 / 2.;
                            let y = -(i as f32 * square_size.y) - 15. - square_size.y / 2. + pieces[piece_name].texture().unwrap().size().y as f32 / 2.;
                            let origin = Vector2f::new(x, y);
                            pieces.get_mut(piece_name).unwrap().set_origin(origin);
                            window.draw(&pieces[piece_name]);
                        }
                    };
                }
            }
            window.display();
            //thread::sleep(Duration::from_millis(1000)) 
        }
    }
}
