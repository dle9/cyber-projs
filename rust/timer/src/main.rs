use tokio;
async fn foo() -> u8 { 5 }

fn bar() -> impl std::future::Future<Output = u8> {
    async {
        let x: u8 = foo().await;
        x + 5
    }
}


async fn foo2() -> String {
    std::thread::sleep(std::time::Duration::from_secs(3));
    return "bruh".to_string();
}
fn bar2() -> impl std::future::Future<Output = String> {
    async {
        let bruh = foo2().await;
        bruh
    }
}

#[tokio::main]
async fn main() {
    let x = bar().await;
    let bruh = bar2().await;
    
    println!("{}", bruh);
    println!("{}", x);
}