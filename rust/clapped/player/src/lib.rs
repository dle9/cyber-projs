use std::io::{Write};

#[derive(Debug)]
struct ValidOptions {
    show_options: Vec<&'static str>, 
    change_options: Vec<&'static str>, 
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: usize,
    pub coin: isize,
    valid: ValidOptions,
}

impl Player {
    // create new player
    pub fn new(name: String) -> Self {
        Player {
            name: name,
            level: 0,
            coin: 0,
            valid: ValidOptions {
                show_options: vec!["profile", "name", "level", "coin"],
                change_options: vec!["name"],
            },
        }
    }

    pub fn handle_show(&self, value: &str) {
        if !self.valid.show_options.contains(&value) {
            println!("\nInvalid SHOW value, '{}' \nChoose from: {:?}", value, self.valid.show_options);
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

    pub fn handle_change(&mut self, option: &str, value: &str) {
        if !self.valid.change_options.contains(&option) {
            println!("\nInvalid CHANGE option, '{}' \nChoose from: {:?}", option, self.valid.change_options);
            return;
        }

        match option {
            "name" => self.set_name(value.to_string()),
            _ => ()
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
    pub fn set_name(&mut self, other: String) {
        self.name = other;
    }
    pub fn get_level(&self) -> &usize {
        return &self.level;
    }
    pub fn set_level(&mut self, other: usize) {
        self.level = other;
    }
    pub fn get_coin(&self) -> &isize {
        return &self.coin;
    }
    pub fn set_coin(&mut self, other: isize) {
        self.coin = other;
    }
}

pub fn get_username() -> String {
    let mut name = String::new();

    println!("\n===============================");
    println!("Hello Player, What's your name?");
    println!("===============================");
    print!("> "); std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut name).expect("\nFailed to read username");
    println!("{}", name);

    return name.trim().to_string();
}

pub fn show_welcome(player: &Player) {
    let msg = format!("Welcome, {}", player.name);
    let msg_length = msg.len();

    // welcome msg
    println!();
    for _ in 0..msg_length { print!("="); } println!();
    println!("{}", msg);
    for _ in 0..msg_length { print!("="); } println!();

    println!("Get as much coin as you can! Good Luck!");
}

pub fn show_prompt(player: &Player) {  
    let msg = format!("{} | Level: {} | Coin: {}", player.name, player.level, player.coin);
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