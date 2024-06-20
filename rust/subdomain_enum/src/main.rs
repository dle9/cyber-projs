mod subdomain_source; // code pulls subdomains from these sources
use crate::subdomain_source::{SubdomainSource, dns::DnsRecords, wordlist::Wordlist, certificate::Certificate}; 

mod display; // stdout ops: prompts, result formatting, summary, etc

#[tokio::main]
async fn main() {
    // get domain to target/enumerate/scan
    let target = display::prompt_target_domain();

    // start subdomain enumeration tasks
    let dns_records = DnsRecords::run(target.clone()).await;
    let wordlist_records = Wordlist::run(target.clone()).await;
    let certificate_records = Certificate::run(target.clone()).await;

    // populate subdomain enumeration results
    let domains = SubdomainSource {
        dns_records: dns_records,
        wordlist: wordlist_records,
        certificate: certificate_records,
    };

    // write results to src/output/
    domains.write_all_results();

    // display time taken, subdomains found, etc at a high level
    domains.display_summary();
}
