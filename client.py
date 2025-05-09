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

# Get input from player to show ready

# Send ready

# Await further instructions

# All ready instruction

# Ack

# Game loop

#   Get cards

#   Bet till all pay, check or player folds

#   Reveal card

#   Continue till 5 cards shown

#   Calc who won then everyone get everyone elses' hands shown, expect folded
#   Maybe change in server settings

#   Move bet money accordingly

#   Quit or continue, just go back to 

sock.close()