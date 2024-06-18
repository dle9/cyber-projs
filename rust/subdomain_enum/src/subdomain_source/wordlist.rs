use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

pub struct Wordlist {
    pub subdomain_count: usize,
    pub total_time: std::time::Duration,
}

pub async fn fetch_records(target: String) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let mut records: Vec<String> = Vec::new();

    // // iterate through wordlist and attempt to connect
    // let client = reqwest::Client::new();

    // let path = Path::new(path_name);

    // let display = path.display();
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("Couldn't open {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // read file into a string
    // let mut subdomain_file_content = String::new();
    // match file.read_to_string(&mut subdomain_file_content) {
    //     Err(why) => panic!("Couldn't read {}: {}", display, why),
    //     Ok(_) => print!("Successfully read from {}", display),
    // }

    // println!("starting wordlist");
    // for domain in subdomain_file_content.split_whitespace().collect::<Vec<&str>>() {
    //     // let url = format!("https://{}.spglobal.com", domain);
    //     println!("{domain}");
    //     // let response = client.get(url).send().await.unwrap(); 
        
    //     // // parse response
    //     // let body = response.text().await.unwrap(); 
    //     // let json: Value = serde_json::from_str(&body).unwrap(); 
    
    //     // // get data
    //     // if let Some(answers) = json["Answer"].as_array() {
    //     //     for answer in answers {
    //     //         if let Some(data) = answer["data"].as_str() {
    //     //             records.push(data.to_string());
    //     //         }
    //     //     }
    //     // }
    // }
    // println!("ending wordlist");

    println!("{:?}", std::env::current_dir());
    Ok(())
}

impl Wordlist {
    pub fn get_total_subdomains(&self) -> usize {
        return self.subdomain_count;
    }

    pub fn get_total_time(&self) -> std::time::Duration {
        return self.total_time;
    }
}