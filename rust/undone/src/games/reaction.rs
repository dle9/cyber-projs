use crate::ui::{print_title, clear_terminal};
use rand::Rng;
use colored::Colorize;
use std::io::Write;

#[derive(Debug)]
pub struct Reaction {
    round: isize,
    fastest: std::time::Duration,
}

impl Reaction {
    // create impulse instnace
    pub fn new() -> Self {
        Self {
            round: 0,
            fastest: std::time::Duration::from_millis(10000),
        }
    }
    
    // run the game
    pub fn run(&mut self) {
        self.print_welcome_loop();
        self.main_loop();
        self.reset();
    }

    // main input loop
    fn main_loop(&mut self) {
        loop {
            if self.round == -1 { break }
            
            // reset variables on every loop start
            let mut _command: &str = "";

            // time until user can input             
            let reveal_time = self.reaction_prompt();
            let start_time = std::time::Instant::now();
            std::thread::sleep(std::time::Duration::from_millis(reveal_time));
            self.reveal();

            // capture input
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("\nFailed to read input");
            
            match input.trim() {
                "exit" | "q" | "e" => break,
                _ => (),
            }

            // handle user input
            let elapsed_time = start_time.elapsed();  
            let reveal_time = std::time::Duration::from_millis(reveal_time);

            let (reaction_time, mut _end) = self.handle_answer(elapsed_time, reveal_time);
            self.handle_round_end(_end, reaction_time);
        }
        println!("\nGoodbye");
    }

    fn reaction_prompt(&mut self) -> u64 {
        // get a random show time
        let reveal_time = self.get_round_info();

        // print the round info
        let msg = format!("Round: {} | Fastest: {:?}", self.round, self.fastest);
        print_title(msg.as_str());

        // sleep between prompt and reveal
        print!("Reaction> "); std::io::stdout().flush().unwrap();

        return reveal_time;
    }

    fn handle_answer(&mut self, elapsed_time: std::time::Duration, reveal_time: std::time::Duration) -> (std::time::Duration, bool) {
        let reaction_time: std::time::Duration;

        // good
        if elapsed_time >= reveal_time {
            reaction_time = elapsed_time - reveal_time;
            if reaction_time < self.fastest {self.fastest = reaction_time}
            self.round += 1;
            return (reaction_time, false);
        } 
        
        // too early
        else {
            reaction_time = elapsed_time;
            self.round = -1;
            return (reaction_time, true);
        }
    }

    fn handle_round_end(&mut self, end: bool, reaction_time: std::time::Duration) {
        // end from handle_answer
        // true = end the game
        self.print_round_end(end, reaction_time);
    }

    fn get_round_info(&self) -> u64 {
        let mut rng = rand::thread_rng();
        let reveal_time = rng.gen_range(700..=6666);
        return reveal_time;
    }

    fn reveal(&self) {
        let msg = format!("Round: {} | Fastest: {:?}", self.round, self.fastest);

        println!();
        for _ in 0..msg.len() { print!("\x1b[32;42m "); } print!("\x1b[0m");
        println!("\n");
    }

    fn print_round_end(&self, end: bool, reaction_time: std::time::Duration) {
        // end from handle_answer
        // true = end the game
        println!("You answered in {:?}", reaction_time);
        if end {
            println!("Returning to main.");
        }
    }

    // reset vars
    fn reset(&mut self) {
        self.round = 0;
        self.fastest = std::time::Duration::from_millis(0);
    }

    fn print_welcome_loop(&mut self) {
        clear_terminal();
        print_title("Welcome to Reaction");
        println!("\nUse exit, e, or q, to quit.");
        
        // loop until user press enter
        loop {
            let mut input = String::new();
            println!("Press {} to begin", "Enter".italic()); 
            print!("> "); std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            
            // handle input 
            match input.as_str() {
                "\n" => {
                    self.round = 0;
                    break;
                },  
                _ => println!("\nInvalid command")
            }
        }
    
        clear_terminal();
    }
}