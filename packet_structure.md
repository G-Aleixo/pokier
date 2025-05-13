# Packet structure

## Data "types"

- card: two ints, first is rank, 2-A, second is suit, spades, hearts, clubs, diamonds

## Packets

- packet_name: SC or CS - incode_name - contained data - additional comment
- deal cards: SC - DEAL - 4 ints representing 2 card data types
- ready: CS - READY - READY
- quit: CS - CLIENT_QUIT - QUIT
- game_start: SC - START - START
- play: CS - MY_TURN - MY_TURN - Indicates the player's turn has been reached
- call: CS - CLIENT_CALL - CLIENT_CALL
- call: SC - PLAYER_CALL - player_id/name that indicates that that player has called
- bet: CS - CLIENT_BET - int representing amount to bet addicionally to already bet amount, -1 for all in
- bet: SC - PLAYER_BET - player_id/name and an int about how much they bet, -1 for all in
- fold: CS - CLIENT_FOLD - CLIENT_FOLD
- fold: SC - PLAYER_FOLD - player_id/name who has folded
