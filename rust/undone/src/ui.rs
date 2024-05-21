use std::io::Write;
use crate::player::Player;

pub fn prompt_username() -> String {
    clear_terminal();
    
    let mut name = String::new();
    
    loop {
        print_title("Hello Player, What's your name?");
        print!("> "); std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut name).expect("\nFailed to read username");
        let name = name.trim();
        
        if name.len() < 1 {
            return "Guest".to_string();
        }   
        else if name.len() > 25 {
            println!("\nName can't be longer than 25 characters.");
        }
        else {
            break;
        }
    }
    
    return name.trim().to_string();
}

/// +=======================+
/// Name | Level: 0 | Coin: 0
/// +=======================+
pub fn print_main_prompt(player: &Player) {  
    let msg = format!("{} | Level: {} | Coin: {}", player.name, player.level, player.coin);
    print_title(msg.as_str());
    print!("Main> "); std::io::stdout().flush().unwrap();
}

pub fn print_title(msg: &str) {
    let msg_length = msg.len();
    println!();
    print!("+"); for _ in 0..msg_length-2 { print!("="); } print!("+"); println!();
    println!("{}", msg);
    print!("+"); for _ in 0..msg_length-2 { print!("="); } print!("+"); println!();
}

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");
    } else {
        std::process::Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }
}