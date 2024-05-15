use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn main() -> std::io::Result<()> {
    println!("\nServer starting...\n");
    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;

    // accept connections in a loop
    loop {
        let (mut socket, addr) = listener.accept().await?;     
        println!("Server: Connection from {}", addr);
        
        // spawn async green thread
        tokio::spawn(async move {
            
            // read incoming data and write it
            let mut buf = [0; 1024];
            loop {
                let bytes_read = match socket.read(&mut buf).await {
                    Ok(bytes_read) if bytes_read == 0 => return,
                    Ok(bytes_read) => bytes_read,
                    Err(e) => { eprintln!("Server: failed to read, {}", e); return }
                };

                // write incoming data
                match socket.write_all(&mut buf[0..bytes_read]).await {
                    Ok(_) => (),
                    Err(e) => { eprintln!("Server: failed to write, {}", e); return }
                }
            }
        });
    }
}