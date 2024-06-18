use serde_json::Value;

pub struct DnsRecords {
    pub a_records: (Vec<String>, std::time::Duration),
    pub ns_records: (Vec<String>, std::time::Duration),
    pub mx_records: (Vec<String>, std::time::Duration),
}

pub async fn fetch_records(target: String, record_type: &str) -> (Vec<String>, std::time::Duration) {
    let start_time = std::time::Instant::now();
    let mut records: Vec<String> = Vec::new();

    // query records
    let client = reqwest::Client::new();
    let url = format!("https://dns.google/resolve?name={}&type={}", target, record_type);
    let response = client.get(url).send().await.unwrap(); 
    
    // parse response
    let body = response.text().await.unwrap(); 
    let json: Value = serde_json::from_str(&body).unwrap(); 

    // get data
    if let Some(answers) = json["Answer"].as_array() {
        for answer in answers {
            if let Some(data) = answer["data"].as_str() {
                records.push(data.to_string());
            }
        }
    }

    return (records, start_time.elapsed());
}

impl DnsRecords {
    pub fn get_total_subdomains(&self) -> usize {
        return self.a_records.0.len() + self.ns_records.0.len() + self.mx_records.0.len(); 
    }

    pub fn get_total_time(&self) -> std::time::Duration {
        return self.a_records.1 + self.ns_records.1 + self.mx_records.1; 
    }
}