import sys
import signal
import socket

IP = "127.0.0.1" 
PORT = 9999
BUF = 1024

# get user inputted port
if len(sys.argv) == 2:
    PORT = int(sys.argv[1])

# create client socket
client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# connect to the server
client_socket.connect((IP,PORT))

# send messages to the server
while True:
    msg = input("msg> ")
    client_socket.send(msg.encode())
    client_socket.recv(BUF).decode()
    if msg == "q":
        break

# finish
client_socket.close()
print("Goodbye")