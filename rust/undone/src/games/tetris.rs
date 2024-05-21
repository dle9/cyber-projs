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

    pub fn run(&mut self) {
        print_welcome();
        self.main_loop();
        self.reset();
    }

    fn main_loop(&mut self) {}
    fn reset(&mut self) {}
}

fn print_welcome() {
    println!("\n=================");
    println!("Welcome to Tetris");
    println!("=================");
    println!("\nTetris tests your planning skills.");
}