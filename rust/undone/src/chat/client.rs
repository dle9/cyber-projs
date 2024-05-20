use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
pub async fn main() -> std::io::Result<()> { 
    let server_addr = "localhost:8080";

    // connect to the server
    let mut stream = tokio::net::TcpStream::connect(server_addr).await?;
    println!("\nClient connected to {}\n", server_addr);

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