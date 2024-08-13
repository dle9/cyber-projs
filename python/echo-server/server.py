import socket 
import logging

logging.basicConfig(level=logging.INFO)

IP = "127.0.0.1" 
PORT = 9999
BUF = 1024

# create server socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind((IP,PORT))
server_socket.listen(2)
logging.info(f"Server listening on {IP}:{PORT}\n")

# get client info
client_socket, client_addr = server_socket.accept()
logging.info(f"Connection from {client_addr}")

msg = client_socket.recv(BUF).decode()
while msg != "q":
    print(f"**Message from {client_addr}: {msg}")
    client_socket.send(msg.encode()) # echo msg back to client
    msg = client_socket.recv(BUF).decode()

# finish
server_socket.close()
logging.info(f"Server closed\n")
