card = tuple[int, int]

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
    match suit: #TODO: Add support to unicode chars later
        case 0:
            card += "S"# chr(9828)
        case 1:
            card += "H"# chr(9829)
        case 2:
            card += "C"# chr(9830)
        case 3:
            card += "D"# chr(9831)
    
    return card

def print_cards(cards_list: list[card]):
    for card in cards_list:
        print(index2card(card[0], card[1]), end=" ")
    print()