// use std::io::Write;

#[derive(Debug)]
pub struct Impulse {
    // player: &Player,
    score: usize,
}

impl Impulse {
    pub fn new() -> Self {
        Self {
            // player,
            score: 0,
        }
    }

    pub fn run() {
        show_welcome();
    }
}

fn show_welcome() {
    // ui::print_title("Welcome to Dice");
    println!("\n===============");
    println!("Welcome to Impulse");
    println!("===============");
    println!("\nImpulse tests your impulse control.");
    println!("\nEach round, you will be given two sets");
    println!("of words. The first set is the color.");
    println!("The second set is the word. Decide if");
    println!("the color matches the word.");
}
