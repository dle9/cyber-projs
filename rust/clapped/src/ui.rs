use std::io::{Write};
use crate::player::Player;


pub fn prompt_username() -> String {
    let mut name = String::new();

    println!("\n===============================");
    println!("Hello Player, What's your name?");
    println!("===============================");
    print!("> "); std::io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut name).expect("\nFailed to read username");

    // user just presses enter
    if name == "\n" {
        return "Guest".to_string();
    }

    return name.trim().to_string();
}

/// =========================
/// Name | Level: 0 | Coin: 0
/// =========================
pub fn show_prompt(player: &Player) {  
    let msg = format!("{} | Level: {} | Coin: {}", player.name, player.level, player.coin);
    let msg_length = msg.len();

    println!();
    for _ in 0..msg_length { print!("="); } println!();
    println!("{}", msg);
    for _ in 0..msg_length { print!("="); } println!();
    print!("> "); std::io::stdout().flush().unwrap();
}