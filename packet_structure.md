# Packet structure

## Data "types"

- card: two ints, first is rank, 2-A, second is suit, spades, hearts, clubs, diamonds

## Packets

- packet_name: SC or CS - incode_name - contained data - additional comment
- deal cards: SC - DEAL - 4 ints representing 2 card data types
- ready: CS - READY - READY
- quit: CS - CLIENT_QUIT - QUIT
- game_start: SC - START - START
- play: SC - TURN - TURN - Indicates the player's turn has been reached
- skip: SC - SKIP - SKIP - Skipped player because they have folded or other circunstances
- call: CS - CLIENT_CALL - CLIENT_CALL
- call: SC - PLAYER_CALL - PLAYER_CALL then player_id/name that indicates that that player has called
- bet: CS - CLIENT_BET - CLIENT_BET then int representing amount to bet addicionally to already bet amount, -1 for all in
- bet: SC - PLAYER_BET - PLAYER_BET then player_id/name then an int about how much they bet, -1 for all in
- check: CS - CLIENT_CHECK - CLIENT_CHECK
- check: SC - PLAYER_CHECK - PLAYER_CHECK then player_id/name about who checked
- fold: CS - CLIENT_FOLD - CLIENT_FOLD
- fold: SC - PLAYER_FOLD - PLAYER_FOLD then player_id/name who has folded
- invalid: SC - INVALID - INVALID - Indicates that the move the client sent is invalid
