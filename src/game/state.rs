use std::collections::HashMap;
use crate::game::deck::{Card, Deck};

pub type PlayerId = u8;

#[derive(Debug, Clone)]
pub struct GameState {
    pub deck: Deck,
    pub discarded: Vec<Card>,
    pub players: HashMap<PlayerId, PlayerState>,
    pub turn_order: Vec<PlayerId>,
    pub current_turn: PlayerId,
    pub status: GameStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    WaitingPlayers,
    Dealing,
    InRound,
    GameEnd,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub id: PlayerId,
    pub hand: Vec<Card>,
    pub status: PlayerStatus,
    pub score: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerStatus {
    Playing,
    Folded,
    Disconnected,
}