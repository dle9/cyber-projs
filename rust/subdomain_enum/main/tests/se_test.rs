// testing subdomain enumeration tasks

use subdomain_source::{
    certificate::CertRecords, dns::DnsRecords, wordlist::WordlistRecords
};

#[tokio::test]
async fn dns_tasks() {
    let dns_records = DnsRecords::run("spglobal.com".to_string()).await;
    assert_eq!(dns_records.get_total_subdomains(), 16);
}

#[tokio::test]
async fn wordlist_tasks() {
    let wordlist_records = WordlistRecords::run("spglobal.com".to_string()).await;
     assert_eq!(wordlist_records.subdomains.len(), 2);
}

#[tokio::test]
async fn certificate_tasks() {
    let certificate_records = CertRecords::run("spglobal.com".to_string()).await;
    assert_eq!(certificate_records.subdomains.len(), 0);
}

