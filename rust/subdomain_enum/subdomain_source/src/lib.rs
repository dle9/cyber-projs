// file operations
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// subdomain sources
pub mod dns;
use dns::DnsRecords;
pub mod wordlist;
use wordlist::WordlistRecords;
pub mod certificate;
use certificate::CertRecords;

use util;

// contains all domain sources
pub struct SubdomainSource {
    pub dns: DnsRecords,
    pub wordlist: WordlistRecords,
    pub certificate: CertRecords,
}

impl SubdomainSource {
    // ====================================== WRITE FUNCTIONS ==========================================
    pub fn write_all_results(&self) {
        // get+format data from all subdomain sources
        let dns_results = self.get_dns_results();
        let wordlist_results = self.get_wordlist_results();

        // combine subdomain source outputs
        let mut all_results = String::new();
        all_results.push_str(&dns_results);
        all_results.push_str(&wordlist_results);

        // write all subdomain results into this main file
        self.write("src/output/subdomains.txt", all_results);

        // write singular results into individual files
        self.write_dns_results();
        self.write_wordlist_results();
    }

    pub fn write_dns_results(&self) {
        let dns_results = self.get_dns_results();
        self.write("src/output/dns.txt", dns_results);
    }

    pub fn write_wordlist_results(&self) {
        let wordlist_results = self.get_wordlist_results();
        self.write("src/output/wordlist.txt", wordlist_results);
    }

    pub fn write(&self, filepath: &str, data: String) {
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

    // ====================================== GET+FORMAT RESULTS ==========================================
    pub fn get_dns_results(&self) -> String {
        let a_records = util::format_title(
            format!("DNS - A Records ({})", self.dns.a_records.0.len()),
            format!("{}", self.dns.a_records.0.join("\n")),
        );
        let ns_records = util::format_title(
            format!("DNS - NS Records ({})", self.dns.ns_records.0.len()),
            format!("{}", self.dns.ns_records.0.join("\n")),
        );
        let mx_records = util::format_title(
            format!("DNS - MX Records ({})", self.dns.mx_records.0.len()),
            format!("{}", self.dns.mx_records.0.join("\n")),
        );

        let mut res = String::new();
        res.push_str(&a_records);
        res.push_str(&ns_records);
        res.push_str(&mx_records);

        return res;
    }

    pub fn get_wordlist_results(&self) -> String {
        let res = util::format_title(
            format!(
                "Successful wordlist connections ({})",
                self.wordlist.subdomains.len()
            ),
            format!("{}", self.wordlist.subdomains.join("\n")),
        );

        return res;
    }

    pub fn display_summary(&self) {
        print!(
            "{}\n{}\n{}\n{}",
            // individual results
            format!(
                "DNS records: Found {} subdomain(s) in {:?}",
                self.dns.get_total_subdomains(),
                self.dns.get_total_time(),
            ),
            format!(
                "Wordlist records: Found {} subdomain(s) in {:?}",
                self.wordlist.subdomains.len(),
                self.wordlist.total_time,
            ),
            // total results
            format!(
                "Total subdomains found: {} subdomains",
                self.get_total_subdomains()
            ),
            format!("Total time taken: {:?}", self.get_total_time()),
        );
    }

    // ========================================= HELPER FUNS =========================================
    fn get_total_subdomains(&self) -> usize {
        return self.dns.get_total_subdomains()
            + self.wordlist.subdomains.len()
            + self.certificate.subdomains.len();
    }

    fn get_total_time(&self) -> std::time::Duration {
        return self.dns.get_total_time() + self.wordlist.total_time + self.certificate.total_time;
    }
}
