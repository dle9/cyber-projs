use crate::ui::{show_prompt_dice, print_title, clear_terminal};

#[derive(Debug)]
pub struct Dice {
    score: usize,
    pub dice1: u8,
    pub dice2: u8,
    pub dice3: u8,
}

impl Dice {
    pub fn new() -> Self {
        Self {
            score: 0,
            dice1: 0,
            dice2: 0,
            dice3: 0,
        }
    }

    pub fn run(&self) {
        show_welcome();

        // loop player input
        loop {
            show_prompt_dice(0, self.score);
    
            // get user input
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
            // trim the automatic new line
            let input = input.trim();
            
            println!("{}", input);
        }
    }
}

fn show_welcome() {
    clear_terminal();
    print_title("Welcome to Dice");
    println!("\nDice tests your memory and mental math skills.");
    println!("\nEach round, roll a chosen amount of die.");
    println!("Then, you will be asked to provide the sum");
    println!("of this set, and all previous sets, of die.");
    print_help();
    
}

fn print_help() {
    println!("\nUsage: roll 1   roll 1 dice");
    println!("       roll 2   roll 2 die");
    println!("       ...                ");
    println!("       roll n   roll n die");
    println!("Alias: r");
}
