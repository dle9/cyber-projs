import sys
import signal
import socket
import threading
import logging

logging.basicConfig(level=logging.INFO)

#####################
#   server config   #
#####################
IP = "127.0.0.1" 
PORT = 9999
BUF = 1024
server_threads = []
stop_server = threading.Event()


#####################
#  user input port  #
#####################
if len(sys.argv) == 2:
    PORT = int(sys.argv[1])

def handle_client(client_socket, client_addr):
    '''handles client connection state'''
    logging.info(f"CONNECTED: {client_addr}")
    print(f"\nCONNECTED: {client_addr}")
    
    try:
        while True:
            msg = client_socket.recv(BUF).decode()
            if msg == "q" or not msg:
                break

            print(f"MESSAGE: {client_addr}> {msg}")
            client_socket.send(msg.encode())  # echo message back to client
            
    except Exception as e:
        print(f"CLIENT EXCEPTION: {client_addr}: {e}")
    
    finally:
        client_socket.close()
        print(f"DISCONNECTED: {client_addr}")

def close_server(signal_received, frame):
    '''catch and handle SIGINT signal'''
    logging.info("Server shutting down...")
    stop_server.set()  # signal all threads to stop

    # close all server threads
    for t in server_threads:
        t.join()

    server_socket.close()
    logging.info("Server closed")
    sys.exit(0)

#####################
# main server loop #
#####################
# SIGINT signal is caught and handled by close_server()
signal.signal(signal.SIGINT, close_server)

# create server socket
server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_socket.bind((IP,PORT))
server_socket.listen(5)
logging.info(f"Server listening on {IP}:{PORT}\n")

while not stop_server.is_set():
    # accept client connections
    try:
        client_socket, client_addr = server_socket.accept()
        t = threading.Thread(target=handle_client, args=(client_socket, client_addr))
        server_threads.append(t)
        t.start()

    # handle SIGINT signal
    except Exception as e:
        if stop_server.is_set():
            break

# finish
close_server(None, None)
