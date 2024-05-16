# Server
---
### Step 1. Create the listener
	- tokio::net::TcpListener::bind()
### Step 2. Accept connections
	- infinite loop
	- (socket, addr) = 	listener.accept()
### Step 3. spawn async threads for each client
	- tokio::spawn(async move {})
### Step 4. Read client msgs in each thread
	- infinite loop
	- bytes_read = socket.read(buf)
### Step 5. Echo msg back to client
	- socket.write_all(buf)

# Client
---
### Step 1. Connect to the server
	- stream = tokio::net::TcpStream::connect(addr)
### Step 2. Write to the server
	- stream.write_all(msg)
### Step 3. Read a response
	- stream.read(buf)

# Broadcast channel
---
### Setup broadcast channel in server to see all client messages
	- let (tx,rx)=tokio::sync::broadcast::channel()
### send data to channel
	- send the value 10
	- tx.send(10).unwrap();
### receive same data sent by transmitters
	- rx.recv.await.unwrap()
### add more receivers 
	- rx = tx.subscribe