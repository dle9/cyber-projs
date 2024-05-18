use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn main(addr: &str) -> std::io::Result<()> { 
    
    // connect to the server
    let mut stream = tokio::net::TcpStream::connect(addr).await?;
    println!("\nClient connected to {}\n", addr);

    // write to the server
    let msg = b"Hello, Server!";
    stream.write_all(msg).await?;

    // read server response
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;

    // output server response
    for i in 0..n {
        print!("{}", buf[i] as char);
    } println!();

    Ok(())
}