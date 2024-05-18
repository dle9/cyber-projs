use std::io::Write;

pub struct Dice {
    pub dice1: u8,
    pub dice2: u8,
    pub dice3: u8,
}

impl Dice {
    pub fn new() -> Self {
        Self {
            dice1: 0,
            dice2: 0,
            dice3: 0,
        }
    }
}

pub fn main() {
    loop {
        println!("inside dice");
        println!("inside dice");
        println!("inside dice");
        break;
    }
}