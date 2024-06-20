use std::io::Write;

// ========================== PROMPTS =============================

pub fn prompt_target_domain() -> String {
    println!();
    let mut target = String::new();

    loop {
        print_prompt("Enter target domain (exclude https:// and www.)".to_string());
        print!("> "); std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut target).expect("\nFailed to read target");
        let target = target.trim();

        if valid_target(target) {   
            break;
        }   
    }

    return target.trim().to_string();
}

// ======================== HELPER FUNCTIONS ========================

pub fn print_prompt(msg: String) {
    print!("\n+"); for _ in 0..msg.len()-2 { print!("="); } print!("+\n");
    println!("{}", msg);
    print!("+"); for _ in 0..msg.len()-2 { print!("="); } print!("+\n");
}

fn valid_target(target: &str) -> bool {
    if target.len() < 1 {
        println!("\nInvalid target");
        return false;
    }   
    return true;
}

pub fn format_title(title: String, msg: String) -> String {
    let mut result = String::new();

    result.push('\n');
    result.push('+');
    for _ in 0..6 {
        result.push('=');
    }
    result.push(' ');
    result.push_str(&title);
    result.push(' ');
    for _ in 0..6 {
        result.push('=');
    }
    result.push('+');
    result.push('\n');

    result.push_str(&msg);
    result.push('\n');

    let bottom_length = 6 + 1 + title.len() + 1 + 6;
    result.push('+');
    for _ in 0..bottom_length {
        result.push('=');
    }
    result.push('+');
    result.push('\n');

    result
}

pub fn display_title(title: &str, msg: String) {
    print!("\n+"); for _ in 0..6 { print!("="); } 
    print!(" {title} ");
    for _ in 0..6 { print!("="); } print!("+\n");

    let bottom_length = 6 + 1 + title.len() + 1 + 6;
    println!("{}", msg);
    print!("+"); for _ in 0..bottom_length { print!("="); } print!("+\n"); 
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