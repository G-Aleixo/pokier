import socket
import pickle, json
import time

import core

player_count: int = 0

def broadcast_server(broadcasted_address: str, broadcast_port: int, stop_broadcast: list[bool]):

    print("Broadcasting started")

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)

    print("Starting broadcast loop")

    while not stop_broadcast[0]:
        sock.sendto(("PKR BROADCAST:" + broadcasted_address).encode(), ("255.255.255.255", broadcast_port))
        time.sleep(2)
    sock.close()
    print("Broadcasting thread stopped")

def connect_players(server_sock) -> tuple[dict[dict], int]:
    global player_count
    
    clients: dict[dict] = {}
    
    while player_count < core.max_players:
        server_sock.listen()

        addr, port = server_sock.accept()
        print(f"Accepted client at port: {port} at address {addr}")

        data = handshake(addr)

        print(f"Data from client is {data}")

        if data != None:
            clients[addr] = data
            player_count += 1
        else:
            addr.close()
    return clients, player_count

def handshake(addr: socket.socket) -> dict:
    addr.send(b"PKER GAME")

    if addr.recv(1024) == b"YES":
        addr.send(b"CONFIRM")
        
        data = json.loads(addr.recv(2048).decode())
        
        data["name"] = "aaa" #TODO: get from client
        
        return data
    else:
        return None
    
def wait_ready(addr: socket.socket, return_list: list[int, socket.socket], index: int, clients) -> bool:
    # wait to receive ready packet from addr or quit
    data: str = addr.recv(1024).decode()
    
    if data == "READY":
        print(f"Client {clients[addr]['name']} is ready!")
        return_list[index][0] = 1
    elif data == "QUIT":
        print(f"Client {clients[addr]['name']} has quit!")
        return_list[index][0] = -1
    else:
        print(f"client {clients[addr]['name']} has sent not gud data: {data}")
        return_list[index][0] = -2

def send_cards(addr: socket.socket, cards):
    addr.send(pickle.dumps(cards))

def broadcast(clients: list[socket.socket], data, except_addr: socket.socket | None = None):
    #TODO: use threading
    for client in clients:
        if client == except_addr:
            continue
        client.send(data)