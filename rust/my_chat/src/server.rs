use tokio::io::{ AsyncReadExt,AsyncWriteExt };

pub async fn main(addr: &str) -> std::io::Result<()> { 
    
    // init listening for connections
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("\nServer started on {}\n", addr);

    // create a channel for connected clients
    // transmitter (tx), receiver (rx)
    // max # of msgs is 21
    // let (tx, _) = tokio::sync::broadcast::channel::<Option>(21);

    // accept connections in a loop
    loop {
        // create socket for each client
        let (mut client_socket, client_addr) = listener.accept().await?;     
        println!("Server: Connection from {}", client_addr);
        
        // create transmitter and receiver for each client
        // let tx = tx.clone();    
        // let rx = tx.subscribe();
        
        // create async thread for each client
        tokio::spawn(async move {
            
            // ======== simple echo server ===========
            let mut buf = [0; 1024];
            loop {
                // read incoming data 
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