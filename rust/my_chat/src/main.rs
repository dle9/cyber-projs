use std::env; use std::io::{Write};
use tokio; mod server; mod client;

#[tokio::main]
async fn main() {
    // define server and client sockets
    let server_addr: &str = "localhost:8080";
    let connect_to_addr: &str = "localhost:8080"; 

    // handle user input
    // ask if user wants to run server or client
    let mode: &str = &handle_user_input();
    
    // determine next steps based on user input
    match mode {
        "server" => { 
            match server::main(server_addr).await {
                Ok(_) => println!("\nServer finished"),
                Err(e) => println!("\nServer failed, {}", e),
            }
        }
        "client" => {
            match client::main(connect_to_addr).await {
                Ok(_) => println!("\nClient finished"),
                Err(e) => println!("\nClient failed, {}", e), 
            }
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => println!("\n Usage: run with 'server' or 'client'"),
    }
}

// when the user runs with no additional arguments
fn show_options() {
    println!("\nRun as server or client?");
    println!("========================");
    println!("1. Server");
    println!("2. Client");
    println!("3. Exit");
}

// prompt the user to run server or client
// then, get their chosen option
fn get_option() -> u8 {
    loop {
        // get the input
        let mut input = String::new();
        show_options();
        print!("Option> "); std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("\nFailed to read input");
        
        // parse the input
        let input = input.trim();
        match input.parse::<u8>() {
            Ok(option) => {
                if option == 1 || option == 2 || option == 3 {
                    return option;
                } else {
                    println!("\nPlease enter a valid option.");
                }
            }
            Err(_) => { println!("\nPlease enter a valid option."); }
        }
    }
}

// return the chosen mode (server, client, exit)
fn handle_user_input() -> String {
    // get the arguments
    // `cargo run` counts as one argument
    let args: Vec<String> = env::args().collect();
    let mut mode: String = "".to_string();
    
    // Used: cargo run
    if args.len() < 2 {
        let option = get_option(); 
        if option == 1 { 
            mode = "server".to_string(); }
        else if option == 2 { 
            mode = "client".to_string(); }
        else if option == 3 { 
            mode = "exit".to_string(); }
    } 

    // Used: cargo run <X>
    else if args.len() == 2 {
        if args[1].as_str() == "server" || args[1].as_str() == "1" { 
            mode = "server".to_string(); }
        else if args[1].as_str() == "client" || args[1].as_str() == "2" { 
            mode = "client".to_string(); }
    }

    return mode;
}