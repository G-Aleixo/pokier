use serde::{Deserialize, Serialize};

use crate::game::{Card, GameState, GameStatus, PlayerId, PlayerState, PlayerStatus};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct GameStateSnapshot {
    deck_size: usize,
    river: Vec<Card>,
    players: Vec<PlayerPublic>,
    turn_order: Vec<PlayerId>,
    current_turn: usize,
    game_status: GameStatus,
    my_hand: Vec<Card>, // only for the recipient
}

impl GameStateSnapshot {
    pub fn from_game_state(state: &GameState, player: &PlayerId) -> GameStateSnapshot {
        GameStateSnapshot {
            deck_size: state.deck.undealt_count(),
            river: state.river.clone(),
            players: state.players.values().map(|player_state| {
                PlayerPublic {
                    id: player_state.id,
                    hand_size: player_state.hand.len(),
                    status: player_state.status
                }
            }).collect(),
            turn_order: state.turn_order.clone(),
            current_turn: state.current_turn,
            game_status: state.status,
            my_hand: state.players.get(player).unwrap().hand.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct PlayerPublic {
    id: PlayerId,
    hand_size: usize,
    status: PlayerStatus
}

impl PlayerPublic {
    pub fn from_player_state(player: &PlayerState) -> PlayerPublic{
        PlayerPublic {
            id: player.id,
            hand_size: player.hand.len(),
            status: player.status
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    RequestSnapshot,
    PlayerJoin,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Snapshot(GameStateSnapshot)
}