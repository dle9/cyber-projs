// use player module from main mod's
use crate::{games::Games, ui, util};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: usize,
    pub coin: isize,
    valid: util::PlayerOptions,
    pub games: Games
}

impl Player {
    // create new player
    pub fn new(name: String) -> Self {
        show_welcome(name.as_str());
        Self {
            name: name,
            level: 0,
            coin: 0,

            // define valid options to be used 
            // with SHOW and CHANGE
            valid: util::PlayerOptions {
                show_options: vec!["profile", "name", "level", "coin"],
                change_options: vec!["name"],
            },

            // create games instance for the player
            games: Games::new(),
        }
    }

    pub fn handle_show(&self, value: &str) {
        if !self.valid.show_options.contains(&value) {
            println!("\nInvalid value, '{}' \nChoose from: {:?}", value, self.valid.show_options);
            return;
        }

        match value {
            "profile" => show_profile(self),
            "name" => println!("Name: {}", self.get_name()),
            "level" => println!("Level: {}", self.get_level()),
            "coin" => println!("Coin: {}", self.get_coin()),
            _ => ()
        }
    }

    pub fn handle_change(&mut self, option: &str, value: &str) {
        if !self.valid.change_options.contains(&option) {
            println!("\nInvalid option, '{}' \nChoose from: {:?}", option, self.valid.change_options);
            return;
        }

        match option {
            "name" => self.set_name(value.to_string()),
            _ => ()
        }
    }

    fn get_name(&self) -> &String {
        return &self.name;
    }
    fn set_name(&mut self, other: String) {
        self.name = other;
    }
    fn get_level(&self) -> &usize {
        return &self.level;
    }
    fn set_level(&mut self, other: usize) {
        self.level = other;
    }
    fn get_coin(&self) -> &isize {
        return &self.coin;
    }
    fn set_coin(&mut self, other: isize) {
        self.coin = other;
    }
}

fn show_welcome(name: &str) {
    ui::clear_terminal();
    let msg = format!("Welcome, {}", name);
    ui::print_title(msg.as_str());
    println!("Get as much coin as you can. Good Luck!");
}

fn show_profile(player: &Player) {
    println!("Name: {}", player.get_name());
    println!("Level: {}", player.get_level());
    println!("Coin: {}", player.get_coin());
}