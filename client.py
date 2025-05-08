import threading
import socket

def handshake(addr: socket.socket) -> dict:

    if addr.recv(1024) == b"PKER GAME":
        addr.send(b"YES")
        if addr.recv(1024) == b"CONFIRM":
            return True
    else:
        return False
    

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

address = "127.0.0.1"
port = 50433

clients = {}

sock.connect((address, port))

if handshake(sock):
    print("Connnected")

sock.close()