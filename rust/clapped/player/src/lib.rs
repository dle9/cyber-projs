use std::io::{Write};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub coin: String,
    pub valid_show: Vec<&'static str>, 
}

impl Player {
    // create new player
    pub fn new() -> Self {
        Player {
            name: String::from("CrazyJohn123"),
            level: 1,
            coin: String::from("Peasant"),
            valid_show: vec!["profile", "name", "level", "coin"],
        }
    }

    pub fn handle_show(&self, value: &str) {
        if !self.valid_show.contains(&value) {
            println!("\nInvalid GET value, '{}' \nValid keys are: {:?}", value, self.valid_show);
            return;
        }

        match value {
            "profile" => show_profile(self),
            "name" => println!("Name: {}", self.get_name()),
            "level" => println!("Level: {}", self.get_level()),
            "coin" => println!("coin: {}", self.get_coin()),
            _ => ()
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
    pub fn set_name(&mut self, other: String) {
        self.name = other;
    }
    pub fn get_level(&self) -> &u32 {
        return &self.level;
    }
    pub fn set_level(&mut self, other: u32) {
        self.level = other;
    }
    pub fn get_coin(&self) -> &String {
        return &self.coin;
    }
    pub fn set_coin(&mut self, other: String) {
        self.coin = other;
    }
}

pub fn show_welcome(player: &Player) {
    let msg = format!("Welcome, {}", player.name);
    let msg_length = msg.len();

    // welcome msg
    println!();
    for _ in 0..msg_length { print!("="); } println!();
    println!("{}", msg);
    for _ in 0..msg_length { print!("="); } println!();

    println!("You start off as a Peasant in Meadow Brook.");
    println!("Get as much coin as you can! Good Luck!");
}

pub fn show_prompt(player: &Player) {  
    let msg = format!("Level {}. {  }", player.level, player.coin);
    let msg_length = msg.len();

    // the prompt
    println!();
    for _ in 0..msg_length { print!("="); } println!();
    println!("{}", msg);
    for _ in 0..msg_length { print!("="); } println!();
    print!("> "); std::io::stdout().flush().unwrap();
}

pub fn show_profile(player: &Player) {
    println!("Name: {}", player.name);
    println!("Level: {}", player.level);
    println!("Coin: {}", player.coin);
}