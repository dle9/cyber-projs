use serde_json::Value;

#[derive(Debug)]
pub struct DnsRecords {
    pub a_records: (Vec<String>, std::time::Duration),
    pub ns_records: (Vec<String>, std::time::Duration),
    pub mx_records: (Vec<String>, std::time::Duration),
}

impl DnsRecords {
    // run the dns record fetching and return instance of DnsRecords
    pub async fn run(target: String) -> Self {
        print!("Starting DNS record search...");
        
        // start dns tasks
        let a_handle = tokio::task::spawn(fetch_records(target.clone(), "A"));
        let ns_handle = tokio::task::spawn(fetch_records(target.clone(), "NS"));
        let mx_handle = tokio::task::spawn(fetch_records(target.clone(), "MX"));

        // wait for dns threads to finish, join the threads
        let (a_records_found, ns_records_found, mx_records_found) =
            tokio::join!(a_handle, ns_handle, mx_handle);

        println!("Finished");
        return Self {
            a_records: (a_records_found.unwrap()),
            ns_records: (ns_records_found.unwrap()),
            mx_records: (mx_records_found.unwrap()),
        };
    }

    pub fn get_total_subdomains(&self) -> usize {
        return self.a_records.0.len() + self.ns_records.0.len() + self.mx_records.0.len();
    }

    pub fn get_total_time(&self) -> std::time::Duration {
        return self.a_records.1 + self.ns_records.1 + self.mx_records.1;
    }
}

async fn fetch_records(target: String, record_type: &str) -> (Vec<String>, std::time::Duration) {
    let start_time = std::time::Instant::now();
    let mut records: Vec<String> = Vec::new();

    // query records
    let client = reqwest::Client::new();
    let url = format!(
        "https://dns.google/resolve?name={}&type={}",
        target, record_type
    );
    let response = client.get(url).send().await.unwrap();

    // parse response and data
    let body = response.text().await.unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    if let Some(answers) = json["Answer"].as_array() {
        for answer in answers {
            if let Some(item) = answer["data"].as_str() {
                let mut data = item.to_string();

                // trim the data
                if data.chars().nth(1) == Some(' ') {
                    data = data[2..].to_string();
                }
                records.push(data);
            }
        }
    }

    return (records, start_time.elapsed());
}