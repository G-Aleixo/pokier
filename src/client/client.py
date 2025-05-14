import threading
import socket
import pickle, json

import helper
import networking as net

card = tuple[int, int]

sock = net.connect()

idk_name = net.handshake(sock) #TODO: change this name later

if idk_name:
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
        exit()
    
    print("Waiting for others to be ready")
    
    # Wait to get dealt cards...
    cards: list[card] = net.get_cards(sock)
    
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