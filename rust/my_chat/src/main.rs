mod server; mod client;
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
        "server" => {   
            server::main().await;
        }
        "client" => {
            client::main().await;
        }
        _ => println!("Invalid argument"),
    }
}