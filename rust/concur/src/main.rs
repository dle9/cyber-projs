mod dns;

// run multiple times to see different outputs 

#[tokio::main]
async fn main() {
    // start tasks
    let a_handle = tokio::task::spawn(dns::get_dns_record1());
    let b_handle = tokio::task::spawn(dns::get_dns_record2());
    let c_handle = tokio::task::spawn(dns::get_dns_record3());

    // wait for all three tasks to complete, then join them    
    let (a_records, ns_records, mx_records) = tokio::join!(a_handle, b_handle, c_handle);
}
