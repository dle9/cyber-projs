import socket

IP = "127.0.0.1" 
PORT = 9999
BUF = 1024

# create client socket
client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# connect to the server
client_socket.connect((IP,PORT))

# send messages to the server
msg = input("msg> ")
while msg != "q":
    client_socket.send(msg.encode())
    client_socket.recv(BUF).decode()
    msg = input("msg> ")

# finish
client_socket.close()
print("Goodbye")