import threading
import socket

# []
# 2
# 3
# 4
# 5
# 6
# 7
# 8
# 9
# 10
# J
# Q
# K
# A

# [][]
# spades
# hearts
# clubs
# diamonds

# value is if the card has been dealt
cards = [[True for _ in range(4)] for _ in range(13)]

def index2card(index: int, suit: int) -> str:
    card = ""
    
    match index:
        case 0:
            card += "2"
        case 1:
            card += "3"
        case 2:
            card += "4"
        case 3:
            card += "5"
        case 4:
            card += "6"
        case 5:
            card += "7"
        case 6:
            card += "8"
        case 7:
            card += "9"
        case 8:
            card += "10"
        case 9:
            card += "J"
        case 10:
            card += "Q"
        case 11:
            card += "K"
        case 12:
            card += "A"
    match suit:
        case 0:
            card += "S"
        case 1:
            card += "H"
        case 2:
            card += "C"
        case 3:
            card += "D"
    
    return card


def handshake(addr: socket.socket) -> dict:
    # mock data

    data = {"name": player_count}

    addr.send(b"PKER GAME")

    if addr.recv(1024) == b"YES":
        addr.send(b"CONFIRM")
        return data
    else:
        return None
    

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

address = "127.0.0.1"
port = 50433

max_player = 2
player_count = 0

clients = {}

sock.bind((address, port))

while player_count < max_player:
    sock.listen()

    addr, port = sock.accept()
    print(f"Accepted client at port: {port} at address {addr}")

    data = handshake(addr)

    print(f"Data from client is {data}")

    if data != None:
        clients[addr] = data
        player_count += 1

print("All clients loaded")

for conn in clients:
    conn.close()


sock.close()