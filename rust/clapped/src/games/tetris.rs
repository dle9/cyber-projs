pub struct Tetris {
    pub score: usize,
}

impl Tetris {
    pub fn new() -> Self {
        Self {
            score: 0,
        }
    }
}

pub fn main() {
    println!("inside tetris");
}