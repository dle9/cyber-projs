// testing subdomain enumeration tasks

use vulnerability_scan::{PortScan};

#[tokio::test]
async fn port_scan_tasks() {
    let scan = PortScan::run("spglobal.com".to_string()).await;
    assert_eq!(scan.ports.len(), 0);
}
