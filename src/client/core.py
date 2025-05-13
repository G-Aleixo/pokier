def get_action() -> str:
    action = " "
    
    print("Your turn has been reached!")
    while not action[0] in "CRPF":
        if action != " ":
            print("Invalid command")
        print("""\
C[ll]: Pay current bet
R[aise]: Raise current bet by amount
P[ass]: Pass turn
F[old]: Give up currently bet amount and hand""")
        action = input("Action: ").upper()
        
        if not action:
            continue
    
    return action[0]