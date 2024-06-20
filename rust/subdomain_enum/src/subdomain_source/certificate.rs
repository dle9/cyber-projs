use serde_json::Value;

pub struct Certificate {
    pub subdomains: Vec<String>,
    pub total_time: std::time::Duration,
}

impl Certificate {
    // run the dns record fetching and return instance of DnsRecords
    pub async fn run(target: String) -> Self {
        print!("\nStarting Certificate search...");
    
        let (subdomains_found, time_taken) = fetch_records(target).await;
        
        println!("Finished");
        return Self {
            subdomains: subdomains_found,
            total_time: time_taken,
        };
    }
}

// TODO
async fn fetch_records(target: String) -> (Vec<String>, std::time::Duration) {
    let start_time = std::time::Instant::now();
    let mut records: Vec<String> = Vec::new();

    // query records
    let client = reqwest::Client::new();
    let url = format!("https://dns.google/resolve?name={}&type=A", target);
    let response = client.get(url).send().await.unwrap();

    // parse response and data
    let body = response.text().await.unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    if let Some(answers) = json["Answer"].as_array() {
        for answer in answers {
            if let Some(data) = answer["data"].as_str() {
                records.push(data.to_string());
            }
        }
    }

    return (records, start_time.elapsed());
}