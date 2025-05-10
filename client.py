import threading
import socket
import pickle

import helper

card = tuple[int, int]

BROADCAST_PORT = 54432

def get_server_ip(broadcast_port: int):
    sock: socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    sock.bind(("0.0.0.0", broadcast_port))

    data, addr = sock.recvfrom(1024)

    while not data.decode().startswith("PKR BROADCAST"):
        data, addr = sock.recvfrom(1024)

    print(f"Got broadcast from {addr}")
    print(f"Got data: {data}")
    print(f"Returning sent ip: {data.decode()[14:]}")
    print("Closing getting ip socket function")
    sock.close()
    return data.decode()[14:]
 
def get_cards(server: socket.socket) -> tuple[list[int, int], list[int, int]]:
    return pickle.loads(server.recv(1024))

def handshake(addr: socket.socket) -> bool:
    if addr.recv(1024) == b"PKER GAME":
        addr.send(b"YES")
        if addr.recv(1024) == b"CONFIRM":
            return True
    else:
        return False

sock: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

address: str = get_server_ip(BROADCAST_PORT) #"127.0.0.1"
port: int = 50433

print(f"Connecting to server at ip: {address}")

sock.connect((address, port))

if handshake(sock):
    print("Connnected")
    
    print("""\
Options:
R[eady]
Q[uit]: """)
    
    data: str = input()
    
    if data.startswith("R"):
        sock.send(b"READY")
    elif data.startswith("Q"):
        sock.send(b"QUIT")
        sock.close()
        exit()
    
    print("Waiting for others to be ready")
    
    # Wait to get dealt cards...
    cards: list[card] = get_cards()
    
    print("Dealt cards:")
    helper.print_cards(cards)

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