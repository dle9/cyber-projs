// this code will get the certificates from
// certificate transparency lists (ctls),
// and search for a specific domain w/ the results
use serde_json::Value;
use std::io::Write;

pub struct CertRecords {
    pub subdomains: Vec<String>,
    pub total_time: std::time::Duration,
}

impl CertRecords {
    // run the dns record fetching and return instance of DnsRecords
    pub async fn run(target: String) -> Self {
        print!("Starting Certificate search...");
        std::io::stdout().flush().unwrap();

        let (subdomains_found, time_taken) = fetch_records(target).await;

        println!("Finished");
        return Self {
            subdomains: subdomains_found,
            total_time: time_taken,
        };
    }
}

// get target certificates from certificate transparency lists (ctl's)
async fn fetch_records(_target: String) -> (Vec<String>, std::time::Duration) {
    let start_time = std::time::Instant::now();
    let records: Vec<String> = Vec::new();

    // get+parse the certificate transparency lists
    let ctl = fetch_ctl().await;
    match ctl {
        Ok(ctl_result) => parse_ctl(ctl_result).await.expect("Failed to parse"),
        Err(err) => println!("Error parsing ctl: {}", err),
    }

    return (records, start_time.elapsed());
}

async fn fetch_ctl() -> Result<String, Box<dyn std::error::Error>> {
    // get the certificate transparency logs
    let client = reqwest::Client::new();
    let ctl_source = format!("https://www.gstatic.com/ct/log_list/v3/log_list.json");
    let ctl_result: String = client.get(ctl_source).send().await?.text().await?;
    return Ok(ctl_result);
}

async fn parse_ctl(ctl: String) -> Result<(), Box<dyn std::error::Error>> {
    // parse response and data
    let json: Value = serde_json::from_str(&ctl).unwrap();
    if let Some(operators) = json["operators"].as_array() {
        for operator in operators {
            if let Some(logs) = operator["logs"].as_array() {
                for log in logs {
                    let client = reqwest::Client::new();

                    let log_url = log["url"].as_str().unwrap_or("no-url");
                    let _log_info: String = client
                        .get(format!("{}/ct/v2/get-sth", log_url))
                        .send()
                        .await?
                        .text()
                        .await?;

                    // println!("fetch from {}", log_url);
                    // println!("{:?}", log_info);
                }
            }
        }
    }

    Ok(())
}
