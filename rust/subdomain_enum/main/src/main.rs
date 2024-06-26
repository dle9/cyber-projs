// code pulls subdomains from these sources
use subdomain_source::{
    certificate::CertRecords, dns::DnsRecords, wordlist::WordlistRecords, SubdomainSource,
};

// find vulnerabilities for domains
use vulnerability_scan::{PortScan, VulnerabilityScan};

use util; // stdout ops: prompts, result formatting, summary, etc

#[tokio::main]
async fn main() {
    // subdomain enumeration (se) step
    let se_target = util::prompt_target_se();
    let subdomains = run_se_tasks(se_target.clone()).await;

    // display se results
    util::display_title("Subdomain Enumeration Results");
    subdomains.write_all_results();
    subdomains.display_summary();
    
    // vulnerability scan (vs) step
    let vs_target = util::prompt_target_vs();
    let scan = run_vs_tasks(vs_target.clone()).await;

    // display vs results
    util::display_title("Vulnerability Scanner Results");
    scan.write_all_results();
    scan.display_summary();
}

async fn run_se_tasks(target: String) -> SubdomainSource {
    // start subdomain enumeration tasks
    util::display_title("Subdomain Enumeration Tasks");
    let dns_records = DnsRecords::run(target.clone()).await;
    let wordlist_records = WordlistRecords::run(target.clone()).await;
    let certificate_records = CertRecords::run(target.clone()).await;

    // populate subdomain enumeration results
    let subdomains = SubdomainSource {
        dns: dns_records,
        wordlist: wordlist_records,
        certificate: certificate_records,
    };

    return subdomains;
}

async fn run_vs_tasks(target: String) -> VulnerabilityScan {
    util::display_title("Vulnerability Scanning Tasks");
    println!("Starting scan for {}...", target.clone());
    let port_scan_records = PortScan::run(target.clone()).await;

    let scan = VulnerabilityScan {
        port_scan: port_scan_records,
    };

    return scan;
}
