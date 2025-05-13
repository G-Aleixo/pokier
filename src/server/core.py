import random

card = tuple[int, int]


max_players: int = 2

chips_for_player: int = 100


# value is if the card has been dealt
cards: list[bool] = [[True for _ in range(4)] for _ in range(13)]

def deal_cards(player_amount: int) -> tuple[list[tuple[card, card]], list[card]]:
    card_list: list[card] = [(rank, suit) for rank in range(13) for suit in range(4)]
    random.shuffle(card_list) # in random we trust
    
    discard: list[card] = []
    
    player_hands: list[list[card]] = [[] for _ in range(player_amount)]
    
    for i in range(player_amount * 2):
        dealt_card: card = card_list.pop(0) 
        player_hands[i % player_amount].append(dealt_card)
        cards[dealt_card[0]][dealt_card[1]] = False
        
        discarted: card = card_list.pop(0) 
        discard.append(discarted)
        cards[discarted[0]][discarted[1]] = False
    
    return player_hands, discard