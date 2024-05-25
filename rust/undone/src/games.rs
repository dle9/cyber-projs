// use player module from the main mod's
use crate::util;

pub mod dice; use dice::Dice;
pub mod impulse; use impulse::Impulse;
pub mod reaction; use reaction::Reaction;
pub mod tetris; use tetris::Tetris;

#[derive(Debug)]
pub struct Games {
    dice: Dice,
    impulse: Impulse,
    reaction: Reaction,
    tetris: Tetris,
    last_game: String,
    valid: util::GameOptions,
}

impl Games {
    pub fn new() -> Self {
        Self {
            dice: Dice::new(),
            impulse: Impulse::new(),
            reaction: Reaction::new(),
            tetris: Tetris::new(),
            last_game: String::new(),

            // define valid options to be used 
            // with PLAY command
            valid: util::GameOptions {
                game_options: vec!["dice", "reaction", "impulse", "tetris"],
            },
            
        }
    }

    pub fn handle_play(&mut self, game: &str) {
        if !self.valid.game_options.contains(&game) {
            println!("\nInvalid game, '{}' \nChoose from: {:?}", game, self.valid.game_options);
            return;
        }

        match game {
            "dice" => {
                self.run_game(game);
                self.last_game = "dice".to_string();
            },
            "reaction" => {
                self.run_game(game);
                self.last_game = "reaction".to_string();
            },
            "impulse" => {
                self.run_game(game);
                self.last_game = "impulse".to_string();
            },
            "tetris" => {
                self.run_game(game);
                self.last_game = "tetris".to_string();
            },

            _ => ()
        }
    }

    pub fn handle_replay(&mut self) {
        if self.last_game.len() < 1 {
            println!("\nNo previously played game");
        } else {
            let mut _game = self.last_game.clone();
            self.run_game(_game.as_str());
        }
    }

    fn run_game(&mut self, game: &str) {
        if game == "dice" { Dice::run(&mut self.dice); }
        if game == "reaction" { Reaction::run(&mut self.reaction); }
        if game == "impulse" { Impulse::run(&mut self.impulse); }
        if game == "tetris" { Tetris::run(&mut self.tetris); }
    }
}