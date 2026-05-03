use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::game::deck::{Card, Deck};

pub type PlayerId = uuid::Uuid;


#[derive(Debug, Clone)]
pub struct GameState {
    pub deck: Deck,
    pub river: Vec<Card>,
    pub discarded: Vec<Card>,
    pub players: HashMap<PlayerId, PlayerState>,
    pub turn_order: Vec<PlayerId>,
    pub current_turn: usize,
    pub status: GameStatus,

    pub min_bet: u32,
    pub last_action: Option<usize>,
    pub resolved: bool,
    pub pool: u32,

    pub dirty: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            deck: Deck::new(),
            river: vec![],
            discarded: vec![],
            players: HashMap::new(),
            turn_order: vec![],
            current_turn: 0,
            status: GameStatus::WaitingPlayers,
            dirty: false,
            last_action: None,
            resolved: false,
            min_bet: 0,
            pool: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum GameStatus {
    WaitingPlayers,
    InRound,
    FinalBet,
    GameEnd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerState {
    pub id: PlayerId,
    pub hand: Vec<Card>,
    pub status: PlayerStatus,
    pub score: u32,
    pub betted: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum PlayerStatus {
    Playing,
    Waiting,
    Folded,
    Disconnected, // a disconnected player is the same as a folded one
}