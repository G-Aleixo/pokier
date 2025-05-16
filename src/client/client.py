import threading
import socket
import pickle, json

import helper
import networking as net
import core

card = tuple[int, int]

sock = net.connect()

#TODO: Add server selection later

accepted = net.handshake(sock) #TODO: change this name later

if not accepted:
    print("Was not accepted into server for reason: IMPLEMENT LATER")
    sock.close()
    print("Closing game...")
    exit()

print("Connnected")

print("""\
Options:
R[eady]
Q[uit]: """)

data: str = input().upper()

if data.startswith("R"):
    sock.send(b"READY")
elif data.startswith("Q"):
    sock.send(b"QUIT")
    sock.close()
    print("Closing game...") #TODO: Add select to go to another server or quit out of game
    exit()
    
print("Waiting for others to be ready")

# Wait to get dealt cards...
cards: list[card] = net.get_cards(sock)

print("Dealt cards:")
helper.print_cards(cards)


while True:
    # Wait till my turn
    data = sock.recv(1024).decode()
    
    if data.find("TURN") != -1:
        action: str = core.get_action()
        
        if action == "B":
            value = input("How much to bet?: ") #TODO: verify number
            sock.send(b"CLIENT_BET")
            sock.send(value)
        elif action == "C":
            sock.send(b"CLIENT_CALL")
        elif action == "P":
            sock.send(b"CLIENT_CHECK")
        elif action == "F":
            sock.send(b"CLIENT_FOLD")
        else:
            print(f"Unknown action: {action}")
    elif data.startswith("PLAYER_"):
        # May be BET, CALL or FOLD
        # TODO: to something with it later
        
        name = sock.recv(1024).decode()
        
        if data.endswith("BET"):
            amount = sock.recv(1024)
            print(f"{name} has BET {amount}")
        elif data.endswith("CALL"):
            print(f"{name} has called")
        elif data.endswith("CHECK"):
            print(f"{name} has checked")
        elif data.endswith("FOLD"):
            print(f"{name} has folded")
        else:
            print(f"Unknown data: {data}")

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