import threading
import random, pickle, time
import socket

import helper

card = tuple[int, int]

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
cards: list[bool] = [[True for _ in range(4)] for _ in range(13)]


def deal_cards(player_amount: int) -> tuple[list[tuple[card, card]], list[card]]:
    card_list: list[card] = [(rank, suit) for rank in range(13) for suit in range(4)]
    random.shuffle(card_list) # in random we trust
    
    discard: list[card] = []
    
    player_hands: list[list[card]] = [[] for _ in range(player_amount)]
    
    for i in range(player_amount * 2):
        player_hands[i % player_amount].append(card_list.pop(0))
        discard.append(card_list.pop(0))
    
    return player_hands, discard

def send_cards(addr: socket.socket, cards):
    addr.send(pickle.dumps(cards))

def broadcast_server(broadcast_port: int):
    global stop_broadcast

    print("Broadcasting started")

    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)

    print("Stating broadcast loop")

    while not stop_broadcast:
        sock.sendto(("PKR BROADCAST:" + str(socket.gethostbyname(socket.gethostname()))).encode(), ("255.255.255.255", broadcast_port))
        time.sleep(2)
    sock.close()
    print("Broadcasting thread stopped")

def handshake(addr: socket.socket) -> dict:
    # mock data

    data: dict = {"name": player_count}

    addr.send(b"PKER GAME")

    if addr.recv(1024) == b"YES":
        addr.send(b"CONFIRM")
        return data
    else:
        return None

def wait_ready(addr: socket.socket, return_list: list[int, socket.socket], index: int) -> bool:
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
    


sock: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

port: int = 50433
address = socket.gethostbyname(socket.gethostname())
BROADCASTING_PORT = 54432

max_players: int = 1
player_count: int = 0

clients: dict[dict] = {}

print(f"Binding socket to {address}:{port}")
sock.bind((address, port))

print("Starting broadcasting thread")
stop_broadcast = False
threading.Thread(target=broadcast_server, args=(BROADCASTING_PORT,)).start()

print("Listening for players")

while player_count < max_players:
    sock.listen()

    addr, port = sock.accept()
    print(f"Accepted client at port: {port} at address {addr}")

    data = handshake(addr)

    print(f"Data from client is {data}")

    if data != None:
        clients[addr] = data
        player_count += 1

print("All clients loaded")
print("Stopping broadcasting thread")
stop_broadcast = True
print("Awaiting all clients to get ready...")

# 0 no response, 1 ready, -1 quit, -2 is error
results: list[list[int, socket.socket]] = []

threads: list[threading.Thread] = []
index: int = 0
for addr in clients:
    results.append([0, addr])
    threads.append(threading.Thread(target=wait_ready, args=(addr, results, index)))
    
    index += 1
    
for thread in threads:
    print(f"Starting waiting thread: {thread.name}")
    thread.start()

print("Waiting for responses...")

all_responded: bool = False
while not all_responded: # Wait till all the results are not 0
    all_responded = True
    for result in results:
        if not result[0]:
            all_responded = False
            

print(results)

print("All responses gotten!")
for result in results:
    if result[0] == -1:
        print(f"Client {clients[result[1]]['name']} has quit")
        print(f"Removing client {clients[result[1]]['name']} from clients and closing sock")
        result[1].close()
        clients.pop(result[1])
        player_count -= 1
    elif result[0] == 1:
        print(f"Client {clients[result[1]]['name']} is ready")
    elif result[0] == -2:
        print(f"Client {clients[result[1]]['name']} has errored")
        

print("Dealing cards")
# deal cards
cards, discarted = deal_cards(player_count)

for hand in cards:
    helper.print_cards(hand)
helper.print_cards(discarted)

i = 0
for conn in clients:
    send_cards(conn, cards[i])
    i += 1


print("closing all connections")
for conn in clients:
    conn.close()


sock.close()