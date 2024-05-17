use clap::{ Parser, Subcommand };
use std::io::{Write};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about = "View a value. Choose from: 'profile' or profile components.")]
    Show{key: String},
    Set{key: String, value: String},
}

#[derive(Debug)]
struct Player {
    name: String,
    level: u32,
    class: String,
    valid_keys: Vec<&'static str>, 
}

fn main() {

    // create a new player
    let mut player = Player::new();
    show_welcome(&player);

    // loop player input
    loop {

        show_prompt(&player);

        // get player input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let input = format!("{} {}", "car ", input);

        // split the input into args
        let args = shlex::split(&input).ok_or("error: Invalid quoting").unwrap();

        // parse the input
        let cli = Cli::try_parse_from(args.iter());

        // check if parsing was successful
        match cli {
            Ok(cli) => {
                // handle the CLI input args
                match cli.commands {
                    Commands::Show { key } => player.handle_show(key.as_str()),
                    Commands::Set { key, value } => player.handle_set(key.as_str(), value.as_str()),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

impl Player {
    // create new player
    fn new() -> Self {
        Player {
            name: String::from("CrazyJohn123"),
            level: 1,
            class: String::from("Peasant"),
            valid_keys: vec!["profile", "name", "level", "class"],
        }
    }

    fn handle_show(&self, key: &str) {
        if !self.valid_keys.contains(&key) {
            println!("\nInvalid GET key, '{}' \nValid keys are: {:?}", key, self.valid_keys);
            return;
        }

        match key {
            "name" => println!("\nYour name is: {}", self.get_name()),
            _ => ()
        }
    }

    fn handle_set(&mut self, key: &str, value: &str) {
        if !self.valid_keys.contains(&key) {
            println!("\nInvalid SET key, '{}' \nValid keys are: {:?}", key, self.valid_keys);
            return;
        }

        match key {
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

fn show_welcome(player: &Player) {
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

fn show_prompt(player: &Player) {  
    let msg = format!("Level {}, {  }", player.level, player.class);
    let msg_length = msg.len();

    // the prompt
    println!();
    for _ in 0..msg_length { print!("="); } println!();
    println!("{}", msg);
    for _ in 0..msg_length { print!("="); } println!();
    print!("> "); std::io::stdout().flush().unwrap();
}

fn show_profile(player: &Player) {
    let msg1 = format!("Name: {}", player.name);
    let msg2 = format!("Level: {}", player.level);
    let msg3 = format!("Class: {}", player.class);
    let mut max_msg = msg1.len();
    if msg2.len() > max_msg {
        max_msg = msg2.len();
    }
    if msg3.len() > max_msg {
        max_msg = msg3.len();
    }

    // the prompt
    println!();
    for _ in 0..max_msg { print!("="); } println!();
    println!("{:<1$}{0}", "Profile", max_msg);
    for _ in 0..max_msg { print!("="); } println!();
    print!("> "); std::io::stdout().flush().unwrap();
}