# Server
---
### Step 1. Create the listener
	- tokio::net::TcpListener::bind()
### Step 2. Accept connections
	- infinite loop
	- (socket, addr) = listener.accept()
### Step 3. spawn async threads for each client
	- tokio::spawn(async move {})
### Step 4. Read client msgs in each thread
	- infinite loop
	- bytes_read = socket.read(buf)
### Step 5. Write client msgs
	- socket.write_all(buf)

# Client
---
 