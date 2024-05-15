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
