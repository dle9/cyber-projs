// use player module from the main mod's
use crate::util;

pub mod dice; use dice::Dice;
pub mod impulse; use impulse::Impulse;
pub mod tetris; use tetris::Tetris;

#[derive(Debug)]
pub struct Games {
    dice: Dice,
    impulse: Impulse,
    tetris: Tetris,
    valid: util::GameOptions,
}

impl Games {
    pub fn new() -> Self {
        Self {
            dice: Dice::new(),
            impulse: Impulse::new(),
            tetris: Tetris::new(),

            // define valid options to be used 
            // with PLAY command
            valid: util::GameOptions {
                game_options: vec!["dice", "impulse", "tetris"],
            }
        }
    }

    pub fn handle_play(&mut self, game: &str) {
        if !self.valid.game_options.contains(&game) {
            println!("\nInvalid game, '{}' \nChoose from: {:?}", game, self.valid.game_options);
            return;
        }

        match game {
            "dice" => self.run_game(game),
            "impulse" => self.run_game(game),
            "tetris" => self.run_game(game),

            _ => ()
        }
    }

    fn run_game(&mut self, game: &str) {
        if game == "dice" { Dice::run(&mut self.dice); }
        if game == "impulse" { Impulse::run(&mut self.impulse); }
        if game == "tetris" { Tetris::run(&mut self.tetris); }
    }
}