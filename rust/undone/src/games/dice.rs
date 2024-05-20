use std::io::Write;
use crate::ui::{print_dice_prompt, print_title, clear_terminal};
use rand::Rng;

#[derive(Debug)]
pub struct Dice {
    round: isize,
    score: usize,
    dice_history: Vec<usize>,
}

impl Dice {
    pub fn new() -> Self { 
        Self {
            round: 0,
            score: 0,
            dice_history: Vec::new(),
        }   
    }

    pub fn run(&mut self) {
        print_welcome();
        self.main_loop();
        self.reset();
    }

    fn main_loop(&mut self) {
        // loop player input
        loop {
            if self.round == -1 {break}

            print_dice_prompt(self.round, self.score);
    
            // get user input
            let mut input = String::new();
            let mut _command: &str = "";
            let mut amount: usize = 0;
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
            // trim the automatic new line
            let input = input.trim();

            if input.len() < 1 {
                continue;
            }

            // parse the input
            let args: Vec<&str> = input.split_whitespace().collect();
            
            // error checking
            match args[0] {
                "roll" | "r" => {  
                    if args.len() < 2 {
                         print_roll_help(); continue;
                    } else if args.len() == 2 {
                        match args[1].parse::<usize>() {
                            Ok(n) => { 
                                _command = args[0]; 
                                amount = n; 
                            },
                            Err(_) => {  print_roll_help(); continue; },
                        }
                    } else {  print_roll_help(); continue; }
                },
                "help" | "h" => {
                    if args.len() <= 1 { _command = args[0]; }
                    else { println!("\nUsage: help"); continue;}
                }
                "exit" | "q" | "e" => {
                    if args.len() <= 1 { _command = args[0]; }
                    else { println!("\nUsage: exit"); continue; }
                }
                _ => { _command = args[0]; println!("\nInvalid command '{_command}'"); continue; },
            }

            // execute args
            match _command {
                "roll" | "r" => self.handle_roll(amount),
                "help" | "h" => print_help(),
                "exit" | "q" | "e" => { println!("\nReturning to Main."); break;},
                _ => (),
            }
        }
    }

    fn handle_roll(&mut self, amount: usize) {

        self.roll(amount);

        // get from (vec.length - i) -> to end of vec 
        let rolled = self.dice_history[self.dice_history.len() - amount as usize..].to_vec();
        println!("\nYou rolled {:?}", rolled);
        std::thread::sleep(std::time::Duration::from_millis( (amount * 333) as u64 ));
        self.handle_round_end(amount);
    }

    fn handle_round_end(&mut self, amount: usize) {
        clear_terminal();

        let sum: usize;

        // loop until valid input (int)
        loop {
            let mut input = String::new();
            print_title("Sum of die?");
            print!("> "); std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).expect("\nFailed to read input");
            
            let input = input.trim();
            match input.parse::<usize>() {
                Ok(n) => {
                    sum = n;
                    break;
                },
                Err(_) => println!("\nInvalid input"),
            }
        }

        self.handle_answer(sum, amount);
        self.dice_history.clear();
    }
    
    fn handle_answer(&mut self, answer: usize, amount: usize) {
        let target: usize = self.dice_history.iter().sum();
        if answer == target {
            let round_score: f64 = 100.0 * amount as f64 * self.calculate_score_multiplier(amount);
            self.score += round_score as usize;  
            self.round += 1;
            self.print_round_end(false);
        } else {
            // -1, end the game
            self.round = -1;
            self.print_round_end(true);
        }
    }

    fn calculate_score_multiplier(&self, amount: usize) -> f64 {
        return (amount as f64).powf(1.5);
    }

    fn roll(&mut self, amount: usize) { 
        let mut rng = rand::thread_rng();

        for _i in 0..amount {
            self.dice_history.push(rng.gen_range(1..=6));
        }
    }

    fn print_round_end(&self, end: bool) {
        if !end {
            clear_terminal();
            println!("Correct!");
        } else {
            let dice_sum: usize = self.dice_history.iter().sum();
            println!("\nIncorrect. The current die are:");
            println!("{:?}", self.dice_history);
            println!("The sum is: {}", dice_sum);
            println!("\nScore: {}", self.score);
        }
    }

    fn reset(&mut self) {
        self.dice_history.clear();
        self.round = 0;
        self.score = 0;
    }
}

fn print_welcome() {
    clear_terminal();
    print_title("Welcome to Dice");
    println!("\nDice tests your memory and mental math skills.");
    println!("\nEach round, roll a chosen amount of die.");
    println!("Then, you will be asked to provide the sum.");
    print_help();
    
}

fn print_help() {
    println!("\nCommands: roll (r) | help (h) | exit (q, e)");
    print_roll_help();
}

fn print_roll_help() {
    println!("\nUsage: roll 1   roll 1 dice");
    println!("       roll 2   roll 2 die");
    println!("       ...");
    println!("       roll n   roll n die");
    println!("Alias: r");
}
