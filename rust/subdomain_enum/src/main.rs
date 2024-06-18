mod display;
mod subdomain_source;

type DnsRecord = (Result<(Vec<String>, std::time::Duration), Box<dyn std::error::Error + 'static>>);

#[tokio::main]
async fn main() {
    let target = display::prompt_target_domain();

    // // start dns tasks
    // let a_handle = tokio::task::spawn(subdomain_source::dns::fetch_records(target.clone(), "A"));
    // let ns_handle = tokio::task::spawn(subdomain_source::dns::fetch_records(target.clone(), "NS"));
    // let mx_handle = tokio::task::spawn(subdomain_source::dns::fetch_records(target.clone(), "MX"));

    // // wait for dns threads to finish, join the threads
    // let (a_records, ns_records, mx_records) = tokio::join!(a_handle, ns_handle, mx_handle);
    let (a_records, ns_records, mx_records): (DnsRecord, DnsRecord, DnsRecord) = subdomain_source::dns::fetch_records(target.clone()).await;

    // start subdomain wordlist enumeration
    let wordlist_records: Vec<&str> = subdomain_source::wordlist::fetch_records(target.clone()).await;
    
    // handle result
    let domains = subdomain_source::SubdomainSource {
        dns_records: subdomain_source::dns::DnsRecords {
            a_records: a_records.unwrap(),
            ns_records: ns_records.unwrap(),
            mx_records: mx_records.unwrap(),
        },
        wordlist: subdomain_source::wordlist::Wordlist {
            subdomain_count: 1,
            total_time: std::time::Duration::from_secs(1),
        },
    };

    // write results to src/output/
    domains.write_subdomains();

    // display time taken, subdomains found, etc
    domains.display_summary();

}