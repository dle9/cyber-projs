// use player module from the main mod's
use crate::util;
use crate::player;

mod dice;
mod impulse;
mod tetris;

pub struct Games {
    dice: dice::Dice,
    impulse: impulse::Impulse,
    tetris: tetris::Tetris,
    valid: util::GameOptions,
}

impl Games {
    pub fn new(player: &player::Player) -> Self {
        Self {
            dice: dice::Dice::new(),
            impulse: impulse::Impulse::new(),
            tetris: tetris::Tetris::new(),

            // define valid options to be used 
            // with PLAY command
            valid: util::GameOptions {
                game_options: vec!["dice", "impulse", "tetris"],
            }
        }
    }

    pub fn handle_play(&self, game: &str) {
        if !self.valid.game_options.contains(&game) {
            println!("\nInvalid game, '{}' \nChoose from: {:?}", game, self.valid.game_options);
            return;
        }

        match game {
            "dice" => self.run_game(game),
            "impulse" => self.run_game(game),
            "tetris" => self.run_game(game),
            _ => Ok(())
        }.expect("Failed to run {game}");
    }

    fn run_game(&self, game: &str) -> std::io::Result<()> {
        // Spawn the child process
        let mut child = std::process::Command::new("cargo")
            .arg("run")
            .arg("--bin")
            .arg(game)
            .spawn()?;

        // wait for child process to finish
        let status = child.wait()?;

        if !status.success() {
            eprintln!("Error running {}: exited with status {}", game, status);
        }

        Ok(())
    }
}