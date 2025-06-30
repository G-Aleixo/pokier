def get_action() -> str:
    action = " "
    
    print("Your turn has been reached!")
    while not action[0] in "CBPF":
        if action != " ":
            print("Invalid command")
        print("""\
C[all]: Pay current bet
B[et]: Raise current bet by amount
P[ass]: Pass turn
F[old]: Give up currently bet amount and hand""")
        action = input("Action: ").upper()
        
        if not action:
            continue
    
    return action[0]