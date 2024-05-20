#[derive(Debug)]
pub struct Tetris {
    pub score: usize,
}

impl Tetris {
    pub fn new() -> Self {
        Self {
            score: 0,
        }
    }

    pub fn run() {
        show_welcome();
    }
}

fn show_welcome() {
    println!("\n=================");
    println!("Welcome to Tetris");
    println!("=================");
    println!("\nTetris tests your planning skills.");
}