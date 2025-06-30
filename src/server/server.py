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
choice = int(input("Choose which addr to use: "))

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

betting_done: bool = False
has_raised: bool = False
last_raise: int = -1
bet_amount: list[int] = [0 for _ in range(player_count)]
player_index: int = 0

folded_list: list[int] = []

while betting_done != True:#core.max_players:
    conn: socket.socket = list(clients.keys())[player_index%player_count]
    
    # Stop when
    # No one bet and reached final player
    # Reached last_raise
    # 

    if player_index%last_raise == 0 and last_raise != -1: # Reached person who bet after other called/folded
        betting_done = True
        break
    elif last_raise == -1 and player_index == player_count and not has_raised: # No one bet
        betting_done = True
        break

    # if player_index%player_count in folded_list:
    #     conn.send(b"SKIP")
    #     player_index += 1
    #     continue # Go to the next player
    conn.send(b"TURN")
    
    data = conn.recv(1024).decode()
    
    if data == "CLIENT_CALL":
        bet_amount[player_index%player_count] = bet_amount[(player_index - 1)%player_count]

        net.broadcast(clients.keys(), b"PLAYER_CALL", except_addr=conn)
        net.broadcast(clients.keys(), clients[conn]["name"].encode(), except_addr=conn)
    elif data == "CLIENT_CHECK":
        if last_raise != -1:
            ... #TODO: send invalid
        net.broadcast(clients.keys(), b"PLAYER_CHECK", except_addr=conn)
        net.broadcast(clients.keys(), clients[conn]["name"].encode(), except_addr=conn)
    elif data == "CLIENT_BET":
        amount = conn.recv(1024)

        #TODO: Verify amount
        has_raised = True
        bet_amount[player_index%player_count] = bet_amount[(player_index - 1)%player_count] + amount

        net.broadcast(clients.keys(), b"PLAYER_BET", except_addr=conn)
        net.broadcast(clients.keys(), clients[conn]["name"].encode(), except_addr=conn)
        net.broadcast(clients.keys(), amount, except_addr=conn)
    elif data == "CLIENT_FOLD":
        # Register fold to folded_list
        folded_list.append(player_index%player_count)

        net.broadcast(clients.keys(), b"PLAYER_FOLD", except_addr=conn)
        net.broadcast(clients.keys(), clients[conn]["name"].encode(), except_addr=conn)
        
    
    player_index += 1

print("closing all connections")
for conn in clients:
    conn.close()


sock.close()