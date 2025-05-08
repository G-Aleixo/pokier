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
card = [[True for _ in range(4)] for _ in range(13)]

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