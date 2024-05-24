use crate::ui::{print_title, clear_terminal};
use rand::Rng;
use colored::Colorize;
use std::io::Write;

#[derive(Debug)]
pub struct Impulse {
    round: isize,
    time_limit: u64, //ms
}

impl Impulse {
    // create impulse instnace
    pub fn new() -> Self {
        Self {
            round: 0,
            time_limit: 1000000, //ms
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
            if self.round == -1 { break; }
            
            // reset variableson every loop start
            self.update_time_limit();
            let mut _command: &str = "";
            
            // prompt for answer
            let (word, color) = self.impulse_prompt();

            // capture input
            let start_time = std::time::Instant::now();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("\nFailed to read input");

            // handle user input
            let elapsed_time = start_time.elapsed();  
            let time_limit = std::time::Duration::from_millis(self.time_limit);

            if elapsed_time <= time_limit {
                input = "in time".to_string();
            } else {
                input = "times up".to_string();
            }

            let mut _end: bool = self.handle_answer(input.as_str(), word.as_str(), color.as_str());
            self.handle_round_end(_end, elapsed_time);
        }
    }

    fn impulse_prompt(&mut self) -> (String, String) {
        // get a random word and color for each round
        let (word, color) = self.get_round_info();

        // print the round info
        let msg = format!("Round: {} | {} | Time limit: {} ms", self.round, word, self.time_limit.max(0));
        print_title(msg.as_str());

        // sleep between prompt and color
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.print_color(word, color);
        print!("Impulse> "); std::io::stdout().flush().unwrap();

        // return round word and color to main loop
        return (word.to_string(), color.to_string());
    }

    fn handle_answer(&mut self, input: &str, word: &str, color: &str) -> bool {
        // println!("input {}", input);
        if self.round == 0 {
            self.round += 1;
            return false;
        }
        else if input == "in time" && word == color {
            self.round += 1;
            return false;
        } 
        else if input == "times up" && word != color {
            self.round += 1;
            return false;
        }

        // lose on:
        // in time , word != color
        // times up, word == color
        else {
            self.round = -1;
            return true;
        }
    }

    fn handle_round_end(&mut self, end: bool, elapsed_time: std::time::Duration) {
        // end from handle_answer
        // true = end the game
        self.print_round_end(end, elapsed_time);
    }

    fn get_round_info(&self) -> (&str, &str) {
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

        return (word, color);
    }

    fn update_time_limit(&mut self) {
        if self.round == 1 {
            self.time_limit = 5000; 
        } 
        else if self.round == 2 {
            self.time_limit = 3000;
        }        
        else if self.round == 3 {
            self.time_limit = 2000;
        }        
        else if self.round == 4 {
            self.time_limit = 1000;
        }        
        else if self.round == 5 {
            self.time_limit = 800;
        }        
        else if self.round <= 11 { //500 ms
            self.time_limit -= 50;
        }
        else if self.round <= 16 { //350 ms
            self.time_limit -= 30;
        }
        else {
            self.time_limit -= 10;
        }
    }

    fn print_round_end(&self, end: bool, elapsed_time: std::time::Duration) {
        // end from handle_answer
        // true = end the game
        if !end {
            if self.round == 1 {
                println!("\nYou answered in {:?}.", elapsed_time);
                println!("Now you try. Think carefully!");
            } else {
                clear_terminal();
                println!("Correct! You answered in {:?}", elapsed_time);
            }
        } else {
            if elapsed_time > std::time::Duration::from_millis(self.time_limit) {
                println!("\nYou ran out of time!");
                println!("You answered in {:?}", elapsed_time);
                println!("Returning to Main");
            } else {
                println!("Incorrect. You answered in {:?}", elapsed_time);
                println!("Returning to main.");
            }
        }
    }

    fn print_color(&self, word: &str, color: &str) {
        // print color w/ same width as prompt
        let msg = format!("Round: {} | {} | Time limit: {} ms", self.round, word, self.time_limit);

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
        self.time_limit = 1000000; //ms
    }

    fn print_welcome_loop(&mut self) {
        clear_terminal();
    
        print_title("Welcome to Impulse");
        println!("\nImpulse tests your reaction speed.");
        println!("\nEach round, you will be given one of");
        println!("three colors. Choose the correct color.");
        
        // loop until user press enter
        loop {
            let mut input = String::new();
            println!("\nPress {} to begin, or {} for the tutorial", "Enter".italic(), "t".italic()); 
            print!("> "); std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            
            // handle input 
            match input.as_str() {
                // skip tutorial
                "\n" => {
                    self.round = 1;
                    break;
                },  
                // enter tutorial
                "t\n" => {
                    break;
                },
                _ => println!("\nInvalid command")
            }
        }
    
        clear_terminal();

        // tutorial
        if self.round == 0 {
            println!("The first round is a freebie!");
            println!("Press {}.", "Enter".italic());
        }
    }
}