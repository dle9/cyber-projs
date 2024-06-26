// file operations
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// request concurrency
use futures::{stream, StreamExt};
use reqwest::Client;
use std::sync::{Arc, Mutex};

const CONCURRENT_REQUESTS: usize = 200;

pub struct WordlistRecords {
    pub subdomains: Vec<String>,
    pub total_time: std::time::Duration,
}

impl WordlistRecords {
    pub async fn run(target: String) -> Self {
        let (subdomains_found, time_taken) = fetch_records(target).await;
        return Self {
            subdomains: subdomains_found.lock().unwrap().clone(), // unwrap vec<string> inside arc<mutex(_)>>
            total_time: time_taken,
        };
    }
}

async fn fetch_records(target: String) -> (Arc<Mutex<Vec<String>>>, std::time::Duration) {
    print!("Starting Wordlist search...");
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

    // init values required for http request concurrency
    let client = Client::new();
    let records_original: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let records = records_original.clone(); // only clone the *reference* to original vec

    // start enumerating the wordlist and making concurrent requests
    let bodies = stream::iter(wordlist_content.split_whitespace().collect::<Vec<&str>>())
        .map(|word| {
            let url = format!("https://{}.{}", word, target);
            let client = &client;
            async move {
                let _ = client.get(url.clone()).send().await?;

                // returns to bodies to be iterated through
                // for status after this code block
                return Ok(url.clone());
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    // populate successful requests into records
    bodies
        .for_each(
            |result: Result<std::string::String, Box<dyn std::error::Error>>| async {
                match result {
                    Ok(url) => {
                        let mut mutex = records.lock().unwrap(); // lock it, get the vec<String> inside
                        mutex.push(url); // push into the vec<string>
                        drop(mutex); // unlock it
                    }
                    Err(_) => (),
                }
            },
        )
        .await;

    // return the original records_original,
    // records was just a temporary reference
    println!("Finished");
    return (records_original, start_time.elapsed());
}
