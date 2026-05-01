use std::{collections::HashMap, hash::Hash};

use pokier::game::{ClientMessage, Deck, GameServer, GameState, GameStatus};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(16);
    let mut server = GameServer {
        state: GameState {
            deck: Deck::new(),
            discarded: vec![],
            players: HashMap::new(),
            turn_order: vec![],
            current_turn: 0,
            status: GameStatus::WaitingPlayers
        },
        incoming: rx,
        outgoing: HashMap::new()
    };
    println!("{server:#?}");
    
    server.run().await;
    println!("hello world!");
}