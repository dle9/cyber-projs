use std::io::Write;
use crate::ui::{print_title, clear_terminal};
use rand::Rng;

#[derive(Debug)]
pub struct Dice {
    round: isize,
    score: usize,
    dice_history: Vec<usize>,
}

impl Dice {
    // create Dice instance
    pub fn new() -> Self {  
        Self {
            round: 0,
            score: 0,
            dice_history: Vec::new(),
        }   
    }

    // run the game
    pub fn run(&mut self) {
        print_welcome();
        self.main_loop();
        self.reset();
    }

    // main input loop
    fn main_loop(&mut self) {
        // loop player input
        loop {
            if self.round == -1 {break}

            self.print_dice_prompt();
    
            // get user input
            let mut input = String::new();
            let mut _command: &str = "";
            let mut dice_rolled: usize = 0;
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
                    if args.len() == 2 {
                        match args[1].parse::<usize>() {
                            Ok(n) => { 
                                _command = args[0]; 
                                dice_rolled = n; 
                            },
                            Err(_) => {  print_game_help(); continue; },
                        }
                    } else if args.len() < 2 {
                        print_game_help(); continue;
                    } else {  print_game_help(); continue; }
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
            let mut answer: isize = 0;
            match _command {
                "roll" | "r" => answer = self.handle_roll(dice_rolled) as isize,
                "help" | "h" => print_help(),
                "exit" | "q" | "e" => { answer = -1;},
                _ => (),
            }

            self.handle_round_end(answer, dice_rolled);
        }
    }

    // reset all variables
    fn reset(&mut self) {
        self.round = 0;
        self.score = 0;
        self.dice_history.clear();
    }

    // handle rolling die each round
    fn handle_roll(&mut self, amount: usize) -> usize {

        self.roll(amount);

        // get from (vec.length - i) -> to end of vec 
        let rolled = self.dice_history[self.dice_history.len() - amount as usize..].to_vec();
        println!("\nYou rolled {:?}", rolled);
        std::thread::sleep(std::time::Duration::from_millis( (amount * 333) as u64 ));

        // loop until valid input (int)
        clear_terminal();
        let sum: usize;

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

        return sum;
    }

    // how to finish each round
    fn handle_round_end(&mut self, sum: isize, amount: usize) {
        let end = self.handle_answer(sum, amount);
        self.print_round_end(end);
        self.dice_history.clear();
    }
    
    // check if sum of all dice = answer
    fn handle_answer(&mut self, answer: isize, amount: usize) -> bool {
        let target: usize = self.dice_history.iter().sum();
        if answer == target as isize {
            let round_score = self.calculate_score(amount);
            self.score += round_score as usize;  
            self.round += 1;

            // round is not ended
            return false;
        } 
        
        else {
            self.round = -1;

            // round is ended
            return true;
        }
    }

    // score components
    fn calculate_score(&self, amount: usize) -> f64 {
        return 100.0 * amount as f64 * (amount as f64).powf(1.5);
    }

    // roll die
    fn roll(&mut self, amount: usize) { 
        let mut rng = rand::thread_rng();

        for _i in 0..amount {
            self.dice_history.push(rng.gen_range(1..=6));
        }
    }

    // prompt for each round
    fn print_dice_prompt(&self) {  
        let msg = format!("Round: {} | Score: {} | Timer: ", self.round, self.score);
        print_title(msg.as_str());
        print!("Dice> "); std::io::stdout().flush().unwrap();
    }

    // finish up each round with a message
    fn print_round_end(&self, end: bool) {
        if !end {
            clear_terminal();
            println!("Correct!");
        } else {
            let dice_sum: usize = self.dice_history.iter().sum();

            if self.dice_history.len() > 0 {
                println!("\nIncorrect. The current die are:");
                println!("{:?}", self.dice_history);
                println!("The sum is: {}", dice_sum);
            }
            println!("\nReturning to Main.");
        }
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
    print_game_help();
}

fn print_game_help() {
    println!("\nUsage: roll 1   roll 1 dice");
    println!("       roll 2   roll 2 die");
    println!("       ...");
    println!("       roll n   roll n die");
    println!("Alias: r");
}
