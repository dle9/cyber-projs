use tokio::io::{ AsyncReadExt,AsyncWriteExt };

pub async fn main(addr: &str) -> std::io::Result<()> { 
    
    // init listening for connections
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("\nServer started on {}\n", addr);

    // accept connections in a loop
    loop {
        let (mut client_socket, addr) = listener.accept().await?;     
        println!("Server: Connection from {}", addr);
        
        // create async thread for each client
        tokio::spawn(async move {
            
            let mut buf = [0; 1024];
            loop {
                
                // read incoming data and write it
                let bytes_read = match client_socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => { println!("Server: Read {} bytes.", n); n},
                    Err(e) => { eprintln!("Failed to read, {}", e); return }
                };

                // write received data
                if bytes_read > 0 {
                    println!("Server: Writing {} bytes.", bytes_read);
                    match client_socket.write_all(&buf[0..bytes_read]).await {
                        Ok(()) => (),
                        Err(e) => { eprintln!("Server: Failed to write, {}", e); return }
                    }
                }
            }
        });
    } // end of server loop
}