pub struct Impulse {
    pub word: String,
    pub color: String,
    pub selection: String,
}

impl Impulse {
    pub fn new() -> Self {
        Self {
            word: String::new(),
            color: String::new(),
            selection: String::new(),
        }
    }
}

pub fn main() {
    println!("inside impulse");
}