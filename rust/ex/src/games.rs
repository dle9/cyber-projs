mod dice;
mod impulse;
mod tetris;
use crate::player;

// init valid options for
// PLAY command
#[derive(Debug)]
struct ValidOptions {
    play_options: Vec<&'static str>, 
}

pub struct Games {
    dice: dice::Dice,
    impulse: impulse::Impulse,
    tetris: tetris::Tetris,
    valid: ValidOptions,
}

impl Games {
    pub fn new(player: &player::Player) -> Self {
        Self {
            dice: dice::Dice {
                dice1: 0,
                dice2: 0,
                dice3: 0,
            },
            impulse: impulse::Impulse {
                word: String::new(),
                color: String::new(),
                selection: String::new(),
            },
            tetris: tetris::Tetris {
                score: 0,
            },

            // define valid options to be used 
            // with PLAY command
            valid: ValidOptions {
                play_options: vec!["dice", "impulse", "tetris"],
            }
        }
    }

    pub fn handle_play(&self, game: &str) {
        if !self.valid.play_options.contains(&game) {
            println!("\nInvalid game, '{}' \nChoose from: {:?}", game, self.valid.play_options);
            return;
        }

        match game {
            "dice" => println!("play dice"),
            "impulse" => println!("play impulse"),
            "tetris" => println!("play tetris"),
            _ => ()
        }
    }
}