use std::io::{Write};

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: u32,
    pub class: String,
    pub valid_keys: Vec<&'static str>, 
}

impl Player {
    // create new player
    pub fn new() -> Self {
        Player {
            name: String::from("CrazyJohn123"),
            level: 1,
            class: String::from("Peasant"),
            valid_keys: vec!["profile", "name", "level", "class"],
        }
    }

    pub fn handle_show(&self, key: &str) {
        if !self.valid_keys.contains(&key) {
            println!("\nInvalid GET key, '{}' \nValid keys are: {:?}", key, self.valid_keys);
            return;
        }

        match key {
            "profile" => show_profile(self),
            "name" => println!("Name: {}", self.get_name()),
            "level" => println!("Level: {}", self.get_level()),
            "class" => println!("Class: {}", self.get_class()),
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
    pub fn get_class(&self) -> &String {
        return &self.class;
    }
    pub fn set_class(&mut self, other: String) {
        self.class = other;
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
    let msg = format!("Level {}. {  }", player.level, player.class);
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
    println!("Class: {}", player.class);
}