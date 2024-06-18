use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod subdomain_source;
mod display;

// contains all domain sources
struct SubdomainSource {
    dns_records: subdomain_source::dns::DnsRecords,
    wordlist: subdomain_source::wordlist::Wordlist,
}

#[tokio::main]
async fn main() {
    let target = display::target_domain();

    // start dns tasks
    let a_handle = tokio::task::spawn(subdomain_source::dns::fetch_records(target.clone(), "A"));
    let ns_handle = tokio::task::spawn(subdomain_source::dns::fetch_records(target.clone(), "NS"));
    let mx_handle = tokio::task::spawn(subdomain_source::dns::fetch_records(target.clone(), "MX"));

    // wait for async tasks to finish, join the threads
    let (a_records, ns_records, mx_records) = tokio::join!(a_handle, ns_handle, mx_handle);

    // handle result
    let domains = SubdomainSource {
        dns_records: subdomain_source::dns::DnsRecords {
            a_records: a_records.unwrap(),
            ns_records: ns_records.unwrap(),
            mx_records: mx_records.unwrap(),
        },
        wordlist: subdomain_source::wordlist::Wordlist {
            subdomain_count: 1, 
            total_time: std::time::Duration::from_secs(1),
        }
    };

    // write results to src/output/
    domains.write_subdomains();

    // display time taken, subdomains found, etc
    domains.display_summary();

    subdomain_source::wordlist::fetch_records(target.clone()).await;
}

impl SubdomainSource {
    // ====================================== WRITE FUNCTIONS ==========================================
    fn write(&self, filepath: &str, data: String) {
        let path = Path::new(filepath);
        let display = path.display();
    
        // open file in write only (File::create)
        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        
        // write to file
        match file.write_all(data.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
            Ok(_) => println!("Successfully wrote to {}", display),
        }
    }

    fn write_subdomains(&self) {
        println!();

        // get all subdomain sources
        let dns_results = self.get_dns_results();
        
        // combine subdomain source outputs
        let mut res = String::new();
        res.push_str(&dns_results);

        // write all subdomain results into this main file
        self.write("src/output/subdomains.txt", res);

        // write singular results into individual files
        self.write_dns_results();
    }

    fn write_dns_results(&self) {
        let dns_results = self.get_dns_results();
        self.write("src/output/dns.txt", dns_results);
    }

    // ====================================== GET RESULTS ==========================================
    fn get_dns_results(&self) -> String {
        let a_records = display::format_title(
            format!("DNS - A Records ({})", self.dns_records.a_records.0.len()), 
            format!("{}", self.dns_records.a_records.0.join("\n")));
        let ns_records = display::format_title(
            format!("DNS - NS Records ({})", self.dns_records.ns_records.0.len()), 
            format!("{}", self.dns_records.ns_records.0.join("\n")));
        let mx_records = display::format_title(
            format!("DNS - MX Records ({})", self.dns_records.mx_records.0.len()), 
            format!("{}", self.dns_records.mx_records.0.join("\n")));

        let mut res = String::new();
        res.push_str(&a_records);
        res.push_str(&ns_records);
        res.push_str(&mx_records);

        return res;
    }

    fn display_summary(&self) {
        display::display_title("Result", 
            format!("{}\n{}\n{}",
                format!("DNS Records: Found {} subdomain(s) in {:?}", 
                    self.dns_records.get_total_subdomains(), 
                    self.dns_records.get_total_time(),
                ),
                format!("\nTotal subdomains: {}  ", self.dns_records.get_total_subdomains()), 
                format!("Total time taken: {:?}", self.dns_records.get_total_time()),
            )
        );
    }

    // ====================================== HELPER FUNCTIONS ========================================== 
    
}