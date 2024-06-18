// file operations
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use futures::{stream, StreamExt};
use reqwest::Client;

pub struct Wordlist {
    pub subdomain_count: usize,
    pub total_time: std::time::Duration,
}

const CONCURRENT_REQUESTS: usize = 2;

pub async fn fetch_records(target: String) {
    println!("Starting brute force search with wordlist");

    let start_time = std::time::Instant::now();

    // get path of wordlist
    let this_path = Path::new(file!());
    let wordlist_path = this_path.with_file_name("wordlist.txt");
    let display = wordlist_path.display();

    // open the wordlist for reading
    let mut file = match File::open(&wordlist_path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // read file into a string
    let mut wordlist_content = String::new();
    match file.read_to_string(&mut wordlist_content) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    // start enumerating the wordlist and making concurrent requests
    let client = Client::new();
    let records: Vec<String> = Vec::new();
    let bodies = stream::iter(wordlist_content.split_whitespace().collect::<Vec<&str>>())
        .map(|word| {
            let url = format!("https://{}.{}", word, target);
            let client = &client;
            async move {
                let resp = client.get(url.clone()).send().await?;

                // returns to bodies to be iterated through 
                // for status after this code block
                return Ok(url.clone());
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    bodies
        .for_each(|result: Result<std::string::String, Box<dyn std::error::Error>>| async {
            match result {
                Ok(url)=> println!("Successful connection to {}", url),
                Err(_) => (),
            }
        })
    .await;
}