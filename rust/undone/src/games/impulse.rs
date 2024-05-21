use crate::ui::{print_title, clear_terminal};
use rand::Rng;
use colored::Colorize;

use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Impulse {
    round: isize,
    remaining_time: u64, //ms
}

impl Impulse {
    // create impulse instnace
    pub fn new() -> Self {
        Self {
            round: 0,
            remaining_time: 3000, //ms
        }
    }
    
    // run the game
    pub fn run(&mut self) {
        print_welcome_loop();
        self.main_loop();
        self.reset();
    }

    // main input loop
    fn main_loop(&mut self) {
        let mut _end: bool = false;
        
        loop {
            if self.round == -1 { break; }

            let (word, color) = self.print_impulse_prompt();
            let mut _command: &str = "";
            let mut _attempted = false;
            
            // some fancy thread stuff
            let input = Arc::new(Mutex::new(String::new()));
            let input_clone_for_input = Arc::clone(&input);
            let input_clone_for_timeout = Arc::clone(&input);

            let input_handled = Arc::new(Mutex::new(false));
            let input_handled_clone_for_input = Arc::clone(&input_handled);
            let input_handled_clone_for_timeout = Arc::clone(&input_handled);

            // Spawn a thread to read input
            let input_thread = thread::spawn(move || {
                let mut input = input_clone_for_input.lock().unwrap();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                *input_handled_clone_for_input.lock().unwrap() = true;
            });
            
            // Spawn a thread for timeout
            let remaining_time = self.remaining_time;
            let timeout_thread = thread::spawn(move || {
                thread::sleep(Duration::from_millis(remaining_time));
                if !*input_handled_clone_for_timeout.lock().unwrap() {
                    let mut input = input_clone_for_timeout.lock().unwrap();
                    *input = "times up".to_string();  // Simulate user input
                    *input_handled_clone_for_timeout.lock().unwrap() = true;
                }
                clear_terminal();
            });

            input_thread.join().unwrap();
            timeout_thread.join().unwrap();

            let input = input.lock().unwrap();

            // user press enter
            if *input != "times up" {
                _attempted = true;
            } 
            // user type help or exit
            else {
                // parse the input
                let args: Vec<&str> = input.split_whitespace().collect();
    
                // error checking
                match args[0] {
                    "help" | "h" => {
                        if args.len() <= 1 { _command = args[0]; }
                        else { println!("\nUsage: help"); continue;}
                    }
                    "exit" | "q" | "e" => {
                        if args.len() <= 1 { _command = args[0]; }
                        else { println!("\nUsage: exit"); continue; }
                    }
                    _ => _end = true,
                }
    
                // execute args
                match _command {
                    "help" | "h" => print_help(),
                    "exit" | "q" | "e" => { println!("\nReturning to Main."); break;},
                    _ => _end = true,
                }
            }

            _end = self.handle_answer(_attempted, word.as_str(), color.as_str());
            self.handle_round_end(_end);
        }
    }

    fn print_impulse_prompt(&mut self) -> (String, String) {
        // get a random word and color
        let mut rng = rand::thread_rng();
        let mut word: &str = "";
        let mut color: &str = "";

        match rng.gen_range(1..=3) {
            1 => word = "blue",
            2 => word = "red",
            3 => word = "yellow",
            _ => (),
        }

        match rng.gen_range(1..=3) {
            1 => color = "blue",
            2 => color = "red",
            3 => color = "yellow",
            _ => (),
        }

        let start_time = std::time::Instant::now();
        while self.remaining_time > 0 {
            // let elapsed_time = std::time::Instant::now().duration_since(start_time).as_millis() as u64;
            self.remaining_time -= 1;
    
            // Construct the message
            let msg = format!("Round: {} | {} | Time: {} ms", self.round, word, self.remaining_time.max(0));
            
            // Print the message with a carriage return at the beginning
            print_title(msg.as_str());
            self.print_color(word, color);
            
            std::io::stdout().flush().unwrap();
            
            // Sleep for a short duration to reduce CPU usage (e.g., 50 ms)
            thread::sleep(Duration::from_millis(50));
        }

        // let msg = format!("Round: {} | {} | Time: {} ms", self.round, word, self.remaining_time);
        print!("Impulse> "); std::io::stdout().flush().unwrap();

        return (word.to_string(), color.to_string());
    }

    fn handle_answer(&mut self, attempted: bool, word: &str, color: &str) -> bool {
        if attempted && word == color {
            self.round += 1;
            return false;
        } 
        else if !attempted && word != color {
            self.round += 1;
            return false;
        }
        else {
            self.round = -1;
            return true;
        }
    }

    fn handle_round_end(&mut self, status: bool) {
        // clear_terminal();
        self.print_round_end(status);
    }

    fn print_round_end(&self, end: bool) {
        if !end {
            // clear_terminal();
            println!("Correct!");
        } else {
            println!("\nIncorrect");
        }
    }

    fn print_color(&self, word: &str, color: &str) {
        // print color w/ same width as prompt
        let msg = format!("Round: {} | {} | Time: {} ms", self.round, word, self.remaining_time);

        match color {
            "blue" =>  {
                println!();
                for _ in 0..msg.len() { print!("\x1b[34;44m "); } print!("\x1b[0m");
                println!("\n");
            },
            "red" => {
                println!();
                for _ in 0..msg.len() { print!("\x1b[31;41m "); } print!("\x1b[0m");
                println!("\n");  
            },
            "yellow" => {
                println!();
                for _ in 0..msg.len() { print!("\x1b[33;43m "); } print!("\x1b[0m");
                println!("\n");  
            },
            _ => (),
        }
    }

    // reset vars
    fn reset(&mut self) {
        self.round = 0;
        self.remaining_time = 3000; //ms
    }
}

fn print_welcome_loop() {
    clear_terminal();

    print_title("Welcome to Impulse");
    println!("\nImpulse tests your reaction speed.");
    println!("\nEach round, you will be given one of");
    println!("three colors. Choose the correct color.");
    print_help();
        
    loop {
        let mut input = String::new();
        print!("\nPress {} to begin> ", "Enter".italic()); std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        if input == "\n" {break}
    }
}

fn print_help() {
    println!("\nCommands: {} | help (h) | exit (q, e)", "Enter".italic());
    print_game_help();
}

fn print_game_help() {
    println!("\nUsage: Press {}, or ignore", "Enter".italic());
}
