use std::io::Write;

// ========================== PROMPTS/MSGS =============================

pub fn prompt_target_se() -> String {
    println!();
    let mut target = String::new();

    loop {
        display_title("Target for Subdomain Enumeration");
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut target)
            .expect("\nFailed to read target");
        let target = target.trim();

        if valid_target(target) {
            break;
        }
    }

    return target.trim().to_string();
}

pub fn prompt_target_vs() -> String {
    println!();
    let mut target = String::new();

    loop {
        display_title("Target for Vulnerability Scan");
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut target)
            .expect("\nFailed to read target");
        let target = target.trim();

        if valid_target(target) {
            break;
        }
    }

    return target.trim().to_string();
}

pub fn display_title(msg: &str) {
    print!("\n+");
    for _ in 0..msg.len() - 2 {
        print!("=");
    }
    print!("+\n");
    println!("{}", msg);
    print!("+");
    for _ in 0..msg.len() - 2 {
        print!("=");
    }
    print!("+\n");
}

// ======================== HELPER FUNCTIONS ========================

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

// pub fn clear_terminal() {
//     if cfg!(target_os = "windows") {
//         std::process::Command::new("cmd")
//             .args(&["/C", "cls"])
//             .status()
//             .expect("Failed to clear terminal");
//     } else {
//         std::process::Command::new("clear")
//             .status()
//             .expect("Failed to clear terminal");
//     }
// }
