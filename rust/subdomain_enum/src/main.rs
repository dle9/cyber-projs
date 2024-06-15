mod dns;
mod subdomain_source;
mod prompt;

#[tokio::main]
async fn main() {
    let target = prompt::target_domain();

    // start dns tasks
    let a_handle = tokio::task::spawn(dns::get_dns_record(target.clone(), "A"));
    let ns_handle = tokio::task::spawn(dns::get_dns_record(target.clone(), "NS"));
    let mx_handle = tokio::task::spawn(dns::get_dns_record(target.clone(), "MX"));

    // wait for dns tasks to finish, join the threads
    let (dns_a, dns_ns, dns_mx) = tokio::join!(a_handle, ns_handle, mx_handle);

    // handle result
    let domains = Subdomains {
        dns_records: subdomain_source::DnsRecords {
            dns_a: dns_a.unwrap(),
            dns_ns: dns_ns.unwrap(),
            dns_mx: dns_mx.unwrap(),
        },
    };

    domains.display();
    domains.display_total_fetch_time();
}

// contains all domain sources
struct Subdomains {
    dns_records: subdomain_source::DnsRecords,
}

impl Subdomains {
    fn display(&self) {
        prompt::print_title("DNS - A Records", format!("{}", self.dns_records.dns_a.0.join("\n")));
        prompt::print_title("DNS - NS Records", format!("{}", self.dns_records.dns_ns.0.join("\n")));
        prompt::print_title("DNS - MX Records", format!("{}", self.dns_records.dns_mx.0.join("\n")));
    }

    fn display_total_fetch_time(&self) {
        prompt::print_title("Time taken", 
            format!("{}\n{}",
                format!("DNS records: {:?}", self.sum_dns_fetch_time()),
                format!("Total time: {:?}", self.sum_dns_fetch_time()),
            )
        );
    }

    fn sum_dns_fetch_time(&self) -> std::time::Duration {
        return self.dns_records.dns_a.1 + self.dns_records.dns_ns.1 + self.dns_records.dns_mx.1; 
    }
}