import threading
import random, time
import socket
import pickle, json


import networking as net
import core
import helper


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


sock: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

port: int = 50433
addressess = socket.getaddrinfo(socket.gethostname(), port)

for i in range(len(addressess)):
    print(f"{i}: {addressess[i]}")
choice = int(input("Choose which addr to use"))

address = addressess[choice][-1][0]

BROADCASTING_PORT = 54432

print(f"Binding socket to {address}:{port}")
sock.bind((address, port))

print("Starting broadcasting thread")
stop_broadcast = [False]
threading.Thread(target=net.broadcast_server, args=(address, BROADCASTING_PORT, stop_broadcast)).start()

print("Listening for players")

clients, player_count = net.connect_players(sock)


print("All clients loaded")
print("Stopping broadcasting thread")
stop_broadcast[0] = True
print("Awaiting all clients to get ready...")

# 0 no response, 1 ready, -1 quit, -2 is error
results: list[list[int, socket.socket]] = []

threads: list[threading.Thread] = []
index: int = 0
for addr in clients:
    results.append([0, addr])
    threads.append(threading.Thread(target=net.wait_ready, args=(addr, results, index, clients)))
    
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
cards, discarted = core.deal_cards(player_count)

for hand in cards:
    helper.print_cards(hand)
helper.print_cards(discarted)

# This may have some bad implications with bad ping
# I'll deal with it in the future :) - 09/05/2025
i = 0
for conn in clients:
    net.send_cards(conn, cards[i])
    i += 1


print("closing all connections")
for conn in clients:
    conn.close()


sock.close()