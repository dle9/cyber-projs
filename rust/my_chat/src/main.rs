use tokio; mod server; mod client;
use std::env; 

#[tokio::main]
async fn main() {
    
    // collect args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify either 'server' or 'client'");
        return;
    }

    // execute server or client based on args
    match args[1].as_str() {

        // async functions must be awaited
        "server" => { match server::main().await {
            Ok(_) => println!("Server successful."),
            Err(e) => println!("Server failed, {}", e),
        }}
        
        "client" => { client::main().await; 
            // match client::main().await {
            // Ok(_) => println!("Client successful."),
            // Err(e) => println!("Client failed, {}", e), }
        }
        _ => println!("Invalid argument"),
    }
}