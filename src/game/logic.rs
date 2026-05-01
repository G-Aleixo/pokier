use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::game::{Deck, GameStateSnapshot, GameStatus, PlayerStatus, message::{ClientMessage, ServerMessage}, state::{GameState, PlayerId}};

#[derive(Debug)]
pub struct GameServer {
    pub state: GameState,
    pub incoming: mpsc::Receiver<ClientMessage>,
    pub outgoing: HashMap<PlayerId, mpsc::Sender<ServerMessage>>,
}

impl GameServer {
    pub async fn run(&mut self) {
        while let Some(msg) = self.incoming.recv().await {
            self.handle_message(msg).await;

            if self.state.status == GameStatus::GameEnd {
                self.reset_state();
            }
        }
    }

    pub async fn handle_message(&mut self, msg: ClientMessage) {
        match msg {
            ClientMessage::RequestSnapshot => {
                
            }
        }
    }

    pub async fn broadcast(&mut self, msg: &ServerMessage) {
        let mut failed = vec![];

        for (player_id, tx) in &self.outgoing {
            if let Err(e) = tx.send(msg.clone()).await {
                eprintln!("Failed to send to player id {player_id}: {e}");

                failed.push(*player_id);
            }
        }

        for player_id in failed {
            self.mark_disconnected(player_id);
        }
    }

    pub async fn send_to(&mut self, msg: &ServerMessage, player: &PlayerId) -> Result<(), mpsc::error::SendError<ServerMessage>> {
        if let Some(tx) = self.outgoing.get(player) {
            tx.send(msg.clone()).await
        } else {
            Ok(())
        }
    }

    async fn broadcast_state(&mut self) {
        let mut failed = vec![];
        for player_id in self.outgoing.keys().cloned().collect::<Vec<_>>() {
            let state = GameStateSnapshot::from_game_state(&self.state, &player_id);
            if let Err(e) = self.send_to(&ServerMessage::Snapshot(state), &player_id).await {
                eprintln!("Failed to send to player id {player_id}: {e}");
                failed.push(player_id);
            };
        };

        for id in failed {
            self.mark_disconnected(id);
        }
    }

    fn mark_disconnected(&mut self, player_id: PlayerId) {
        if let Some(player) = self.state.players.get_mut(&player_id) {
            player.status = PlayerStatus::Disconnected;
        };

        self.state.turn_order.retain(|id| *id != player_id)
    }

    fn reset_state(&mut self) {
        self.state.deck = Deck::new().shuffled();
        self.state.discarded = vec![];
        self.state.current_turn = self.state.turn_order[0];

        self.state.status = GameStatus::WaitingPlayers;
    }

    pub async fn new_game(&mut self) {
        // reset deck and stuff
        self.reset_state();

        // deal out the hands
        for player in self.state.players.values_mut() {
            player.hand.append(&mut self.state.deck.deal(2));
        }

        self.broadcast_state().await;
    }
}