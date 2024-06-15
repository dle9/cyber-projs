// file containing all sources of subdomains for a target

pub struct DnsRecords {
    pub dns_a: (Vec<String>, std::time::Duration),
    pub dns_ns: (Vec<String>, std::time::Duration),
    pub dns_mx: (Vec<String>, std::time::Duration),
}