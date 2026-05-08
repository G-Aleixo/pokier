use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::game::{Card, GameState, GameStatus, PlayerId, PlayerState, PlayerStatus};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct GameStateSnapshot {
    pub deck_size: usize,
    pub river: Vec<Card>,
    pub players: HashMap<PlayerId, PlayerPublic>,
    pub turn_order: Vec<PlayerId>,
    pub current_turn: usize,
    pub game_status: GameStatus,
    pub my_hand: Vec<Card>, // only for the recipient
    pub my_id: PlayerId,
    
    pub min_bet: u32,
    pub pool: u32,
}

impl GameStateSnapshot {
    pub fn from_game_state(state: &GameState, player: &PlayerId) -> GameStateSnapshot {
        GameStateSnapshot {
            deck_size: state.deck.undealt_count(),
            river: state.river.clone(),
            players: state.players.values().map(|player_state| {
                (player_state.id, PlayerPublic {
                    id: player_state.id,
                    hand_size: player_state.hand.len(),
                    status: player_state.status,
                    score: player_state.score,
                    betted: player_state.betted,
                })
            }).collect(),
            turn_order: state.turn_order.clone(),
            current_turn: state.current_turn,
            game_status: state.status,
            my_hand: state.players.get(player).unwrap().hand.clone(),
            my_id: *player,
            min_bet: state.min_bet,
            pool: state.pool,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub struct PlayerPublic {
    pub id: PlayerId,
    pub hand_size: usize,
    pub status: PlayerStatus,
    pub score: u32,
    pub betted: u32,
}

impl PlayerPublic {
    pub fn from_player_state(player: &PlayerState) -> PlayerPublic{
        PlayerPublic {
            id: player.id,
            hand_size: player.hand.len(),
            status: player.status,
            score: player.score,
            betted: player.betted,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    RequestSnapshot,
    PlayerJoin,
    Action(ClientAction),
}

#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum ClientAction {
    /// Bet emcompasses all the actions a player may do that involves chips
    Bet(u32),
    Fold,
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Snapshot(GameStateSnapshot),
    // invalid messages from the client will just be discarded
    // the current model where a whole snapshot is sent means that
    // everyone should be synced up
    // Invalid,
}